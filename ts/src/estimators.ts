/**
 * Pragmastat estimators implementation
 */

import { fastCenter } from './fastCenter';
import { fastSpread } from './fastSpread';
import { fastShift, fastRatio } from './fastShift';
import { pairwiseMargin } from './pairwiseMargin';
import { signedRankMargin } from './signedRankMargin';
import { signMarginRandomized } from './signMargin';
import { fastCenterQuantileBounds } from './fastCenterQuantiles';
import { minAchievableMisrateOneSample, minAchievableMisrateTwoSample } from './minMisrate';
import { AssumptionError } from './assumptions';
import { MeasurementUnit } from './measurement-unit';
import { Measurement } from './measurement';
import { Sample, checkNonWeighted, checkCompatibleUnits, convertToFiner } from './sample';
import { Rng } from './rng';

/**
 * Represents an interval with lower and upper bounds and a unit.
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

/**
 * Calculate the Center - median of all pairwise averages (x[i] + x[j])/2
 * Uses fast O(n log n) algorithm.
 * @param x Sample
 * @returns The center measurement
 */
export function center(x: Sample): Measurement {
  checkNonWeighted(x);
  const vals = [...x.values];
  // fastCenter performs its own internal work on raw arrays
  const result = fastCenter(vals);
  return new Measurement(result, x.unit);
}

/**
 * Calculate the Spread - median of all pairwise absolute differences |x[i] - x[j]|
 * Uses fast O(n log n) algorithm.
 *
 * @param x Sample
 * @returns The spread measurement
 * @throws AssumptionError if sample is tie-dominant
 */
export function spread(x: Sample): Measurement {
  checkNonWeighted(x);
  const vals = [...x.values];
  const spreadVal = fastSpread(vals);
  if (spreadVal <= 0) {
    throw AssumptionError.sparity('x');
  }
  return new Measurement(spreadVal, x.unit);
}

/**
 * Calculate the RelSpread - ratio of Spread to absolute Center
 *
 * @deprecated Use `spread(x).value / Math.abs(center(x).value)` instead.
 *
 * @param x Sample
 * @returns The relative spread measurement
 * @throws AssumptionError if sample contains non-positive values
 */
export function relSpread(x: Sample): Measurement {
  checkNonWeighted(x);
  const vals = [...x.values];
  // Check positivity
  for (const v of vals) {
    if (v <= 0) {
      throw AssumptionError.positivity('x');
    }
  }
  const c = fastCenter(vals);
  const s = fastSpread(vals);
  return new Measurement(s / Math.abs(c), MeasurementUnit.NUMBER);
}

/**
 * Calculate the Shift - median of all pairwise differences (x[i] - y[j])
 * Uses fast O((m + n) * log(precision)) algorithm.
 * @param x First sample
 * @param y Second sample
 * @returns The shift measurement
 */
export function shift(x: Sample, y: Sample): Measurement {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);
  const resultUnit = cx.unit;

  const xv = [...cx.values];
  const yv = [...cy.values];

  const result = fastShift(xv, yv, [0.5], false)[0];
  return new Measurement(result, resultUnit);
}

/**
 * Calculate the Ratio - median of all pairwise ratios (x[i] / y[j]) via log-transformation
 * Equivalent to: exp(Shift(log(x), log(y)))
 *
 * @param x First sample
 * @param y Second sample
 * @returns The ratio measurement
 * @throws AssumptionError if either sample contains non-positive values
 */
export function ratio(x: Sample, y: Sample): Measurement {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);

  const xv = [...cx.values];
  const yv = [...cy.values];
  // Check positivity (not covered by Sample construction)
  if (xv.some((v) => v <= 0)) {
    throw AssumptionError.positivity('x');
  }
  if (yv.some((v) => v <= 0)) {
    throw AssumptionError.positivity('y');
  }

  const result = fastRatio(xv, yv, [0.5], false)[0];
  return new Measurement(result, MeasurementUnit.RATIO);
}

/**
 * Calculate the AvgSpread - weighted average of spreads: (n*Spread(x) + m*Spread(y))/(n+m)
 *
 * @internal
 * @param x First sample
 * @param y Second sample
 * @returns The combined spread measurement
 */
function avgSpread(x: Sample, y: Sample): Measurement {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);
  const resultUnit = cx.unit;

  const xv = [...cx.values];
  const yv = [...cy.values];

  const nx = xv.length;
  const ny = yv.length;

  const spreadX = fastSpread(xv);
  if (spreadX <= 0) {
    throw AssumptionError.sparity('x');
  }
  const spreadY = fastSpread(yv);
  if (spreadY <= 0) {
    throw AssumptionError.sparity('y');
  }

  const result = (nx * spreadX + ny * spreadY) / (nx + ny);
  return new Measurement(result, resultUnit);
}

/**
 * Calculate the Disparity - Shift / AvgSpread
 *
 * @param x First sample
 * @param y Second sample
 * @returns The disparity measurement
 */
export function disparity(x: Sample, y: Sample): Measurement {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);

  const xv = [...cx.values];
  const yv = [...cy.values];

  const nx = xv.length;
  const ny = yv.length;

  const spreadX = fastSpread(xv);
  if (spreadX <= 0) {
    throw AssumptionError.sparity('x');
  }
  const spreadY = fastSpread(yv);
  if (spreadY <= 0) {
    throw AssumptionError.sparity('y');
  }

  const shiftVal = fastShift(xv, yv, [0.5], false)[0];
  const avgSpreadVal = (nx * spreadX + ny * spreadY) / (nx + ny);

  return new Measurement(shiftVal / avgSpreadVal, MeasurementUnit.DISPARITY);
}

/**
 * Provides bounds on the Shift estimator with specified misclassification rate (ShiftBounds)
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate
 * @returns Bounds with lower, upper, and unit
 */
export function shiftBounds(x: Sample, y: Sample, misrate: number = DEFAULT_MISRATE): Bounds {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);
  const resultUnit = cx.unit;

  const xv = [...cx.values];
  const yv = [...cy.values];

  const n = xv.length;
  const m = yv.length;

  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateTwoSample(n, m);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  const xs = [...xv].sort((a, b) => a - b);
  const ys = [...yv].sort((a, b) => a - b);

  const total = BigInt(n) * BigInt(m);

  if (total === 1n) {
    const value = xs[0] - ys[0];
    return new Bounds(value, value, resultUnit);
  }

  const margin = BigInt(pairwiseMargin(n, m, misrate));
  const maxHalfMargin = (total - 1n) / 2n;
  let halfMargin = margin / 2n;
  if (halfMargin > maxHalfMargin) {
    halfMargin = maxHalfMargin;
  }
  const kLeft = halfMargin;
  const kRight = total - 1n - halfMargin;

  const denominator = Number(total - 1n) || 1;
  const pLeft = Number(kLeft) / denominator;
  const pRight = Number(kRight) / denominator;

  const [left, right] = fastShift(xs, ys, [pLeft, pRight], true);
  const lower = Math.min(left, right);
  const upper = Math.max(left, right);

  return new Bounds(lower, upper, resultUnit);
}

/**
 * Provides bounds on the Ratio estimator with specified misclassification rate (RatioBounds)
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate
 * @returns Bounds with lower, upper, and unit
 */
export function ratioBounds(x: Sample, y: Sample, misrate: number = DEFAULT_MISRATE): Bounds {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);

  const xv = [...cx.values];
  const yv = [...cy.values];
  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateTwoSample(xv.length, yv.length);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  // Check positivity and log-transform
  const logX = new Array<number>(xv.length);
  for (let i = 0; i < xv.length; i++) {
    if (xv[i] <= 0) throw AssumptionError.positivity('x');
    logX[i] = Math.log(xv[i]);
  }
  const logY = new Array<number>(yv.length);
  for (let i = 0; i < yv.length; i++) {
    if (yv[i] <= 0) throw AssumptionError.positivity('y');
    logY[i] = Math.log(yv[i]);
  }

  // Delegate to internal shiftBounds logic on log-space arrays
  const logBounds = rawShiftBounds(logX, logY, misrate);

  return new Bounds(Math.exp(logBounds.lower), Math.exp(logBounds.upper), MeasurementUnit.RATIO);
}

/**
 * Provides exact bounds on the Center (Hodges-Lehmann pseudomedian)
 *
 * @param x Sample
 * @param misrate Misclassification rate
 * @returns Bounds with lower, upper, and unit
 */
export function centerBounds(x: Sample, misrate: number = DEFAULT_MISRATE): Bounds {
  checkNonWeighted(x);

  const xv = [...x.values];

  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const n = xv.length;

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

  const sorted = [...xv].sort((a, b) => a - b);

  const [lo, hi] = fastCenterQuantileBounds(sorted, kLeft, kRight);
  return new Bounds(lo, hi, x.unit);
}

/**
 * Provides distribution-free bounds for the Spread estimator
 *
 * @param x Sample
 * @param misrate Misclassification rate
 * @param seed Optional string seed
 * @returns Bounds with lower, upper, and unit
 */
export function spreadBounds(x: Sample, misrate: number = DEFAULT_MISRATE, seed?: string): Bounds {
  checkNonWeighted(x);

  const xv = [...x.values];

  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');
  const n = xv.length;
  const m = Math.floor(n / 2);
  const minMisrate = minAchievableMisrateOneSample(m);
  if (misrate < minMisrate) throw AssumptionError.domain('misrate');
  if (n < 2) {
    throw AssumptionError.sparity('x');
  }
  if (fastSpread(xv) <= 0) {
    throw AssumptionError.sparity('x');
  }

  const rng = seed !== undefined ? new Rng(seed) : new Rng();
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
    diffs.push(Math.abs(xv[a] - xv[b]));
  }
  diffs.sort((a, b) => a - b);

  return new Bounds(diffs[kLeft - 1], diffs[kRight - 1], x.unit);
}

/**
 * Internal raw shiftBounds that operates on number arrays (for ratioBounds delegation).
 */
function rawShiftBounds(
  xv: number[],
  yv: number[],
  misrate: number,
): { lower: number; upper: number } {
  const n = xv.length;
  const m = yv.length;

  const xs = [...xv].sort((a, b) => a - b);
  const ys = [...yv].sort((a, b) => a - b);

  const total = BigInt(n) * BigInt(m);

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

  const denominator = Number(total - 1n) || 1;
  const pLeft = Number(kLeft) / denominator;
  const pRight = Number(kRight) / denominator;

  const [left, right] = fastShift(xs, ys, [pLeft, pRight], true);
  const lower = Math.min(left, right);
  const upper = Math.max(left, right);

  return { lower, upper };
}

/**
 * Internal AvgSpreadBounds using Bonferroni combination.
 *
 * @internal
 */
function avgSpreadBounds(
  x: Sample,
  y: Sample,
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
): Bounds {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);
  const resultUnit = cx.unit;

  const xv = [...cx.values];
  const yv = [...cy.values];

  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');

  const n = xv.length;
  const m = yv.length;
  if (n < 2) throw AssumptionError.domain('x');
  if (m < 2) throw AssumptionError.domain('y');

  const alpha = misrate / 2;
  const minX = minAchievableMisrateOneSample(Math.floor(n / 2));
  const minY = minAchievableMisrateOneSample(Math.floor(m / 2));
  if (alpha < minX || alpha < minY) throw AssumptionError.domain('misrate');

  if (fastSpread(xv) <= 0) {
    throw AssumptionError.sparity('x');
  }
  if (fastSpread(yv) <= 0) {
    throw AssumptionError.sparity('y');
  }

  const boundsX = spreadBounds(cx, alpha, seed);
  const boundsY = spreadBounds(cy, alpha, seed);

  const weightX = n / (n + m);
  const weightY = m / (n + m);

  return new Bounds(
    weightX * boundsX.lower + weightY * boundsY.lower,
    weightX * boundsX.upper + weightY * boundsY.upper,
    resultUnit,
  );
}

/**
 * Provides distribution-free bounds for the Disparity estimator
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate
 * @param seed Optional string seed
 * @returns Bounds with lower, upper, and unit
 */
export function disparityBounds(
  x: Sample,
  y: Sample,
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
): Bounds {
  checkNonWeighted(x);
  checkNonWeighted(y);
  checkCompatibleUnits(x, y);
  const [cx, cy] = convertToFiner(x, y);

  const xv = [...cx.values];
  const yv = [...cy.values];

  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');

  const n = xv.length;
  const m = yv.length;
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

  if (fastSpread(xv) <= 0) {
    throw AssumptionError.sparity('x');
  }
  if (fastSpread(yv) <= 0) {
    throw AssumptionError.sparity('y');
  }

  const sb = shiftBounds(cx, cy, alphaShift);
  const ab = avgSpreadBounds(cx, cy, alphaAvg, seed);

  const la = ab.lower;
  const ua = ab.upper;
  const ls = sb.lower;
  const us = sb.upper;

  const D = MeasurementUnit.DISPARITY;

  if (la > 0.0) {
    const r1 = ls / la;
    const r2 = ls / ua;
    const r3 = us / la;
    const r4 = us / ua;
    return new Bounds(Math.min(r1, r2, r3, r4), Math.max(r1, r2, r3, r4), D);
  }

  if (ua <= 0.0) {
    if (ls === 0.0 && us === 0.0) return new Bounds(0.0, 0.0, D);
    if (ls >= 0.0) return new Bounds(0.0, Infinity, D);
    if (us <= 0.0) return new Bounds(-Infinity, 0.0, D);
    return new Bounds(-Infinity, Infinity, D);
  }

  // Default: ua > 0 && la <= 0
  if (ls > 0.0) return new Bounds(ls / ua, Infinity, D);
  if (us < 0.0) return new Bounds(-Infinity, us / ua, D);
  if (ls === 0.0 && us === 0.0) return new Bounds(0.0, 0.0, D);
  if (ls === 0.0 && us > 0.0) return new Bounds(0.0, Infinity, D);
  if (ls < 0.0 && us === 0.0) return new Bounds(-Infinity, 0.0, D);

  return new Bounds(-Infinity, Infinity, D);
}

// Internal-only exports for testing (not part of public API)
export { avgSpread as _avgSpread, avgSpreadBounds as _avgSpreadBounds };
