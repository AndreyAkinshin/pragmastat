/**
 * Power (Pareto) distribution.
 */

import { MACHINE_EPSILON } from '../constants';
import { Rng } from '../rng';
import { expFunction } from '../expFunction';
import { logFunction } from '../logFunction';
import { Distribution } from './distribution';

/**
 * Power (Pareto) distribution with minimum value and shape parameter.
 *
 * Follows a power-law distribution where large values are rare but possible.
 *
 * @example
 * const rng = new Rng("demo-dist-power");
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
   * @throws Error if min or shape is not finite and positive.
   */
  constructor(min: number, shape: number) {
    if (!(Number.isFinite(min) && min > 0)) {
      throw new Error('min must be finite and positive');
    }
    if (!(Number.isFinite(shape) && shape > 0)) {
      throw new Error('shape must be finite and positive');
    }
    this.min = min;
    this.shape = shape;
  }

  sample(rng: Rng): number {
    // Inverse CDF method: min / (1 - U)^(1/shape)
    let u = rng.uniformFloat();
    // Avoid division by zero - use machine epsilon for cross-language consistency
    if (u === 1.0) {
      u = 1.0 - MACHINE_EPSILON;
    }
    return this.min / Math.pow(1.0 - u, 1.0 / this.shape);
  }

  /**
   * shape * min^shape / x^(shape+1), rearranged as
   * (shape / x) * exp(shape * (log min - log x)). The toolkit owns an exponential and
   * a logarithm and owns no general power, so a real exponent is spelled as the
   * exponential of a logarithm. This family is the reason the logarithm had to exist
   * before the trio could.
   *
   * The support is [min, infinity), closed at the left, so the density at min is
   * shape / min rather than zero, while the cumulative there is zero. Uniform reads
   * the same way at its own lower endpoint.
   */
  densityAt(x: number): number {
    if (x < this.min) {
      return 0;
    }
    return (this.shape / x) * expFunction(this.shape * (logFunction(this.min) - logFunction(x)));
  }

  cumulativeAt(x: number): number {
    if (x <= this.min) {
      return 0;
    }
    return 1 - expFunction(this.shape * (logFunction(this.min) - logFunction(x)));
  }

  quantileAt(p: number): number {
    if (p <= 0) {
      return this.min;
    }
    if (p >= 1) {
      return Infinity;
    }
    return this.min * expFunction(-logFunction(1 - p) / this.shape);
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
