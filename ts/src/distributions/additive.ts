/**
 * Additive (Normal/Gaussian) distribution.
 */

import { Rng } from '../rng';
import { additiveCumulative } from '../additiveCumulative';
import { additiveQuantile } from '../additiveQuantile';
import { expFunction } from '../expFunction';
import { SQRT_2PI } from './constants';
import { Distribution } from './distribution';
import { samplingCumulance } from './samplingCumulance';

/**
 * Additive (Normal/Gaussian) distribution with given mean and standard deviation.
 *
 * Draws by inverse transform, through the toolkit's own inverse normal CDF.
 *
 * @example
 * const rng = new Rng("demo-dist-additive");
 * const dist = new Additive(0, 1); // Standard normal
 * const sample = dist.sample(rng);
 */
export class Additive implements Distribution {
  private readonly mean: number;
  private readonly stdDev: number;

  /**
   * Create a new additive (normal) distribution.
   *
   * @param mean - Location parameter (center of the distribution).
   * @param stdDev - Scale parameter (standard deviation).
   * @throws Error if mean is not finite, or if stdDev is not finite and positive.
   */
  constructor(mean: number, stdDev: number) {
    if (!Number.isFinite(mean)) {
      throw new Error('mean must be finite');
    }
    if (!(Number.isFinite(stdDev) && stdDev > 0)) {
      throw new Error('stdDev must be finite and positive');
    }
    this.mean = mean;
    this.stdDev = stdDev;
  }

  sample(rng: Rng): number {
    return this.quantileAt(samplingCumulance(rng));
  }

  densityAt(x: number): number {
    const z = (x - this.mean) / this.stdDev;
    return expFunction((-z * z) / 2) / (this.stdDev * SQRT_2PI);
  }

  cumulativeAt(x: number): number {
    return additiveCumulative((x - this.mean) / this.stdDev);
  }

  /**
   * The inverse goes through additiveQuantile, which is what that function was built
   * for. It replaces two Newton steps taken against an error function accurate to
   * 1.5e-07, which had left this quantile eight orders of magnitude coarser than the
   * cumulative beside it. The infinities at the ends are the ones additiveQuantile
   * already returns.
   */
  quantileAt(p: number): number {
    return this.mean + this.stdDev * additiveQuantile(p);
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
