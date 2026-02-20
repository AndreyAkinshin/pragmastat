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
import { checkValidity, checkPositivity, log, AssumptionError } from './assumptions';
import { Rng } from './rng';

/**
 * Calculate the Center - median of all pairwise averages (x[i] + x[j])/2
 * Uses fast O(n log n) algorithm.
 * @param x Array of sample values
 * @returns The center estimate
 */
export function center(x: number[]): number {
  // Check validity (priority 0)
  checkValidity(x, 'x');
  return fastCenter(x);
}

/**
 * Calculate the Spread - median of all pairwise absolute differences |x[i] - x[j]|
 * Uses fast O(n log n) algorithm.
 *
 * Assumptions:
 *   sparity(x) - sample must be non tie-dominant (Spread > 0)
 *
 * @param x Array of sample values
 * @returns The spread estimate
 * @throws AssumptionError if sample is empty, contains NaN/Inf, or is tie-dominant
 */
export function spread(x: number[]): number {
  // Check validity (priority 0)
  checkValidity(x, 'x');
  const spreadVal = fastSpread(x);
  if (spreadVal <= 0) {
    throw AssumptionError.sparity('x');
  }
  return spreadVal;
}

/**
 * Calculate the RelSpread - ratio of Spread to absolute Center
 *
 * @deprecated Use `spread(x) / Math.abs(center(x))` instead.
 *
 * Assumptions:
 *   positivity(x) - all values must be strictly positive (ensures Center > 0)
 *
 * @param x Array of sample values
 * @returns The relative spread estimate
 * @throws AssumptionError if sample is empty, contains NaN/Inf, or contains non-positive values
 */
export function relSpread(x: number[]): number {
  // Check validity (priority 0)
  checkValidity(x, 'x');
  // Check positivity (priority 1)
  checkPositivity(x, 'x');
  // Calculate center (we know x is valid, center should succeed)
  const c = fastCenter(x);
  // Calculate spread (using internal implementation since we already validated)
  const s = fastSpread(x);
  // center is guaranteed positive because all values are positive
  return s / Math.abs(c);
}

/**
 * Calculate the Shift - median of all pairwise differences (x[i] - y[j])
 * Uses fast O((m + n) * log(precision)) algorithm.
 * @param x First sample
 * @param y Second sample
 * @returns The shift estimate
 */
export function shift(x: number[], y: number[]): number {
  // Check validity (priority 0)
  checkValidity(x, 'x');
  checkValidity(y, 'y');

  return fastShift(x, y, [0.5], false)[0];
}

/**
 * Calculate the Ratio - median of all pairwise ratios (x[i] / y[j]) via log-transformation
 * Equivalent to: exp(Shift(log(x), log(y)))
 * Uses fast O((m + n) * log(precision)) algorithm.
 *
 * Assumptions:
 *   positivity(x) - all values in x must be strictly positive
 *   positivity(y) - all values in y must be strictly positive
 *
 * @param x First sample
 * @param y Second sample
 * @returns The ratio estimate
 * @throws AssumptionError if either sample is empty, contains NaN/Inf, or contains non-positive values
 */
export function ratio(x: number[], y: number[]): number {
  // Check validity for x (priority 0, subject x)
  checkValidity(x, 'x');
  // Check validity for y (priority 0, subject y)
  checkValidity(y, 'y');
  // Check positivity for x (priority 1, subject x)
  checkPositivity(x, 'x');
  // Check positivity for y (priority 1, subject y)
  checkPositivity(y, 'y');

  return fastRatio(x, y, [0.5], false)[0];
}

/**
 * Calculate the AvgSpread - weighted average of spreads: (n*Spread(x) + m*Spread(y))/(n+m)
 *
 * Assumptions:
 *   sparity(x) - first sample must be non tie-dominant (Spread > 0)
 *   sparity(y) - second sample must be non tie-dominant (Spread > 0)
 *
 * @internal
 * @param x First sample
 * @param y Second sample
 * @returns The combined spread estimate
 * @throws AssumptionError if either sample is empty, contains NaN/Inf, or is tie-dominant
 */
export function avgSpread(x: number[], y: number[]): number {
  // Check validity for x (priority 0, subject x)
  checkValidity(x, 'x');
  // Check validity for y (priority 0, subject y)
  checkValidity(y, 'y');

  const nx = x.length;
  const ny = y.length;

  const spreadX = fastSpread(x);
  if (spreadX <= 0) {
    throw AssumptionError.sparity('x');
  }
  const spreadY = fastSpread(y);
  if (spreadY <= 0) {
    throw AssumptionError.sparity('y');
  }

  return (nx * spreadX + ny * spreadY) / (nx + ny);
}

/**
 * Calculate the Disparity - Shift / AvgSpread
 *
 * Assumptions:
 *   sparity(x) - first sample must be non tie-dominant (Spread > 0)
 *   sparity(y) - second sample must be non tie-dominant (Spread > 0)
 *
 * @param x First sample
 * @param y Second sample
 * @returns The disparity estimate
 * @throws AssumptionError if either sample is empty, contains NaN/Inf, or is tie-dominant
 */
export function disparity(x: number[], y: number[]): number {
  // Check validity for x (priority 0, subject x)
  checkValidity(x, 'x');
  // Check validity for y (priority 0, subject y)
  checkValidity(y, 'y');

  const nx = x.length;
  const ny = y.length;

  const spreadX = fastSpread(x);
  if (spreadX <= 0) {
    throw AssumptionError.sparity('x');
  }
  const spreadY = fastSpread(y);
  if (spreadY <= 0) {
    throw AssumptionError.sparity('y');
  }

  // Calculate shift (we know inputs are valid)
  const shiftVal = fastShift(x, y, [0.5], false)[0];
  const avgSpreadVal = (nx * spreadX + ny * spreadY) / (nx + ny);

  return shiftVal / avgSpreadVal;
}

export const DEFAULT_MISRATE = 1e-3;

/**
 * Represents an interval with lower and upper bounds
 */
export interface Bounds {
  lower: number;
  upper: number;
}

/**
 * Provides bounds on the Shift estimator with specified misclassification rate (ShiftBounds)
 *
 * The misrate represents the probability that the true shift falls outside the computed bounds.
 * This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate (probability that true shift falls outside bounds)
 * @returns An object containing the lower and upper bounds
 * @throws AssumptionError if either sample is empty or contains NaN/Inf
 */
export function shiftBounds(x: number[], y: number[], misrate: number = DEFAULT_MISRATE): Bounds {
  // Check validity for x
  checkValidity(x, 'x');
  // Check validity for y
  checkValidity(y, 'y');

  const n = x.length;
  const m = y.length;

  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateTwoSample(n, m);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  // Sort both arrays
  const xs = [...x].sort((a, b) => a - b);
  const ys = [...y].sort((a, b) => a - b);

  const total = n * m;

  // Special case: when there's only one pairwise difference, bounds collapse to a single value
  if (total === 1) {
    const value = xs[0] - ys[0];
    return { lower: value, upper: value };
  }

  const margin = pairwiseMargin(n, m, misrate);
  const halfMargin = Math.min(Math.floor(margin / 2), Math.floor((total - 1) / 2));
  const kLeft = halfMargin;
  const kRight = total - 1 - halfMargin;

  // Compute quantile positions
  const denominator = total - 1 || 1;
  const pLeft = kLeft / denominator;
  const pRight = kRight / denominator;

  // Use fastShift to compute quantiles of pairwise differences
  const [left, right] = fastShift(xs, ys, [pLeft, pRight], true);
  const lower = Math.min(left, right);
  const upper = Math.max(left, right);

  return { lower, upper };
}

/**
 * Provides bounds on the Ratio estimator with specified misclassification rate (RatioBounds)
 *
 * Computes bounds via log-transformation and ShiftBounds delegation:
 * RatioBounds(x, y, misrate) = exp(ShiftBounds(log(x), log(y), misrate))
 *
 * Assumptions:
 *   positivity(x) - all values in x must be strictly positive
 *   positivity(y) - all values in y must be strictly positive
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate (probability that true ratio falls outside bounds)
 * @returns An object containing the lower and upper bounds
 * @throws AssumptionError if either sample is empty, contains NaN/Inf, or contains non-positive values
 */
export function ratioBounds(x: number[], y: number[], misrate: number = DEFAULT_MISRATE): Bounds {
  checkValidity(x, 'x');
  checkValidity(y, 'y');

  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateTwoSample(x.length, y.length);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  // Log-transform samples (includes positivity check)
  const logX = log(x, 'x');
  const logY = log(y, 'y');

  // Delegate to shiftBounds in log-space
  const logBounds = shiftBounds(logX, logY, misrate);

  // Exp-transform back to ratio-space
  return {
    lower: Math.exp(logBounds.lower),
    upper: Math.exp(logBounds.upper),
  };
}

/**
 * Provides exact bounds on the Center (Hodges-Lehmann pseudomedian) with specified misclassification rate
 *
 * Uses SignedRankMargin to determine which pairwise averages form the bounds.
 *
 * @param x Sample array
 * @param misrate Misclassification rate (probability that true center falls outside bounds)
 * @returns An object containing the lower and upper bounds
 * @throws AssumptionError if sample is empty or contains NaN/Inf
 */
export function centerBounds(x: number[], misrate: number = DEFAULT_MISRATE): Bounds {
  checkValidity(x, 'x');

  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const n = x.length;

  if (n < 2) {
    throw AssumptionError.domain('x');
  }

  // Validate misrate
  const minMisrate = minAchievableMisrateOneSample(n);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  // Total number of pairwise averages (including self-pairs)
  const totalPairs = (n * (n + 1)) / 2;

  // Get signed-rank margin
  const margin = signedRankMargin(n, misrate);
  const halfMargin = Math.min(Math.floor(margin / 2), Math.floor((totalPairs - 1) / 2));

  // k_left and k_right are 1-based ranks
  const kLeft = halfMargin + 1;
  const kRight = totalPairs - halfMargin;

  // Sort the input
  const sorted = [...x].sort((a, b) => a - b);

  const [lo, hi] = fastCenterQuantileBounds(sorted, kLeft, kRight);
  return { lower: lo, upper: hi };
}

/**
 * Provides distribution-free bounds for the Spread estimator using disjoint pairs
 * with sign-test inversion.
 *
 * @param x Sample array
 * @param misrate Misclassification rate (probability that true spread falls outside bounds)
 * @param seed Optional string seed for deterministic randomization
 * @returns An object containing the lower and upper bounds
 * @throws AssumptionError if sample is invalid, misrate is out of domain, or sample is tie-dominant
 */
export function spreadBounds(
  x: number[],
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
): Bounds {
  checkValidity(x, 'x');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');
  const n = x.length;
  const m = Math.floor(n / 2);
  const minMisrate = minAchievableMisrateOneSample(m);
  if (misrate < minMisrate) throw AssumptionError.domain('misrate');
  if (x.length < 2) {
    throw AssumptionError.sparity('x');
  }
  if (fastSpread(x) <= 0) {
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
    diffs.push(Math.abs(x[a] - x[b]));
  }
  diffs.sort((a, b) => a - b);

  return { lower: diffs[kLeft - 1], upper: diffs[kRight - 1] };
}

/**
 * Provides distribution-free bounds for AvgSpread using Bonferroni combination.
 *
 * @internal
 * @param x First sample array
 * @param y Second sample array
 * @param misrate Misclassification rate (probability that true avg_spread falls outside bounds)
 * @param seed Optional string seed for deterministic randomization
 * @returns An object containing the lower and upper bounds
 * @throws AssumptionError if input is invalid, misrate is out of domain, or sample is tie-dominant
 */
export function avgSpreadBounds(
  x: number[],
  y: number[],
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
): Bounds {
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

  if (fastSpread(x) <= 0) {
    throw AssumptionError.sparity('x');
  }
  if (fastSpread(y) <= 0) {
    throw AssumptionError.sparity('y');
  }

  const boundsX = spreadBounds(x, alpha, seed);
  const boundsY = spreadBounds(y, alpha, seed);

  const weightX = n / (n + m);
  const weightY = m / (n + m);

  return {
    lower: weightX * boundsX.lower + weightY * boundsY.lower,
    upper: weightX * boundsX.upper + weightY * boundsY.upper,
  };
}

/**
 * Provides distribution-free bounds for the Disparity estimator (Shift / AvgSpread)
 * using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate
 * @param seed Optional string seed for deterministic randomization
 * @returns An object containing the lower and upper bounds
 * @throws AssumptionError if inputs are invalid, misrate is out of domain, or samples are tie-dominant
 */
export function disparityBounds(
  x: number[],
  y: number[],
  misrate: number = DEFAULT_MISRATE,
  seed?: string,
): Bounds {
  // Check validity (priority 0)
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

  if (fastSpread(x) <= 0) {
    throw AssumptionError.sparity('x');
  }
  if (fastSpread(y) <= 0) {
    throw AssumptionError.sparity('y');
  }

  const sb = shiftBounds(x, y, alphaShift);
  const ab = avgSpreadBounds(x, y, alphaAvg, seed);

  const la = ab.lower;
  const ua = ab.upper;
  const ls = sb.lower;
  const us = sb.upper;

  if (la > 0.0) {
    const r1 = ls / la;
    const r2 = ls / ua;
    const r3 = us / la;
    const r4 = us / ua;
    const lower = Math.min(r1, r2, r3, r4);
    const upper = Math.max(r1, r2, r3, r4);
    return { lower, upper };
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
