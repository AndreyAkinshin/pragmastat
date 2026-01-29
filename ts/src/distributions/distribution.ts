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
   * Generate multiple samples from this distribution.
   */
  samples(rng: Rng, count: number): number[];
}
