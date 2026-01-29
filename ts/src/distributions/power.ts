/**
 * Power (Pareto) distribution.
 */

import { MACHINE_EPSILON } from '../constants';
import { Rng } from '../rng';
import { Distribution } from './distribution';

/**
 * Power (Pareto) distribution with minimum value and shape parameter.
 *
 * Follows a power-law distribution where large values are rare but possible.
 *
 * @example
 * const rng = new Rng(1729);
 * const dist = new Power(1, 2); // min=1, shape=2
 * const sample = dist.sample(rng);
 */
export class Power implements Distribution {
  private readonly min: number;
  private readonly shape: number;

  /**
   * Create a new power (Pareto) distribution.
   *
   * @param min - Minimum value (lower bound, > 0).
   * @param shape - Shape parameter (alpha > 0, controls tail heaviness).
   * @throws Error if min <= 0 or shape <= 0.
   */
  constructor(min: number, shape: number) {
    if (min <= 0) {
      throw new Error('min must be positive');
    }
    if (shape <= 0) {
      throw new Error('shape must be positive');
    }
    this.min = min;
    this.shape = shape;
  }

  sample(rng: Rng): number {
    // Inverse CDF method: min / (1 - U)^(1/shape)
    let u = rng.uniform();
    // Avoid division by zero - use machine epsilon for cross-language consistency
    if (u === 1.0) {
      u = 1.0 - MACHINE_EPSILON;
    }
    return this.min / Math.pow(1.0 - u, 1.0 / this.shape);
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
