/**
 * Pragmastat estimators implementation
 */

import { median } from './utils';
import { fastCenter } from './fastCenter';
import { fastSpread } from './fastSpread';

/**
 * Calculate the Center - median of all pairwise averages (x[i] + x[j])/2
 * Uses fast O(n log n) algorithm.
 * @param x Array of sample values
 * @returns The center estimate
 */
export function center(x: number[]): number {
  return fastCenter(x);
}

/**
 * Calculate the Spread - median of all pairwise absolute differences |x[i] - x[j]|
 * Uses fast O(n log n) algorithm.
 * @param x Array of sample values
 * @returns The spread estimate
 */
export function spread(x: number[]): number {
  return fastSpread(x);
}

/**
 * Calculate the RelSpread - ratio of Spread to absolute Center
 * @param x Array of sample values
 * @returns The relative spread estimate
 */
export function relSpread(x: number[]): number {
  if (x.length === 0) {
    throw new Error('Input array cannot be empty');
  }

  const s = spread(x);
  const c = center(x);

  if (c === 0) {
    throw new Error('RelSpread is undefined when Center equals zero');
  }

  return s / Math.abs(c);
}

/**
 * Calculate the Shift - median of all pairwise differences (x[i] - y[j])
 * @param x First sample
 * @param y Second sample
 * @returns The shift estimate
 */
export function shift(x: number[], y: number[]): number {
  const nx = x.length;
  const ny = y.length;

  if (nx === 0 || ny === 0) {
    throw new Error('Input arrays cannot be empty');
  }

  const pairwiseDifferences: number[] = [];
  for (let i = 0; i < nx; i++) {
    for (let j = 0; j < ny; j++) {
      pairwiseDifferences.push(x[i] - y[j]);
    }
  }

  return median(pairwiseDifferences);
}

/**
 * Calculate the Ratio - median of all pairwise ratios (x[i] / y[j])
 * @param x First sample
 * @param y Second sample
 * @returns The ratio estimate
 */
export function ratio(x: number[], y: number[]): number {
  const nx = x.length;
  const ny = y.length;

  if (nx === 0 || ny === 0) {
    throw new Error('Input arrays cannot be empty');
  }

  // Check that all y values are strictly positive
  if (y.some((val) => val <= 0)) {
    throw new Error('All values in y must be strictly positive');
  }

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
 * @param x First sample
 * @param y Second sample
 * @returns The combined spread estimate
 */
export function avgSpread(x: number[], y: number[]): number {
  const nx = x.length;
  const ny = y.length;

  if (nx === 0 || ny === 0) {
    throw new Error('Input arrays cannot be empty');
  }

  // Calculate weighted average of individual spreads
  const spreadX = spread(x);
  const spreadY = spread(y);

  return (nx * spreadX + ny * spreadY) / (nx + ny);
}

/**
 * Calculate the Disparity - Shift / AvgSpread
 * @param x First sample
 * @param y Second sample
 * @returns The disparity estimate
 */
export function disparity(x: number[], y: number[]): number {
  if (x.length === 0 || y.length === 0) {
    throw new Error('Input arrays cannot be empty');
  }

  const shiftVal = shift(x, y);
  const combinedSpread = avgSpread(x, y);

  if (combinedSpread === 0) {
    return Infinity;
  }

  return shiftVal / combinedSpread;
}
