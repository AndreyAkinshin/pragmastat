/**
 * Pragmastat estimators implementation
 */

import { fastCenter } from './fastCenter';
import { fastSpread } from './fastSpread';
import { fastShift, fastRatio } from './fastShift';
import { pairwiseMargin } from './pairwiseMargin';
import { signedRankMargin } from './signedRankMargin';
import { fastCenterQuantileBounds } from './fastCenterQuantiles';
import { minAchievableMisrateOneSample, minAchievableMisrateTwoSample } from './minMisrate';
import { checkValidity, checkPositivity, checkSparity, log, AssumptionError } from './assumptions';

/**
 * Calculate the median of an array of numbers
 * @param values Array of numbers
 * @returns The median value
 */
// Not re-exported from index.ts: internal utility kept for cross-language consistency
export function median(values: number[]): number {
  // Check validity (priority 0)
  checkValidity(values, 'x');

  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);

  if (sorted.length % 2 === 0) {
    return (sorted[mid - 1] + sorted[mid]) / 2;
  } else {
    return sorted[mid];
  }
}

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
  // Check sparity (priority 2)
  checkSparity(x, 'x');
  return fastSpread(x);
}

/**
 * Calculate the RelSpread - ratio of Spread to absolute Center
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
  // Check sparity for x (priority 2, subject x)
  checkSparity(x, 'x');
  // Check sparity for y (priority 2, subject y)
  checkSparity(y, 'y');

  const nx = x.length;
  const ny = y.length;

  // Calculate spreads (using internal implementation since we already validated)
  const spreadX = fastSpread(x);
  const spreadY = fastSpread(y);

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
  // Check sparity for x (priority 2, subject x)
  checkSparity(x, 'x');
  // Check sparity for y (priority 2, subject y)
  checkSparity(y, 'y');

  const nx = x.length;
  const ny = y.length;

  // Calculate shift (we know inputs are valid)
  const shiftVal = fastShift(x, y, [0.5], false)[0];
  // Calculate avg_spread (using internal implementation since we already validated)
  const spreadX = fastSpread(x);
  const spreadY = fastSpread(y);
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
