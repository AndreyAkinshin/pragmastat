/**
 * Multiplicative (Log-Normal) distribution.
 */

import { Rng } from '../rng';
import { additiveCumulative } from '../additiveCumulative';
import { additiveQuantile } from '../additiveQuantile';
import { expFunction } from '../expFunction';
import { logFunction } from '../logFunction';
import { SQRT_2PI } from './constants';
import { Distribution } from './distribution';
import { samplingCumulance } from './samplingCumulance';

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
  private readonly logMean: number;
  private readonly logStdDev: number;

  /**
   * Create a new multiplicative (log-normal) distribution.
   *
   * @param logMean - Mean of log values (location parameter).
   * @param logStdDev - Standard deviation of log values (scale parameter).
   * @throws Error if logMean is not finite, or if logStdDev is not finite and positive.
   */
  constructor(logMean: number, logStdDev: number) {
    if (!Number.isFinite(logMean)) {
      throw new Error('logMean must be finite');
    }
    if (!(Number.isFinite(logStdDev) && logStdDev > 0)) {
      throw new Error('logStdDev must be finite and positive');
    }
    this.logMean = logMean;
    this.logStdDev = logStdDev;
  }

  sample(rng: Rng): number {
    return this.quantileAt(samplingCumulance(rng));
  }

  /**
   * The support is x > 0, and that is the test. It used to be x < 1e-9, which is not
   * the edge of the support but an arbitrary point inside it: a Multiplic(-30, 1) has
   * its median at 9.4e-14, so that threshold reported zero density and zero
   * probability across essentially the whole distribution.
   */
  densityAt(x: number): number {
    if (x <= 0) {
      return 0;
    }
    const z = (logFunction(x) - this.logMean) / this.logStdDev;
    return expFunction((-z * z) / 2) / (x * this.logStdDev * SQRT_2PI);
  }

  /**
   * A lognormal variable's distribution function is the normal one applied to log x,
   * so this goes through additiveCumulative rather than spelling the same thing out
   * through erf.
   */
  cumulativeAt(x: number): number {
    if (x <= 0) {
      return 0;
    }
    return additiveCumulative((logFunction(x) - this.logMean) / this.logStdDev);
  }

  quantileAt(p: number): number {
    if (p <= 0) {
      return 0;
    }
    if (p >= 1) {
      return Infinity;
    }
    return expFunction(this.logMean + this.logStdDev * additiveQuantile(p));
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
