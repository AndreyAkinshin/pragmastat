/**
 * FNV-1a hash algorithm (64-bit) and seed derivation helpers.
 *
 * Reference: http://www.isthe.com/chongo/tech/comp/fnv/
 *
 * Shared by the Monahan-selection estimators (centerImpl, spreadImpl) to derive
 * a deterministic RNG seed from their float64 input.
 *
 * Internal implementation - not part of public API.
 */

const FNV_OFFSET_BASIS = 0xcbf29ce484222325n;
const FNV_PRIME = 0x00000100000001b3n;
const MASK64 = (1n << 64n) - 1n;

/**
 * Convert a float64 to its IEEE 754 binary representation as bigint.
 */
function float64ToBits(value: number): bigint {
  const buffer = new ArrayBuffer(8);
  new Float64Array(buffer)[0] = value;
  const view = new DataView(buffer);
  return (BigInt(view.getUint32(4, true)) << 32n) | BigInt(view.getUint32(0, true));
}

/**
 * Derive a deterministic seed from input values using FNV-1a hash.
 *
 * @param values Array of numeric values
 * @returns A signed 64-bit seed (as a bigint) suitable for {@link Rng}
 * @internal
 */
export function deriveSeed(values: readonly number[]): bigint {
  let hash = FNV_OFFSET_BASIS;
  for (const v of values) {
    const bits = float64ToBits(v);
    for (let i = 0; i < 8; i++) {
      hash ^= (bits >> BigInt(i * 8)) & 0xffn;
      hash = (hash * FNV_PRIME) & MASK64;
    }
  }
  return BigInt.asIntN(64, hash);
}
