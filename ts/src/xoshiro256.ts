/**
 * xoshiro256++ PRNG implementation for cross-language reproducibility.
 *
 * Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
 *
 * Uses BigInt internally for 64-bit arithmetic correctness.
 */

const U64_MAX = 0xffffffffffffffffn;

function rotl(x: bigint, k: bigint): bigint {
  return ((x << k) | (x >> (64n - k))) & U64_MAX;
}

/**
 * SplitMix64 PRNG for seed expansion
 */
class SplitMix64 {
  private state: bigint;

  constructor(seed: bigint) {
    this.state = seed & U64_MAX;
  }

  next(): bigint {
    this.state = (this.state + 0x9e3779b97f4a7c15n) & U64_MAX;
    let z = this.state;
    z = ((z ^ (z >> 30n)) * 0xbf58476d1ce4e5b9n) & U64_MAX;
    z = ((z ^ (z >> 27n)) * 0x94d049bb133111ebn) & U64_MAX;
    return (z ^ (z >> 31n)) & U64_MAX;
  }
}

/**
 * xoshiro256++ PRNG
 *
 * This is the jump-free version of the algorithm. It passes BigCrush
 * and is used by .NET 6+, Julia, and Rust's rand crate.
 */
export class Xoshiro256PlusPlus {
  private state: [bigint, bigint, bigint, bigint];

  constructor(seed: bigint) {
    const sm = new SplitMix64(seed);
    this.state = [sm.next(), sm.next(), sm.next(), sm.next()];
  }

  nextU64(): bigint {
    const s = this.state;
    const result = (rotl((s[0] + s[3]) & U64_MAX, 23n) + s[0]) & U64_MAX;

    const t = (s[1] << 17n) & U64_MAX;

    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];

    s[2] ^= t;
    s[3] = rotl(s[3], 45n);

    return result;
  }

  uniform(): number {
    // Use upper 53 bits for maximum precision in float64
    const u64 = this.nextU64();
    return Number(u64 >> 11n) * (1.0 / Number(1n << 53n));
  }

  /**
   * Generate a uniform integer in [min, max).
   * @throws RangeError if max - min exceeds i64 range.
   */
  uniformInt(min: bigint, max: bigint): bigint {
    if (min >= max) {
      return min;
    }
    const range = max - min;
    // Validate range fits in i64 (for cross-language consistency)
    if (range > 0x7fffffffffffffffn) {
      throw new RangeError('uniform_int: range overflow (max - min exceeds i64)');
    }
    return min + (this.nextU64() % range);
  }
}

// FNV-1a hash constants
const FNV_OFFSET_BASIS = 0xcbf29ce484222325n;
const FNV_PRIME = 0x00000100000001b3n;

/**
 * Compute FNV-1a 64-bit hash of a string
 */
export function fnv1aHash(s: string): bigint {
  let hash = FNV_OFFSET_BASIS;
  const encoder = new TextEncoder();
  const bytes = encoder.encode(s);

  for (const byte of bytes) {
    hash ^= BigInt(byte);
    hash = (hash * FNV_PRIME) & U64_MAX;
  }

  return hash;
}
