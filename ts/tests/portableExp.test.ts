import { pow2, portableExp } from '../src/portableExp';
import { expectBitwise } from './bitwise';

// The exponential is the one library call on the margin path that IEEE 754 leaves free, so
// this suite treats it the way the selection estimators are treated: by payload, never by
// tolerance.
//
// What is checked here is what the shared fixture cannot check. The agreement of the seven
// implementations is checked by tests/portable-exp, argument by argument, loaded in
// reference.test.ts; this file holds the two claims that fixture has no way to carry.
//
// The first is that `2 ** n` is exact. JavaScript has no ldexp, and ECMAScript defines the
// exponentiation operator as implementation-approximated rather than correctly-rounded, so
// the scaling step of the reduction rests on a property of the runtime rather than of the
// standard. It is therefore checked against a construction that cannot be wrong: the binary64
// with biased exponent n + 1023 and a zero significand is 2^n by the definition of the format.
// This is a claim about the runtime rather than about the shared behavior, which is why the
// range it covers is derived from the constants below rather than shared with the other ports.
//
// The second is the behavior outside the finite range. JSON has no literal for an infinity or
// a NaN, so no fixture can state that the top of the range overflows or that the special values
// pass through.

const scratch = new DataView(new ArrayBuffer(8));

/** 2^n from the format definition: biased exponent n + 1023, significand zero. */
function pow2FromExponentField(n: number): number {
  scratch.setBigUint64(0, BigInt(n + 1023) << BigInt(52));
  return scratch.getFloat64(0);
}

// Mirrors the cutoffs and the reciprocal of ln 2 in src/portableExp.ts, so that the exponent
// range the scaling can reach is derived here rather than asserted from memory.
const INV_LN2 = 1.4426950408889634;
const MAX_ARG = 709.79;
const MIN_ARG = -745.2;

describe('portableExp', () => {
  // k = floor(y*INV_LN2 + 1/2) is non-decreasing in y, so over the band the cutoffs admit it
  // is bounded by its values at the two ends. The scaling then uses trunc(k/2) and k - trunc(k/2).
  it('reaches exponents -538..512 and no others', () => {
    const kMin = Math.floor(MIN_ARG * INV_LN2 + 0.5);
    const kMax = Math.floor(MAX_ARG * INV_LN2 + 0.5);
    expect([kMin, kMax]).toEqual([-1075, 1024]);

    const exponents = [kMin, kMax].flatMap((k) => [Math.trunc(k / 2), k - Math.trunc(k / 2)]);
    expect(Math.min(...exponents)).toBe(-538);
    expect(Math.max(...exponents)).toBe(512);
  });

  // Every exponent the reduction can hand to the scaling is a normal one, which is the reason
  // the scaling is split in two: a single step would have to represent 2^k for k down to -1075.
  it('computes an exact 2^n for every reachable exponent', () => {
    for (let n = -538; n <= 512; n++) {
      expectBitwise(`pow2(${n})`, pow2(n), pow2FromExponentField(n));
    }
  });

  // The fixture stops at 709 and 709.78 because the next representable results are not finite.
  // The high cutoff itself is not the early return: `y > 709.79` is false at 709.79, so the
  // reduction runs and the scaling is what overflows. The early return is above it.
  it('overflows to infinity at the top of the range', () => {
    expect(portableExp(MAX_ARG)).toBe(Infinity);
    expect(portableExp(710)).toBe(Infinity);
  });

  it('carries the infinities and NaN through', () => {
    expect(portableExp(Infinity)).toBe(Infinity);
    expectBitwise('portableExp(-Infinity)', portableExp(-Infinity), 0);
    expect(Number.isNaN(portableExp(NaN))).toBe(true);
  });
});
