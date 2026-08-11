/**
 * Binary64 primitives shared by the distribution estimation models.
 *
 * Every operation here is written out explicitly so that all seven language implementations
 * evaluate the same graph and produce identical bits. None of these helpers may be replaced by
 * a fused, vectorized or library variant without a new algorithm identifier.
 */

import { SMALLEST_POSITIVE_SUBNORMAL } from './constants';

const scratch = new DataView(new ArrayBuffer(8));

/** Next representable binary64 value toward positive infinity. */
export function distNextUp(value: number): number {
  if (Number.isNaN(value) || value === Infinity) return value;
  if (value === 0) return SMALLEST_POSITIVE_SUBNORMAL;

  scratch.setFloat64(0, value);
  const bits = scratch.getBigInt64(0) + (value > 0 ? 1n : -1n);
  scratch.setBigInt64(0, bits);
  return scratch.getFloat64(0);
}

/** Next representable binary64 value toward negative infinity. */
export function distNextDown(value: number): number {
  if (Number.isNaN(value) || value === -Infinity) return value;
  if (value === 0) return -SMALLEST_POSITIVE_SUBNORMAL;

  scratch.setFloat64(0, value);
  const bits = scratch.getBigInt64(0) + (value > 0 ? -1n : 1n);
  scratch.setBigInt64(0, bits);
  return scratch.getFloat64(0);
}

/** Overflow-safe midpoint of two finite values. */
export function distMidpoint(a: number, b: number): number {
  if (a < 0 && 0 < b) return a / 2 + b / 2;
  return a + (b - a) / 2;
}

/** Interpolates between `a` and `b`, reproducing both endpoints exactly. */
export function distLerp(a: number, b: number, t: number): number {
  if (t === 0) return a;
  if (t === 1) return b;
  if (a < 0 && 0 < b) return (1 - t) * a + t * b;
  return a + t * (b - a);
}

/**
 * Ratio clamped to the unit interval. Callers must have proved `0 <= num <= den` and `den > 0`.
 *
 * Reachable as NaN only through a broken `assumeSorted` promise (a caller-input contract
 * violation, not a value this library ever produces from valid input), which is why this throws
 * a plain `Error` rather than an `AssumptionError`: the sample itself already satisfied every
 * assumption checked at construction.
 */
export function distUnitRatio(num: number, den: number): number {
  const t = num / den;
  if (Number.isNaN(t)) {
    throw new Error('Numerical conformance failure: unitRatio produced NaN');
  }
  return Math.min(1, Math.max(0, t));
}

/** Smallest integer k with `k >= ceil(n / k)`, computed without forming `k*k`. */
export function distCeilSqrt(n: number): number {
  let k = 1;
  while (k < Math.floor((n + k - 1) / k)) {
    k++;
  }
  return k;
}

/** First index i in `[from, to)` with `values[i] >= key`, over an ascending array. */
export function distLowerBound(
  values: readonly number[],
  from: number,
  to: number,
  key: number,
): number {
  let lo = from;
  let hi = to;
  while (lo < hi) {
    const mid = lo + Math.floor((hi - lo) / 2);
    if (values[mid] < key) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return lo;
}

/** First index i with `values[i] > key`, over an ascending array. */
export function distUpperBound(values: readonly number[], key: number): number {
  let lo = 0;
  let hi = values.length;
  while (lo < hi) {
    const mid = lo + Math.floor((hi - lo) / 2);
    if (values[mid] <= key) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return lo;
}
