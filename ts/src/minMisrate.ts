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
  // Repeated halving rather than Math.pow: this is a power of two, and scaling by an exponent is exact in binary64. A general
  // power function returns the same value in every implementation anyone ships, but the
  // specification does not require it to, and this value is a domain boundary: it decides
  // which misrates the toolkit accepts at all.
  // ECMAScript defines Math.pow as implementation-approximated, so it is not the
  // primitive to reach for when the value has to be exact. Each halving is.
  let v = 1;
  for (let i = 1; i < n; i++) {
    v /= 2;
  }
  return v;
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
