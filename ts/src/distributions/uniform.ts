/**
 * Uniform distribution on a bounded interval.
 */

import { Rng } from '../rng';
import { Distribution } from './distribution';

/**
 * Uniform distribution on [min, max).
 *
 * @example
 * const rng = new Rng("demo-dist-uniform");
 * const dist = new Uniform(0, 10);
 * const sample = dist.sample(rng);
 */
export class Uniform implements Distribution {
  private readonly min: number;
  private readonly max: number;

  /**
   * Create a new uniform distribution on [min, max).
   *
   * @param min - Lower bound (inclusive).
   * @param max - Upper bound (exclusive).
   * @throws Error if min or max is not finite, or if min is not less than max.
   */
  constructor(min: number, max: number) {
    if (!Number.isFinite(min)) {
      throw new Error('min must be finite');
    }
    if (!Number.isFinite(max)) {
      throw new Error('max must be finite');
    }
    if (!(min < max)) {
      throw new Error('min must be less than max');
    }
    this.min = min;
    this.max = max;
  }

  sample(rng: Rng): number {
    return this.min + rng.uniformFloat() * (this.max - this.min);
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
