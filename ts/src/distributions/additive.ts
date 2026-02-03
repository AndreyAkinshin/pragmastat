/**
 * Additive (Normal/Gaussian) distribution.
 */

import { SMALLEST_POSITIVE_SUBNORMAL } from '../constants';
import { Rng } from '../rng';
import { Distribution } from './distribution';

/**
 * Additive (Normal/Gaussian) distribution with given mean and standard deviation.
 *
 * Uses the Box-Muller transform to generate samples.
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
   * @throws Error if stdDev <= 0.
   */
  constructor(mean: number, stdDev: number) {
    if (stdDev <= 0) {
      throw new Error('stdDev must be positive');
    }
    this.mean = mean;
    this.stdDev = stdDev;
  }

  sample(rng: Rng): number {
    // Box-Muller transform
    let u1 = rng.uniform();
    const u2 = rng.uniform();

    // Avoid log(0) - use smallest positive subnormal for cross-language consistency
    if (u1 === 0) {
      u1 = SMALLEST_POSITIVE_SUBNORMAL;
    }

    const r = Math.sqrt(-2.0 * Math.log(u1));
    const theta = 2.0 * Math.PI * u2;

    // Use the first of the two Box-Muller outputs
    const z = r * Math.cos(theta);

    return this.mean + z * this.stdDev;
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }
}
