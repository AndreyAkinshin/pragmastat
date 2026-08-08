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

  /**
   * The support is half-open, so the density is zero at max and positive at min.
   * This is the one family assembled from arithmetic alone: no primitive of the
   * toolkit appears on any of its three paths.
   */
  densityAt(x: number): number {
    // NaN is refused before the bounds are compared. This is the only one of the fifteen
    // functions whose result does not depend on its argument, so a NaN reaching the
    // comparison satisfies neither half of it and falls through to an ordinary density.
    if (Number.isNaN(x)) {
      return x;
    }
    if (x < this.min || x >= this.max) {
      return 0;
    }
    return 1 / (this.max - this.min);
  }

  cumulativeAt(x: number): number {
    if (x < this.min) {
      return 0;
    }
    if (x >= this.max) {
      return 1;
    }
    return (x - this.min) / (this.max - this.min);
  }

  quantileAt(p: number): number {
    if (p <= 0) {
      return this.min;
    }
    if (p >= 1) {
      return this.max;
    }
    return this.min + p * (this.max - this.min);
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
