/**
 * Fast O((m + n) * log(precision)) implementation for computing quantiles
 * of all pairwise differences {x_i - y_j}.
 *
 * Based on binary search with two-pointer counting algorithm.
 * Internal implementation - not part of public API.
 */

import { log } from './assumptions';

/**
 * Computes quantiles of all pairwise differences { x_i - y_j }.
 * Time: O((m + n) * log(precision)) per quantile. Space: O(1).
 *
 * @param x First array of numeric values
 * @param y Second array of numeric values
 * @param p Probabilities in [0, 1]
 * @param assumeSorted If false, arrays will be sorted
 * @returns Array of quantile values corresponding to probabilities in p
 * @internal
 */
export function fastShift(
  x: number[],
  y: number[],
  p: number[],
  assumeSorted: boolean = false,
): number[] {
  if (!x || !y || !p) {
    throw new Error('All inputs must be non-null');
  }
  if (x.length === 0 || y.length === 0) {
    throw new Error('x and y must be non-empty');
  }

  // Validate probabilities
  for (const pk of p) {
    if (isNaN(pk) || pk < 0.0 || pk > 1.0) {
      throw new Error('Probabilities must be within [0, 1]');
    }
  }

  // Sort if needed
  const xs = assumeSorted ? x : [...x].sort((a, b) => a - b);
  const ys = assumeSorted ? y : [...y].sort((a, b) => a - b);

  // Check for NaN in data
  for (const xi of xs) {
    if (isNaN(xi)) {
      throw new Error('NaN values found in x');
    }
  }
  for (const yj of ys) {
    if (isNaN(yj)) {
      throw new Error('NaN values found in y');
    }
  }

  const m = xs.length;
  const n = ys.length;
  const total = m * n;

  // Type-7 quantile: h = 1 + (n-1)*p, then interpolate between floor(h) and ceil(h)
  const requiredRanks = new Set<number>();
  const interpolationParams: Array<{
    lowerRank: number;
    upperRank: number;
    weight: number;
  }> = [];

  for (let i = 0; i < p.length; i++) {
    const h = 1.0 + (total - 1) * p[i];
    let lowerRank = Math.floor(h);
    let upperRank = Math.ceil(h);
    const weight = h - lowerRank;

    if (lowerRank < 1) lowerRank = 1;
    if (upperRank > total) upperRank = total;

    interpolationParams.push({ lowerRank, upperRank, weight });
    requiredRanks.add(lowerRank);
    requiredRanks.add(upperRank);
  }

  // Compute values for required ranks
  const rankValues = new Map<number, number>();
  for (const rank of Array.from(requiredRanks).sort((a, b) => a - b)) {
    rankValues.set(rank, selectKthPairwiseDiff(xs, ys, rank));
  }

  // Interpolate to get final quantile values
  const result: number[] = [];
  for (const { lowerRank, upperRank, weight } of interpolationParams) {
    const lower = rankValues.get(lowerRank)!;
    const upper = rankValues.get(upperRank)!;
    result.push(weight === 0.0 ? lower : (1.0 - weight) * lower + weight * upper);
  }

  return result;
}

/**
 * Binary search in [min_diff, max_diff] that snaps to actual discrete values.
 * Avoids materializing all m*n differences.
 */
function selectKthPairwiseDiff(x: number[], y: number[], k: number): number {
  const m = x.length;
  const n = y.length;
  const total = m * n;

  if (k < 1 || k > total) {
    throw new Error(`k must be between 1 and ${total}`);
  }

  let searchMin = x[0] - y[n - 1];
  let searchMax = x[m - 1] - y[0];

  if (isNaN(searchMin) || isNaN(searchMax)) {
    throw new Error('NaN in input values');
  }

  const maxIterations = 128; // Sufficient for double precision convergence
  let prevMin = -Infinity;
  let prevMax = Infinity;

  for (let iter = 0; iter < maxIterations && searchMin !== searchMax; iter++) {
    const mid = midpoint(searchMin, searchMax);
    const { countLessOrEqual, closestBelow, closestAbove } = countAndNeighbors(x, y, mid);

    if (closestBelow === closestAbove) {
      return closestBelow;
    }

    // No progress means we're stuck between two discrete values
    if (searchMin === prevMin && searchMax === prevMax) {
      return countLessOrEqual >= k ? closestBelow : closestAbove;
    }

    prevMin = searchMin;
    prevMax = searchMax;

    if (countLessOrEqual >= k) {
      searchMax = closestBelow;
    } else {
      searchMin = closestAbove;
    }
  }

  if (searchMin !== searchMax) {
    throw new Error('Convergence failure (pathological input)');
  }

  return searchMin;
}

/**
 * Two-pointer algorithm: counts pairs where x[i] - y[j] <= threshold, and tracks
 * the closest actual differences on either side of threshold.
 */
function countAndNeighbors(
  x: number[],
  y: number[],
  threshold: number,
): { countLessOrEqual: number; closestBelow: number; closestAbove: number } {
  const m = x.length;
  const n = y.length;
  let count = 0;
  let maxBelow = -Infinity;
  let minAbove = Infinity;

  let j = 0;
  for (let i = 0; i < m; i++) {
    // Move j forward while x[i] - y[j] > threshold
    while (j < n && x[i] - y[j] > threshold) {
      j++;
    }

    // Count elements where x[i] - y[j] <= threshold
    count += n - j;

    // Track the closest differences on either side of threshold
    if (j < n) {
      const diff = x[i] - y[j];
      if (diff > maxBelow) maxBelow = diff;
    }

    if (j > 0) {
      const diff = x[i] - y[j - 1];
      if (diff < minAbove) minAbove = diff;
    }
  }

  // Fallback to actual min/max if no boundaries found
  if (!isFinite(maxBelow)) {
    maxBelow = x[0] - y[n - 1];
  }
  if (!isFinite(minAbove)) {
    minAbove = x[m - 1] - y[0];
  }

  return {
    countLessOrEqual: count,
    closestBelow: maxBelow,
    closestAbove: minAbove,
  };
}

/**
 * Compute midpoint avoiding overflow
 */
function midpoint(a: number, b: number): number {
  return a + (b - a) * 0.5;
}

/**
 * Computes quantiles of all pairwise ratios { x_i / y_j } via log-transformation.
 * Time: O((m + n) * log(precision)) per quantile. Space: O(m + n).
 *
 * @param x First array of positive numeric values
 * @param y Second array of positive numeric values
 * @param p Probabilities in [0, 1]
 * @param assumeSorted If false, arrays will be sorted
 * @returns Array of quantile values corresponding to probabilities in p
 * @internal
 */
export function fastRatio(
  x: number[],
  y: number[],
  p: number[],
  assumeSorted: boolean = false,
): number[] {
  if (!x || !y || !p) {
    throw new Error('All inputs must be non-null');
  }
  if (x.length === 0 || y.length === 0) {
    throw new Error('x and y must be non-empty');
  }

  // Log-transform both samples (includes positivity check)
  const logX = log(x, 'x');
  const logY = log(y, 'y');

  // Delegate to fastShift in log-space
  const logResult = fastShift(logX, logY, p, assumeSorted);

  // Exp-transform back to ratio-space
  return logResult.map((v) => Math.exp(v));
}
