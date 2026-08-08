/**
 * Exponential distribution.
 */

import { MACHINE_EPSILON } from '../constants';
import { Rng } from '../rng';
import { expFunction } from '../expFunction';
import { logFunction } from '../logFunction';
import { Distribution } from './distribution';

/**
 * Exponential distribution with given rate parameter.
 *
 * The mean of this distribution is 1/rate.
 *
 * @example
 * const rng = new Rng("demo-dist-exp");
 * const dist = new Exp(1); // rate=1, mean=1
 * const sample = dist.sample(rng);
 */
export class Exp implements Distribution {
  private readonly rate: number;

  /**
   * Create a new exponential distribution with given rate.
   *
   * @param rate - Rate parameter (lambda > 0).
   * @throws Error if rate is not finite and positive.
   */
  constructor(rate: number) {
    if (!(Number.isFinite(rate) && rate > 0)) {
      throw new Error('rate must be finite and positive');
    }
    this.rate = rate;
  }

  sample(rng: Rng): number {
    // Inverse CDF method: -ln(1 - U) / rate
    let u = rng.uniformFloat();
    // Avoid log(0) - use machine epsilon for cross-language consistency
    if (u === 1.0) {
      u = 1.0 - MACHINE_EPSILON;
    }
    return -Math.log(1.0 - u) / this.rate;
  }

  densityAt(x: number): number {
    if (x < 0) {
      return 0;
    }
    return this.rate * expFunction(-this.rate * x);
  }

  cumulativeAt(x: number): number {
    if (x < 0) {
      return 0;
    }
    return 1 - expFunction(-this.rate * x);
  }

  quantileAt(p: number): number {
    if (p <= 0) {
      return 0;
    }
    if (p >= 1) {
      return Infinity;
    }
    return -logFunction(1 - p) / this.rate;
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
