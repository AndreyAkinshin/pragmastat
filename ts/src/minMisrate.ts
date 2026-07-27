/**
 * MinAchievableMisrate functions for bounds validation.
 */

import { AssumptionError } from './assumptions';
import { binomialCoefficient } from './binomial';

/**
 * Computes the minimum achievable misrate for one-sample bounds.
 *
 * For a sample of size n, the minimum achievable misrate is 2^(1-n),
 * which corresponds to the probability of the most extreme configuration
 * in the Wilcoxon signed-rank distribution.
 *
 * @param n Sample size (must be positive)
 * @returns Minimum achievable misrate
 */
export function minAchievableMisrateOneSample(n: number): number {
  if (n <= 0) {
    throw AssumptionError.domain('x');
  }
  return Math.pow(2, 1 - n);
}

/**
 * Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.
 *
 * @param n Size of first sample (must be positive)
 * @param m Size of second sample (must be positive)
 * @returns Minimum achievable misrate
 */
export function minAchievableMisrateTwoSample(n: number, m: number): number {
  if (n <= 0) {
    throw AssumptionError.domain('x');
  }
  if (m <= 0) {
    throw AssumptionError.domain('y');
  }
  // Shares one binomial with the exact pairwise-margin distribution (see binomial.ts):
  // the floor and the distribution it guards must be computed by the same route.
  return 2.0 / binomialCoefficient(n + m, n);
}
