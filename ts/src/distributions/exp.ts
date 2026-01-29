/**
 * Exponential distribution.
 */

import { MACHINE_EPSILON } from '../constants';
import { Rng } from '../rng';
import { Distribution } from './distribution';

/**
 * Exponential distribution with given rate parameter.
 *
 * The mean of this distribution is 1/rate.
 *
 * @example
 * const rng = new Rng(1729);
 * const dist = new Exp(1); // rate=1, mean=1
 * const sample = dist.sample(rng);
 */
export class Exp implements Distribution {
  private readonly rate: number;

  /**
   * Create a new exponential distribution with given rate.
   *
   * @param rate - Rate parameter (lambda > 0).
   * @throws Error if rate <= 0.
   */
  constructor(rate: number) {
    if (rate <= 0) {
      throw new Error('rate must be positive');
    }
    this.rate = rate;
  }

  sample(rng: Rng): number {
    // Inverse CDF method: -ln(1 - U) / rate
    let u = rng.uniform();
    // Avoid log(0) - use machine epsilon for cross-language consistency
    if (u === 1.0) {
      u = 1.0 - MACHINE_EPSILON;
    }
    return -Math.log(1.0 - u) / this.rate;
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
