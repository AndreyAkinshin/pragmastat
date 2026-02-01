/**
 * Pragmastat estimators implementation
 */

import { median } from './utils';
import { fastCenter } from './fastCenter';
import { fastSpread } from './fastSpread';
import { fastShift } from './fastShift';
import { pairwiseMargin } from './pairwiseMargin';
import { checkValidity, checkPositivity, checkSparity } from './assumptions';

/**
 * Calculate the Center - median of all pairwise averages (x[i] + x[j])/2
 * Uses fast O(n log n) algorithm.
 * @param x Array of sample values
 * @returns The center estimate
 */
export function center(x: number[]): number {
  // Check validity (priority 0)
  checkValidity(x, 'x', 'Center');
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
  checkValidity(x, 'x', 'Spread');
  // Check sparity (priority 2)
  checkSparity(x, 'x', 'Spread');
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
  checkValidity(x, 'x', 'RelSpread');
  // Check positivity (priority 1)
  checkPositivity(x, 'x', 'RelSpread');
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
  checkValidity(x, 'x', 'Shift');
  checkValidity(y, 'y', 'Shift');

  return fastShift(x, y, [0.5], false)[0];
}

/**
 * Calculate the Ratio - median of all pairwise ratios (x[i] / y[j])
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
  checkValidity(x, 'x', 'Ratio');
  // Check validity for y (priority 0, subject y)
  checkValidity(y, 'y', 'Ratio');
  // Check positivity for x (priority 1, subject x)
  checkPositivity(x, 'x', 'Ratio');
  // Check positivity for y (priority 1, subject y)
  checkPositivity(y, 'y', 'Ratio');

  const nx = x.length;
  const ny = y.length;
  const pairwiseRatios: number[] = [];
  for (let i = 0; i < nx; i++) {
    for (let j = 0; j < ny; j++) {
      pairwiseRatios.push(x[i] / y[j]);
    }
  }

  return median(pairwiseRatios);
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
  checkValidity(x, 'x', 'AvgSpread');
  // Check validity for y (priority 0, subject y)
  checkValidity(y, 'y', 'AvgSpread');
  // Check sparity for x (priority 2, subject x)
  checkSparity(x, 'x', 'AvgSpread');
  // Check sparity for y (priority 2, subject y)
  checkSparity(y, 'y', 'AvgSpread');

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
  checkValidity(x, 'x', 'Disparity');
  // Check validity for y (priority 0, subject y)
  checkValidity(y, 'y', 'Disparity');
  // Check sparity for x (priority 2, subject x)
  checkSparity(x, 'x', 'Disparity');
  // Check sparity for y (priority 2, subject y)
  checkSparity(y, 'y', 'Disparity');

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
export function shiftBounds(x: number[], y: number[], misrate: number): Bounds {
  // Check validity for x
  checkValidity(x, 'x', 'ShiftBounds');
  // Check validity for y
  checkValidity(y, 'y', 'ShiftBounds');

  const n = x.length;
  const m = y.length;

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
