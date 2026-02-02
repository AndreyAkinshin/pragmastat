/**
 * Deterministic random number generator for cross-language reproducibility.
 *
 * The Rng class provides a deterministic PRNG based on xoshiro256++ that
 * produces identical sequences across all Pragmastat language implementations.
 */

import { Xoshiro256PlusPlus, fnv1aHash } from './xoshiro256';

/**
 * A deterministic random number generator.
 *
 * Rng uses xoshiro256++ internally and guarantees identical output sequences
 * across all Pragmastat language implementations when initialized with the same seed.
 *
 * @example
 * // Create from integer seed
 * const rng = new Rng(1729);
 * const value = rng.uniform();
 *
 * @example
 * // Create from string seed
 * const rng = new Rng("experiment-1");
 *
 * @example
 * // Shuffle an array
 * const rng = new Rng(1729);
 * const shuffled = rng.shuffle([1, 2, 3, 4, 5]);
 *
 * @example
 * // Sample k elements
 * const rng = new Rng(1729);
 * const sampled = rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
 */
export class Rng {
  private inner: Xoshiro256PlusPlus;

  /**
   * Create a new Rng.
   *
   * @param seed - If number: use as integer seed directly.
   *               If string: hash using FNV-1a to produce a numeric seed.
   *               If undefined: use current time for entropy (non-deterministic).
   */
  constructor(seed?: number | string) {
    let seedBigInt: bigint;

    if (seed === undefined) {
      seedBigInt = BigInt(Date.now());
    } else if (typeof seed === 'string') {
      seedBigInt = fnv1aHash(seed);
    } else {
      // Convert number to bigint, handling negative numbers via two's complement
      seedBigInt = BigInt.asIntN(64, BigInt(seed));
    }

    this.inner = new Xoshiro256PlusPlus(seedBigInt);
  }

  // ========================================================================
  // Floating Point Methods
  // ========================================================================

  /**
   * Generate a uniform random float in [0, 1).
   *
   * Uses 53 bits of precision for the mantissa.
   *
   * @returns Random value in [0, 1).
   */
  uniform(): number {
    return this.inner.uniform();
  }

  /**
   * Generate a uniform random float in [min, max).
   *
   * @param min - Minimum value (inclusive).
   * @param max - Maximum value (exclusive).
   * @returns Random value in [min, max). Returns min if min >= max.
   */
  uniformRange(min: number, max: number): number {
    return this.inner.uniformRange(min, max);
  }

  // ========================================================================
  // Integer Methods
  // ========================================================================

  /**
   * Generate a uniform random integer in [min, max).
   *
   * Uses modulo reduction which introduces slight bias for ranges that don't
   * evenly divide 2^64. This bias is negligible for statistical simulations
   * but not suitable for cryptographic applications.
   *
   * @param min - Minimum value (inclusive).
   * @param max - Maximum value (exclusive).
   * @returns Random integer in [min, max). Returns min if min >= max.
   */
  uniformInt(min: number, max: number): number {
    const result = this.inner.uniformInt(BigInt(min), BigInt(max));
    return Number(result);
  }

  /**
   * Generate a uniform random BigInt in [min, max).
   *
   * Uses modulo reduction which introduces slight bias for ranges that don't
   * evenly divide 2^64. This bias is negligible for statistical simulations
   * but not suitable for cryptographic applications.
   *
   * @param min - Minimum value (inclusive).
   * @param max - Maximum value (exclusive).
   * @returns Random BigInt in [min, max). Returns min if min >= max.
   */
  uniformBigInt(min: bigint, max: bigint): bigint {
    return this.inner.uniformInt(min, max);
  }

  // ========================================================================
  // Boolean Methods
  // ========================================================================

  /**
   * Generate a uniform random boolean with P(true) = 0.5.
   *
   * @returns Random boolean value.
   */
  uniformBool(): boolean {
    return this.inner.uniformBool();
  }

  /**
   * Return a shuffled copy of the input array.
   *
   * Uses the Fisher-Yates shuffle algorithm for uniform distribution.
   * The original array is not modified.
   *
   * @param x - Input array to shuffle.
   * @returns Shuffled copy of the input.
   */
  shuffle<T>(x: T[]): T[] {
    const result = [...x];
    const n = result.length;

    // Fisher-Yates shuffle (backwards)
    for (let i = n - 1; i > 0; i--) {
      const j = this.uniformInt(0, i + 1);
      [result[i], result[j]] = [result[j], result[i]];
    }

    return result;
  }

  /**
   * Sample k elements from the input array without replacement.
   *
   * Uses selection sampling to maintain order of first appearance.
   * Returns up to k elements; if k >= x.length, returns all elements.
   *
   * @param x - Input array to sample from.
   * @param k - Number of elements to sample. Must be non-negative.
   * @returns Array of k sampled elements.
   * @throws Error if k is negative.
   */
  sample<T>(x: T[], k: number): T[] {
    if (k < 0) {
      throw new Error('sample: k must be non-negative');
    }
    const n = x.length;
    if (k >= n) {
      return [...x];
    }

    const result: T[] = [];
    let remaining = k;

    for (let i = 0; i < n && remaining > 0; i++) {
      const available = n - i;
      // Probability of selecting this item: remaining / available
      if (this.uniform() * available < remaining) {
        result.push(x[i]);
        remaining--;
      }
    }

    return result;
  }
}
