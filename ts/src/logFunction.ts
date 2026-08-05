/**
 * The natural logarithm this toolkit evaluates, in place of the platform's.
 *
 * IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
 * nothing about the logarithm. The reason this one has to be ours is stronger than for most
 * transcendentals: ratio is exp(median(log x - log y)), so a last-bit difference in log moves the
 * median's input and changes which pairwise element is selected. That is a different answer, not a
 * small error in the same one.
 *
 * The constants and the polynomial come from tests/oracles/fit_log.py, which documents the
 * reduction and what is fitted. Which of the two range endpoints a port names depends on how it
 * splits x: a library frexp returns a significand in [0.5, 1) and the reduction doubles the low
 * end, while reading the exponent field gives one in [1, 2) and it halves the high end. Both
 * arrive at the same [sqrt(2)/2, sqrt(2)) and the seven agree bit for bit; only the endpoint each
 * has to name differs.
 */

// Constants of the range reduction. ln 2 is split so that k*LN2_HI is exact: it carries 32
// significant bits and |k| needs at most 11, which leaves the product inside the 53 available.
const LN2_HI = 0.6931471803691238;
const LN2_LO = 1.9082149292705877e-10;
const SQRT2 = 1.4142135623730951;

// Chebyshev fit of (log((1+s)/(1-s)) - 2*s) / s^3 over the reduced range. The leading terms are
// the recognizable 2/3, 2/5, 2/7 of the odd series doubled, which is where the assembly's missing
// factor of two lives; the tail departs from the series because the fit minimizes the worst error
// rather than matching Taylor.
const POLY_P = [
  0.666666666666667, 0.39999999999899505, 0.28571428625975487, 0.2222221113479508,
  0.18182889125261723, 0.15331721600556042, 0.14616449685043406,
];

const scratch = new DataView(new ArrayBuffer(8));

/**
 * Splits `x` into `[m, k]` with `x === m * 2**k` and `m` in `[sqrt(2)/2, sqrt(2))`.
 *
 * JavaScript has no frexp, so the exponent is read out of the bit pattern. Subnormals are scaled
 * up first: their exponent field is zero and carries no information, so the leading bit has to be
 * brought into the normal range before the field means anything. The ports with a library frexp
 * land on the same pair, since this is a decomposition rather than a rounding.
 */
function reduce(x: number): [number, number] {
  let value = x;
  let k = 0;

  scratch.setFloat64(0, value);
  if ((scratch.getUint32(0) >>> 20) % 0x800 === 0) {
    // Subnormal: multiply by 2^54, which is exact, and account for it in k.
    value *= 1.8014398509481984e16;
    k -= 54;
    scratch.setFloat64(0, value);
  }

  const high = scratch.getUint32(0);
  k += ((high >>> 20) % 0x800) - 1023;
  // Replace the exponent field with the one for [1, 2).
  scratch.setUint32(0, (high & 0x000fffff) | 0x3ff00000);
  let m = scratch.getFloat64(0);

  // Recenter on [sqrt(2)/2, sqrt(2)) so |f| stays small at both ends. Halving is exact.
  if (m >= SQRT2) {
    m *= 0.5;
    k += 1;
  }

  return [m, k];
}

/**
 * Returns the natural logarithm of `x`.
 *
 * The special cases follow IEEE 754 and match what every platform returns for them, since they
 * are the cases a platform has no freedom about.
 */
export function logFunction(x: number): number {
  if (Number.isNaN(x)) return x;
  if (x < 0) return NaN;
  if (x === 0) return -Infinity;
  if (x === Infinity) return Infinity;
  if (x === 1) return 0;

  const [m, k] = reduce(x);
  const f = m - 1;
  const s = f / (2 + f);
  const z = s * s;

  let acc = POLY_P[POLY_P.length - 1];
  for (let i = POLY_P.length - 2; i >= 0; i--) {
    acc = acc * z + POLY_P[i];
  }
  const r = z * acc;
  const hfsq = 0.5 * f * f;

  if (k === 0) return f - (hfsq - s * (hfsq + r));
  return k * LN2_HI - (hfsq - (s * (hfsq + r) + k * LN2_LO) - f);
}
