/**
 * Pragmastat estimators implementation.
 *
 * Each estimator has exactly ONE implementation (a `*Core` wrapper function
 * that operates on native `number[]`, delegating to the imported `*Impl`
 * kernel). Two thin entry points wrap it:
 *   - the RAW native-array API: `fn(values: number[], assumeSorted?)` returns
 *     a unitless result (a plain `number`, or a plain `Bounds` carrying
 *     `MeasurementUnit.NUMBER`);
 *   - the Sample API: `fn(sample: Sample)` returns a `Measurement`/`Bounds`
 *     carrying the propagated unit.
 *
 * The public functions are union-typed (`number[] | Sample`) and dispatch to
 * the shared impl, so there is no duplicated estimator logic.
 *
 * ## assumeSorted contract (default `false`)
 *
 * For order-INDEPENDENT estimators (center, spread, shift, ratio, disparity,
 * centerBounds, shiftBounds, ratioBounds), `assumeSorted = true` means "the
 * input is already sorted ascending — skip the internal sort". This changes
 * the computation path. Passing `true` on UNSORTED input is a contract
 * violation (undefined behavior): the result is unspecified. Termination is
 * still guaranteed: the selection loops are bounded and fail with a
 * deterministic convergence error on pathological input.
 *
 * For SHUFFLE-based bounds the disjoint-pair shuffle ALWAYS runs on the passed
 * array's order, so the flag never affects the shuffle. But `assumeSorted` is
 * NOT a free no-op for these either: the sparity (spread > 0) check runs
 * `spreadImpl(x, assumeSorted)`, and on UNSORTED input with `assumeSorted = true`
 * that feeds unsorted data to a sorted-only kernel. On genuinely SORTED input
 * the flag is inert; on UNSORTED input passing `assumeSorted = true` is
 * undefined behavior — exactly like every other estimator — and may hit the
 * iteration cap and ERROR (or pass only by luck). Only the shuffle part is inert.
 *
 * `disparityBounds` likewise is inert on SORTED input only: alongside the shuffle
 * it makes an embedded order-independent shiftBounds sub-call that consumes the
 * passed slice as a "sorted view". On genuinely sorted input the flag is inert;
 * on UNSORTED input `assumeSorted = true` is undefined behavior and CAN change
 * the result (the sub-call treats the unsorted slice as sorted).
 *
 * The Sample entry always passes the cached `sortedValues` (so `assumeSorted`
 * is effectively `true` for it); the shuffle-based Sample path additionally
 * passes the cached sorted view for the sparity check while shuffling the
 * original `values`.
 */

import { centerImpl } from './centerImpl';
import { spreadImpl } from './spreadImpl';
import { shiftImpl } from './shiftImpl';
import { pairwiseMargin } from './pairwiseMargin';
import { signedRankMargin } from './signedRankMargin';
import { signMarginRandomized } from './signMargin';
import { centerQuantileBoundsImpl } from './centerQuantilesImpl';
import { minAchievableMisrateOneSample, minAchievableMisrateTwoSample } from './minMisrate';
import { AssumptionError, checkValidity, checkPositivity, log } from './assumptions';
import { MeasurementUnit } from './measurement-unit';
import { Measurement } from './measurement';
import { Sample, checkNonWeighted, checkCompatibleUnits, convertToFiner } from './sample';
import { Rng } from './rng';

/**
 * Represents an interval with lower and upper bounds and a unit.
 *
 * The RAW API returns `Bounds` carrying `MeasurementUnit.NUMBER` (unitless);
 * the Sample API returns `Bounds` carrying the propagated unit.
 */
export class Bounds {
  constructor(
    readonly lower: number,
    readonly upper: number,
    readonly unit: MeasurementUnit,
  ) {}

  contains(value: number): boolean {
    return this.lower <= value && value <= this.upper;
  }
}

export const DEFAULT_MISRATE = 1e-3;

/** Plain unitless bounds returned by the raw impl helpers. */
interface RawBounds {
  lower: number;
  upper: number;
}

/** Returns a sorted copy of `x`, or `x` itself when `assumeSorted`. */
function sortedOne(x: readonly number[], assumeSorted: boolean): readonly number[] {
  if (assumeSorted) return x;
  return [...x].sort((a, b) => a - b);
}

/**
 * Maps the public `assumeSorted` flag to the optional pre-sorted view used by
 * the shuffle-based bounds: when the caller's array is already sorted it
 * doubles as the sorted view for the order-independent sparity check (skipping
 * a re-sort). The disjoint-pair shuffle always runs on the caller's array
 * regardless, so the flag never affects the shuffle. On SORTED input the flag
 * is inert; on UNSORTED input passing `true` is undefined behavior (the sparity
 * check then runs the sorted-only spread kernel on unsorted data).
 */
function sortedView(x: readonly number[], assumeSorted: boolean): readonly number[] | null {
  return assumeSorted ? x : null;
}

/**
 * Computes the spread value for the sparity check. The result is
 * order-independent, so a pre-sorted view (when available) is used to skip
 * re-sorting; otherwise the original array is sorted internally.
 */
function spreadForSparity(orig: readonly number[], sorted: readonly number[] | null): number {
  return sorted !== null ? spreadImpl(sorted, true) : spreadImpl(orig, false);
}

// =============================================================================
// Raw core wrappers — the single implementation of each estimator
// (each delegates to the imported `*Impl` kernel)
// =============================================================================

function centerCore(x: readonly number[], assumeSorted: boolean): number {
  checkValidity(x, 'x');
  return centerImpl(x, assumeSorted);
}

function spreadCore(x: readonly number[], assumeSorted: boolean): number {
  checkValidity(x, 'x');
  const spreadVal = spreadImpl(x, assumeSorted);
  if (spreadVal <= 0) {
    throw AssumptionError.sparity('x');
  }
  return spreadVal;
}

function shiftCore(x: readonly number[], y: readonly number[], assumeSorted: boolean): number {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  return shiftImpl(x, y, [0.5], assumeSorted)[0];
}

function ratioCore(x: readonly number[], y: readonly number[], assumeSorted: boolean): number {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  checkPositivity(x, 'x');
  checkPositivity(y, 'y');
  // ratio is exp(shift(log x, log y)); log is monotonic, so the assumeSorted
  // fast path carries through the log-transform.
  const logX = log(x, 'x');
  const logY = log(y, 'y');
  return Math.exp(shiftImpl(logX, logY, [0.5], assumeSorted)[0]);
}

function disparityCore(x: readonly number[], y: readonly number[], assumeSorted: boolean): number {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  const nx = x.length;
  const ny = y.length;
  const spreadX = spreadImpl(x, assumeSorted);
  if (spreadX <= 0) {
    throw AssumptionError.sparity('x');
  }
  const spreadY = spreadImpl(y, assumeSorted);
  if (spreadY <= 0) {
    throw AssumptionError.sparity('y');
  }
  const shiftVal = shiftImpl(x, y, [0.5], assumeSorted)[0];
  const avgSpreadVal = (nx * spreadX + ny * spreadY) / (nx + ny);
  return shiftVal / avgSpreadVal;
}

function shiftBoundsImpl(
  x: readonly number[],
  y: readonly number[],
  misrate: number,
  assumeSorted: boolean,
): RawBounds {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const n = x.length;
  const m = y.length;

  const minMisrate = minAchievableMisrateTwoSample(n, m);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  const total = BigInt(n) * BigInt(m);

  const xs = sortedOne(x, assumeSorted);
  const ys = sortedOne(y, assumeSorted);

  if (total === 1n) {
    const value = xs[0] - ys[0];
    return { lower: value, upper: value };
  }

  const margin = BigInt(pairwiseMargin(n, m, misrate));
  const maxHalfMargin = (total - 1n) / 2n;
  let halfMargin = margin / 2n;
  if (halfMargin > maxHalfMargin) {
    halfMargin = maxHalfMargin;
  }
  const kLeft = halfMargin;
  const kRight = total - 1n - halfMargin;

  // total >= 2 here (the total === 1 case returned above), so total - 1 >= 1.
  const denominator = Number(total - 1n);
  const pLeft = Number(kLeft) / denominator;
  const pRight = Number(kRight) / denominator;

  const [left, right] = shiftImpl(xs, ys, [pLeft, pRight], true);
  return { lower: Math.min(left, right), upper: Math.max(left, right) };
}

function ratioBoundsImpl(
  x: readonly number[],
  y: readonly number[],
  misrate: number,
  assumeSorted: boolean,
): RawBounds {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateTwoSample(x.length, y.length);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  // log is monotonic: sorted positive input -> sorted log output, so the
  // assumeSorted flag carries through to the shift-bounds sub-computation.
  const logX = log(x, 'x');
  const logY = log(y, 'y');
  const logBounds = shiftBoundsImpl(logX, logY, misrate, assumeSorted);
  return { lower: Math.exp(logBounds.lower), upper: Math.exp(logBounds.upper) };
}

function centerBoundsImpl(x: readonly number[], misrate: number, assumeSorted: boolean): RawBounds {
  checkValidity(x, 'x');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const n = x.length;
  if (n < 2) {
    throw AssumptionError.domain('x');
  }

  const minMisrate = minAchievableMisrateOneSample(n);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  const totalPairs = (BigInt(n) * BigInt(n + 1)) / 2n;
  const margin = BigInt(signedRankMargin(n, misrate));
  const maxHalfMargin = (totalPairs - 1n) / 2n;
  let halfMargin = margin / 2n;
  if (halfMargin > maxHalfMargin) {
    halfMargin = maxHalfMargin;
  }
  const kLeft = Number(halfMargin + 1n);
  const kRight = Number(totalPairs - halfMargin);

  const sorted = sortedOne(x, assumeSorted);
  const [lo, hi] = centerQuantileBoundsImpl(sorted, kLeft, kRight);
  return { lower: lo, upper: hi };
}

/**
 * Shuffles the ORIGINAL order into disjoint pairs and returns order-statistic
 * bounds. The caller is responsible for validity/domain/sparity checks, so it
 * can be reused (e.g. by avgSpreadBounds) without re-running spreadImpl.
 *
 * `x` is always in original order (the shuffle is order-dependent).
 */
function spreadBoundsShuffle(x: readonly number[], misrate: number, rng: Rng): RawBounds {
  const n = x.length;
  const m = Math.floor(n / 2);

  const margin = signMarginRandomized(m, misrate, rng);
  let halfMargin = Math.floor(margin / 2);
  const maxHalfMargin = Math.floor((m - 1) / 2);
  if (halfMargin > maxHalfMargin) halfMargin = maxHalfMargin;
  const kLeft = halfMargin + 1;
  const kRight = m - halfMargin;

  const indices = Array.from({ length: n }, (_, i) => i);
  const shuffled = rng.shuffle(indices);
  const diffs: number[] = [];
  for (let i = 0; i < m; i++) {
    const a = shuffled[2 * i];
    const b = shuffled[2 * i + 1];
    diffs.push(Math.abs(x[a] - x[b]));
  }
  diffs.sort((a, b) => a - b);

  return { lower: diffs[kLeft - 1], upper: diffs[kRight - 1] };
}

/**
 * The single spreadBounds implementation.
 *
 * `x` is always in ORIGINAL order (the disjoint-pair shuffle is
 * order-dependent). `sortedX`, when provided, is a pre-sorted view used only to
 * speed up the order-independent sparity check; it NEVER feeds the shuffle.
 */
function spreadBoundsImpl(
  x: readonly number[],
  sortedX: readonly number[] | null,
  misrate: number,
  rng: Rng,
): RawBounds {
  checkValidity(x, 'x');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');
  const n = x.length;
  if (n < 2) {
    throw AssumptionError.sparity('x');
  }
  const m = Math.floor(n / 2);
  const minMisrate = minAchievableMisrateOneSample(m);
  if (misrate < minMisrate) throw AssumptionError.domain('misrate');
  if (spreadForSparity(x, sortedX) <= 0) {
    throw AssumptionError.sparity('x');
  }
  return spreadBoundsShuffle(x, misrate, rng);
}

/**
 * The single avgSpreadBounds implementation (internal helper).
 *
 * `x`/`y` are always in ORIGINAL order; `sortedX`/`sortedY`, when provided, are
 * pre-sorted views used only for the order-independent sparity check.
 */
function avgSpreadBoundsImpl(
  x: readonly number[],
  sortedX: readonly number[] | null,
  y: readonly number[],
  sortedY: readonly number[] | null,
  misrate: number,
  rngX: Rng,
  rngY: Rng,
): RawBounds {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');

  const n = x.length;
  const m = y.length;
  if (n < 2) throw AssumptionError.domain('x');
  if (m < 2) throw AssumptionError.domain('y');

  const alpha = misrate / 2;
  const minX = minAchievableMisrateOneSample(Math.floor(n / 2));
  const minY = minAchievableMisrateOneSample(Math.floor(m / 2));
  if (alpha < minX || alpha < minY) throw AssumptionError.domain('misrate');

  if (spreadForSparity(x, sortedX) <= 0) {
    throw AssumptionError.sparity('x');
  }
  if (spreadForSparity(y, sortedY) <= 0) {
    throw AssumptionError.sparity('y');
  }

  // The shuffle operates on the ORIGINAL order; sorted views are sparity-only.
  const boundsX = spreadBoundsShuffle(x, alpha, rngX);
  const boundsY = spreadBoundsShuffle(y, alpha, rngY);

  const weightX = n / (n + m);
  const weightY = m / (n + m);
  return {
    lower: weightX * boundsX.lower + weightY * boundsY.lower,
    upper: weightX * boundsX.upper + weightY * boundsY.upper,
  };
}

/** Computes disparity bounds from shift bounds (ls, us) and avg-spread bounds (la, ua). */
function disparityBoundsFromComponents(ls: number, us: number, la: number, ua: number): RawBounds {
  if (la > 0.0) {
    const r1 = ls / la;
    const r2 = ls / ua;
    const r3 = us / la;
    const r4 = us / ua;
    return { lower: Math.min(r1, r2, r3, r4), upper: Math.max(r1, r2, r3, r4) };
  }

  if (ua <= 0.0) {
    if (ls === 0.0 && us === 0.0) return { lower: 0.0, upper: 0.0 };
    if (ls >= 0.0) return { lower: 0.0, upper: Infinity };
    if (us <= 0.0) return { lower: -Infinity, upper: 0.0 };
    return { lower: -Infinity, upper: Infinity };
  }

  // Default: ua > 0 && la <= 0
  if (ls > 0.0) return { lower: ls / ua, upper: Infinity };
  if (us < 0.0) return { lower: -Infinity, upper: us / ua };
  if (ls === 0.0 && us === 0.0) return { lower: 0.0, upper: 0.0 };
  if (ls === 0.0 && us > 0.0) return { lower: 0.0, upper: Infinity };
  if (ls < 0.0 && us === 0.0) return { lower: -Infinity, upper: 0.0 };

  return { lower: -Infinity, upper: Infinity };
}

/**
 * The single disparityBounds implementation.
 *
 * `x`/`y` are always in ORIGINAL order; `sortedX`/`sortedY`, when present, are
 * pre-sorted views used only for the order-independent sparity and shift-bounds
 * sub-computations.
 */
function disparityBoundsImpl(
  x: readonly number[],
  sortedX: readonly number[] | null,
  y: readonly number[],
  sortedY: readonly number[] | null,
  misrate: number,
  rngX: Rng,
  rngY: Rng,
): RawBounds {
  checkValidity(x, 'x');
  checkValidity(y, 'y');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');

  const n = x.length;
  const m = y.length;
  if (n < 2) throw AssumptionError.domain('x');
  if (m < 2) throw AssumptionError.domain('y');

  const minShift = minAchievableMisrateTwoSample(n, m);
  const minX = minAchievableMisrateOneSample(Math.floor(n / 2));
  const minY = minAchievableMisrateOneSample(Math.floor(m / 2));
  const minAvg = 2.0 * Math.max(minX, minY);

  if (misrate < minShift + minAvg) throw AssumptionError.domain('misrate');

  const extra = misrate - (minShift + minAvg);
  const alphaShift = minShift + extra / 2.0;
  const alphaAvg = minAvg + extra / 2.0;

  // The spread > 0 sparity check is performed by avgSpreadBoundsImpl below
  // (identical predicate and 'x'/'y' order). shiftBounds runs first but cannot
  // throw for these inputs (alphaShift >= the two-sample minimum), so it cannot
  // mask that sparity error.
  // shiftBounds is order-independent given sorted input; use sorted views when present.
  const sb =
    sortedX !== null && sortedY !== null
      ? shiftBoundsImpl(sortedX, sortedY, alphaShift, true)
      : shiftBoundsImpl(x, y, alphaShift, false);
  const ab = avgSpreadBoundsImpl(x, sortedX, y, sortedY, alphaAvg, rngX, rngY);

  return disparityBoundsFromComponents(sb.lower, sb.upper, ab.lower, ab.upper);
}

// =============================================================================
// Public API — union-typed entry points (number[] = raw/unitless, Sample = unit)
// =============================================================================

/**
 * Calculate the Center - median of all pairwise averages (x[i] + x[j])/2.
 *
 * @param x A native `number[]` (raw, unitless) or a `Sample`.
 * @param assumeSorted Only honored for `number[]` input; if `true`, the array
 *   is assumed sorted ascending and the internal sort is skipped (see module
 *   docs for the undefined-behavior contract).
 * @returns A unitless `number` for `number[]` input, or a `Measurement` for a `Sample`.
 */
export function center(x: number[], assumeSorted?: boolean): number;
export function center(x: Sample): Measurement;
export function center(x: number[] | Sample, assumeSorted = false): number | Measurement {
  if (x instanceof Sample) {
    checkNonWeighted('x', x);
    return new Measurement(centerCore(x.sortedValues, true), x.unit);
  }
  return centerCore(x, assumeSorted);
}

/**
 * Calculate the Spread - median of all pairwise absolute differences |x[i] - x[j]|.
 *
 * @throws AssumptionError if the sample is tie-dominant (spread <= 0).
 */
export function spread(x: number[], assumeSorted?: boolean): number;
export function spread(x: Sample): Measurement;
export function spread(x: number[] | Sample, assumeSorted = false): number | Measurement {
  if (x instanceof Sample) {
    checkNonWeighted('x', x);
    return new Measurement(spreadCore(x.sortedValues, true), x.unit);
  }
  return spreadCore(x, assumeSorted);
}

/**
 * Calculate the Shift - median of all pairwise differences (x[i] - y[j]).
 */
export function shift(x: number[], y: number[], assumeSorted?: boolean): number;
export function shift(x: Sample, y: Sample): Measurement;
export function shift(
  x: number[] | Sample,
  y: number[] | Sample,
  assumeSorted = false,
): number | Measurement {
  if (x instanceof Sample || y instanceof Sample) {
    const sx = x as Sample;
    const sy = y as Sample;
    checkNonWeighted('x', sx);
    checkNonWeighted('y', sy);
    checkCompatibleUnits(sx, sy);
    const [cx, cy] = convertToFiner(sx, sy);
    return new Measurement(shiftCore(cx.sortedValues, cy.sortedValues, true), cx.unit);
  }
  return shiftCore(x, y, assumeSorted);
}

/**
 * Calculate the Ratio - median of all pairwise ratios (x[i] / y[j]).
 *
 * @throws AssumptionError if either sample contains non-positive values.
 */
export function ratio(x: number[], y: number[], assumeSorted?: boolean): number;
export function ratio(x: Sample, y: Sample): Measurement;
export function ratio(
  x: number[] | Sample,
  y: number[] | Sample,
  assumeSorted = false,
): number | Measurement {
  if (x instanceof Sample || y instanceof Sample) {
    const sx = x as Sample;
    const sy = y as Sample;
    checkNonWeighted('x', sx);
    checkNonWeighted('y', sy);
    checkCompatibleUnits(sx, sy);
    const [cx, cy] = convertToFiner(sx, sy);
    return new Measurement(
      ratioCore(cx.sortedValues, cy.sortedValues, true),
      MeasurementUnit.RATIO,
    );
  }
  return ratioCore(x, y, assumeSorted);
}

/**
 * Calculate the Disparity - Shift / AvgSpread.
 */
export function disparity(x: number[], y: number[], assumeSorted?: boolean): number;
export function disparity(x: Sample, y: Sample): Measurement;
export function disparity(
  x: number[] | Sample,
  y: number[] | Sample,
  assumeSorted = false,
): number | Measurement {
  if (x instanceof Sample || y instanceof Sample) {
    const sx = x as Sample;
    const sy = y as Sample;
    checkNonWeighted('x', sx);
    checkNonWeighted('y', sy);
    checkCompatibleUnits(sx, sy);
    const [cx, cy] = convertToFiner(sx, sy);
    return new Measurement(
      disparityCore(cx.sortedValues, cy.sortedValues, true),
      MeasurementUnit.DISPARITY,
    );
  }
  return disparityCore(x, y, assumeSorted);
}

/**
 * Provides bounds on the Shift estimator with specified misclassification rate.
 */
export function shiftBounds(
  x: number[],
  y: number[],
  misrate?: number,
  assumeSorted?: boolean,
): Bounds;
export function shiftBounds(x: Sample, y: Sample, misrate?: number): Bounds;
export function shiftBounds(
  x: number[] | Sample,
  y: number[] | Sample,
  misrate: number = DEFAULT_MISRATE,
  assumeSorted = false,
): Bounds {
  if (x instanceof Sample || y instanceof Sample) {
    const sx = x as Sample;
    const sy = y as Sample;
    checkNonWeighted('x', sx);
    checkNonWeighted('y', sy);
    checkCompatibleUnits(sx, sy);
    const [cx, cy] = convertToFiner(sx, sy);
    const rb = shiftBoundsImpl(cx.sortedValues, cy.sortedValues, misrate, true);
    return new Bounds(rb.lower, rb.upper, cx.unit);
  }
  const rb = shiftBoundsImpl(x, y, misrate, assumeSorted);
  return new Bounds(rb.lower, rb.upper, MeasurementUnit.NUMBER);
}

/**
 * Provides bounds on the Ratio estimator with specified misclassification rate.
 */
export function ratioBounds(
  x: number[],
  y: number[],
  misrate?: number,
  assumeSorted?: boolean,
): Bounds;
export function ratioBounds(x: Sample, y: Sample, misrate?: number): Bounds;
export function ratioBounds(
  x: number[] | Sample,
  y: number[] | Sample,
  misrate: number = DEFAULT_MISRATE,
  assumeSorted = false,
): Bounds {
  if (x instanceof Sample || y instanceof Sample) {
    const sx = x as Sample;
    const sy = y as Sample;
    checkNonWeighted('x', sx);
    checkNonWeighted('y', sy);
    checkCompatibleUnits(sx, sy);
    const [cx, cy] = convertToFiner(sx, sy);
    const rb = ratioBoundsImpl(cx.sortedValues, cy.sortedValues, misrate, true);
    return new Bounds(rb.lower, rb.upper, MeasurementUnit.RATIO);
  }
  const rb = ratioBoundsImpl(x, y, misrate, assumeSorted);
  return new Bounds(rb.lower, rb.upper, MeasurementUnit.NUMBER);
}

/**
 * Provides exact bounds on the Center (Hodges-Lehmann pseudomedian).
 */
export function centerBounds(x: number[], misrate?: number, assumeSorted?: boolean): Bounds;
export function centerBounds(x: Sample, misrate?: number): Bounds;
export function centerBounds(
  x: number[] | Sample,
  misrate: number = DEFAULT_MISRATE,
  assumeSorted = false,
): Bounds {
  if (x instanceof Sample) {
    checkNonWeighted('x', x);
    const rb = centerBoundsImpl(x.sortedValues, misrate, true);
    return new Bounds(rb.lower, rb.upper, x.unit);
  }
  const rb = centerBoundsImpl(x, misrate, assumeSorted);
  return new Bounds(rb.lower, rb.upper, MeasurementUnit.NUMBER);
}

/**
 * Provides distribution-free bounds for the Spread estimator.
 *
 * The disjoint-pair shuffle always runs on the passed order, so the flag never
 * affects the shuffle. The sparity check, however, runs `spreadImpl(x,
 * assumeSorted)`: on SORTED input `assumeSorted` is inert, but on UNSORTED input
 * `assumeSorted = true` is undefined behavior (same as all estimators) and may
 * ERROR by feeding unsorted data to the sorted-only spread kernel.
 */
export function spreadBounds(
  x: number[],
  misrate?: number,
  seed?: string,
  assumeSorted?: boolean,
): Bounds;
export function spreadBounds(x: Sample, misrate?: number, seed?: string): Bounds;
export function spreadBounds(
  x: number[] | Sample,
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
  assumeSorted = false,
): Bounds {
  const rng = seed !== undefined ? new Rng(seed) : new Rng();
  if (x instanceof Sample) {
    checkNonWeighted('x', x);
    // Shuffle runs on the original order; the cached sorted view is sparity-only.
    const rb = spreadBoundsImpl(x.values, x.sortedValues, misrate, rng);
    return new Bounds(rb.lower, rb.upper, x.unit);
  }
  const rb = spreadBoundsImpl(x, sortedView(x, assumeSorted), misrate, rng);
  return new Bounds(rb.lower, rb.upper, MeasurementUnit.NUMBER);
}

/**
 * Provides distribution-free bounds for the Disparity estimator.
 *
 * The disjoint-pair shuffle always runs on the passed order. Unlike spreadBounds,
 * this estimator is NOT inert under `assumeSorted`: it also makes an embedded
 * order-independent shiftBounds sub-call that consumes the passed slice as a
 * sorted view. On genuinely sorted input `assumeSorted` does not change the
 * result; on UNSORTED input `assumeSorted = true` is undefined behavior and CAN
 * change the result.
 */
export function disparityBounds(
  x: number[],
  y: number[],
  misrate?: number,
  seed?: string,
  assumeSorted?: boolean,
): Bounds;
export function disparityBounds(x: Sample, y: Sample, misrate?: number, seed?: string): Bounds;
export function disparityBounds(
  x: number[] | Sample,
  y: number[] | Sample,
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
  assumeSorted = false,
): Bounds {
  const rngX = seed !== undefined ? new Rng(seed) : new Rng();
  const rngY = seed !== undefined ? new Rng(seed) : new Rng();
  if (x instanceof Sample || y instanceof Sample) {
    const sx = x as Sample;
    const sy = y as Sample;
    checkNonWeighted('x', sx);
    checkNonWeighted('y', sy);
    checkCompatibleUnits(sx, sy);
    const [cx, cy] = convertToFiner(sx, sy);
    const rb = disparityBoundsImpl(
      cx.values,
      cx.sortedValues,
      cy.values,
      cy.sortedValues,
      misrate,
      rngX,
      rngY,
    );
    return new Bounds(rb.lower, rb.upper, MeasurementUnit.DISPARITY);
  }
  const rb = disparityBoundsImpl(
    x,
    sortedView(x, assumeSorted),
    y,
    sortedView(y, assumeSorted),
    misrate,
    rngX,
    rngY,
  );
  return new Bounds(rb.lower, rb.upper, MeasurementUnit.NUMBER);
}

// =============================================================================
// Internal helpers retained for tests / compare.ts (Sample-only, with units)
// =============================================================================

/**
 * Calculate the AvgSpread - weighted average of spreads.
 * @internal
 */
function avgSpread(x: Sample, y: Sample): Measurement {
  checkNonWeighted('x', x);
  checkNonWeighted('y', y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);
  const resultUnit = cx.unit;

  const nx = cx.values.length;
  const ny = cy.values.length;

  const spreadX = spreadImpl(cx.sortedValues, true);
  if (spreadX <= 0) {
    throw AssumptionError.sparity('x');
  }
  const spreadY = spreadImpl(cy.sortedValues, true);
  if (spreadY <= 0) {
    throw AssumptionError.sparity('y');
  }

  const result = (nx * spreadX + ny * spreadY) / (nx + ny);
  return new Measurement(result, resultUnit);
}

/**
 * Internal AvgSpreadBounds using Bonferroni combination.
 * @internal
 */
function avgSpreadBounds(
  x: Sample,
  y: Sample,
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
): Bounds {
  checkNonWeighted('x', x);
  checkNonWeighted('y', y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);
  const rngX = seed !== undefined ? new Rng(seed) : new Rng();
  const rngY = seed !== undefined ? new Rng(seed) : new Rng();
  const rb = avgSpreadBoundsImpl(
    cx.values,
    cx.sortedValues,
    cy.values,
    cy.sortedValues,
    misrate,
    rngX,
    rngY,
  );
  return new Bounds(rb.lower, rb.upper, cx.unit);
}

// Internal-only exports for testing (not part of public API)
export { avgSpread as _avgSpread, avgSpreadBounds as _avgSpreadBounds };
