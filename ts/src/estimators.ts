/**
 * Pragmastat estimators implementation
 */

import { median } from './utils';

/**
 * Calculate the Center - median of all pairwise averages (x[i] + x[j])/2
 * @param x Array of sample values
 * @returns The center estimate
 */
export function center(x: number[]): number {
  const n = x.length;
  if (n === 0) {
    throw new Error('Input array cannot be empty');
  }

  const pairwiseAverages: number[] = [];
  for (let i = 0; i < n; i++) {
    for (let j = i; j < n; j++) {
      pairwiseAverages.push((x[i] + x[j]) / 2);
    }
  }

  return median(pairwiseAverages);
}

/**
 * Calculate the Spread - median of all pairwise absolute differences |x[i] - x[j]|
 * @param x Array of sample values
 * @returns The spread estimate
 */
export function spread(x: number[]): number {
  const n = x.length;
  if (n === 0) {
    throw new Error('Input array cannot be empty');
  }
  if (n === 1) {
    return 0;
  }

  const pairwiseDifferences: number[] = [];
  for (let i = 0; i < n; i++) {
    for (let j = i + 1; j < n; j++) {
      pairwiseDifferences.push(Math.abs(x[j] - x[i]));
    }
  }

  return median(pairwiseDifferences);
}

/**
 * Calculate the Volatility - ratio of Spread to absolute Center
 * @param x Array of sample values
 * @returns The volatility estimate
 */
export function volatility(x: number[]): number {
  if (x.length === 0) {
    throw new Error('Input array cannot be empty');
  }

  const s = spread(x);
  const c = center(x);

  if (c === 0) {
    throw new Error('Volatility is undefined when Center equals zero');
  }

  return s / Math.abs(c);
}

/**
 * Calculate the Precision - 2 * Spread / sqrt(n)
 * @param x Array of sample values
 * @returns The precision estimate
 */
export function precision(x: number[]): number {
  const n = x.length;
  if (n === 0) {
    throw new Error('Input array cannot be empty');
  }

  return (2 * spread(x)) / Math.sqrt(n);
}

/**
 * Calculate the MedShift - median of all pairwise differences (x[i] - y[j])
 * @param x First sample
 * @param y Second sample
 * @returns The shift estimate
 */
export function medShift(x: number[], y: number[]): number {
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
 * Calculate the MedRatio - median of all pairwise ratios (x[i] / y[j])
 * @param x First sample
 * @param y Second sample
 * @returns The ratio estimate
 */
export function medRatio(x: number[], y: number[]): number {
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
 * Calculate the MedSpread - weighted average of spreads: (n*Spread(x) + m*Spread(y))/(n+m)
 * @param x First sample
 * @param y Second sample
 * @returns The combined spread estimate
 */
export function medSpread(x: number[], y: number[]): number {
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
 * Calculate the MedDisparity - MedShift / MedSpread
 * @param x First sample
 * @param y Second sample
 * @returns The disparity estimate
 */
export function medDisparity(x: number[], y: number[]): number {
  if (x.length === 0 || y.length === 0) {
    throw new Error('Input arrays cannot be empty');
  }

  const shift = medShift(x, y);
  const combinedSpread = medSpread(x, y);

  if (combinedSpread === 0) {
    return Infinity;
  }

  return shift / combinedSpread;
}
