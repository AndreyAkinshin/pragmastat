/**
 * Base interface for distributions that can generate samples.
 */

import { Rng } from '../rng';

/**
 * Base interface for distributions that can generate samples.
 */
export interface Distribution {
  /**
   * Generate a single sample from this distribution.
   */
  sample(rng: Rng): number;

  /**
   * Probability density at x. Zero outside the support.
   */
  densityAt(x: number): number;

  /**
   * Probability of not exceeding x. Zero below the support, one above it.
   */
  cumulativeAt(x: number): number;

  /**
   * The value below which a fraction p of the distribution lies.
   *
   * A p outside (0, 1) returns the endpoints of the support, which are infinities
   * for the unbounded families.
   */
  quantileAt(p: number): number;

  /**
   * Generate multiple samples from this distribution.
   */
  samples(rng: Rng, count: number): number[];
}
