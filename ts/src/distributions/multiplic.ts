/**
 * Multiplicative (Log-Normal) distribution.
 */

import { Rng } from '../rng';
import { Distribution } from './distribution';
import { Additive } from './additive';

/**
 * Multiplicative (Log-Normal) distribution.
 *
 * The logarithm of samples follows an Additive (Normal) distribution.
 *
 * @example
 * const rng = new Rng("demo-dist-multiplic");
 * const dist = new Multiplic(0, 1);
 * const sample = dist.sample(rng);
 */
export class Multiplic implements Distribution {
  private readonly additive: Additive;

  /**
   * Create a new multiplicative (log-normal) distribution.
   *
   * @param logMean - Mean of log values (location parameter).
   * @param logStdDev - Standard deviation of log values (scale parameter).
   * @throws Error if logStdDev <= 0.
   */
  constructor(logMean: number, logStdDev: number) {
    if (logStdDev <= 0) {
      throw new Error('logStdDev must be positive');
    }
    this.additive = new Additive(logMean, logStdDev);
  }

  sample(rng: Rng): number {
    return Math.exp(this.additive.sample(rng));
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
