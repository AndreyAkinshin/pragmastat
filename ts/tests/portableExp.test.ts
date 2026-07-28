import { pow2, portableExp } from '../src/portableExp';
import { expectBitwise } from './bitwise';

// The exponential is the one library call on the margin path that IEEE 754 leaves free, so
// this suite treats it the way the selection estimators are treated: by payload, never by
// tolerance. Two claims are checked here.
//
// The first is that `2 ** n` is exact. JavaScript has no ldexp, and ECMAScript defines the
// exponentiation operator as implementation-approximated rather than correctly-rounded, so
// the scaling step of the reduction rests on a property of the runtime rather than of the
// standard. It is therefore checked against a construction that cannot be wrong: the binary64
// with biased exponent n + 1023 and a zero significand is 2^n by the definition of the format.
//
// The second is that the assembled result matches the other ports bit for bit. The expected
// payloads below were produced by go/portable_exp.go.

const scratch = new DataView(new ArrayBuffer(8));

/** The double carrying this binary64 payload, written as 16 hex digits. */
function fromHex(hex: string): number {
  scratch.setBigUint64(0, BigInt(`0x${hex}`));
  return scratch.getFloat64(0);
}

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

  // Arguments and results as binary64 payloads, from go/portable_exp.go. The band, both
  // cutoffs, the denormal results below -708, and a few values a reader can check against a
  // table: exp(1) = 2.718281828459045 and exp(-1) = 0.36787944117144233.
  it('returns the payload the other ports return', () => {
    const cases: [string, string][] = [
      ['c08749999999999a', '0000000000000000'], // exp(-745.2), the low cutoff
      ['c087480000000000', '0000000000000001'], // exp(-745), the smallest denormal
      ['c08624cccccccccd', '000d0d8838ef6116'], // exp(-708.6), denormal
      ['c085e00000000000', '00d14f2b0fb9307f'], // exp(-700)
      ['c059000000000000', '36ea8c1f14e2af5d'], // exp(-100)
      ['bff0000000000000', '3fd78b56362cef38'], // exp(-1)
      ['bfe0000000000000', '3fe368b2fc6f960a'], // exp(-0.5)
      ['bfd62e42fefa39ef', '3fe6a09e667f3bcc'], // exp(-ln2/2), a reduction endpoint
      ['0000000000000000', '3ff0000000000000'], // exp(0)
      ['3fd62e42fefa39ef', '3ff6a09e667f3bcc'], // exp(ln2/2), the other endpoint
      ['3fe62e42fefa39ef', '4000000000000000'], // exp(ln2)
      ['3fe0000000000000', '3ffa61298e1e069c'], // exp(0.5)
      ['3ff0000000000000', '4005bf0a8b145769'], // exp(1)
      ['4059000000000000', '48f3494a9b171bf5'], // exp(100)
      ['40862e3d70a3d70a', '7fefe9ce5c4c52b4'], // exp(709.78), the largest finite result
      ['40862e51eb851eb8', '7ff0000000000000'], // exp(709.79), the high cutoff
    ];
    for (const [argHex, expectedHex] of cases) {
      const arg = fromHex(argHex);
      expectBitwise(`portableExp(${arg})`, portableExp(arg), fromHex(expectedHex));
    }
  });

  it('carries the infinities and NaN through', () => {
    expect(portableExp(Infinity)).toBe(Infinity);
    expectBitwise('portableExp(-Infinity)', portableExp(-Infinity), 0);
    expect(Number.isNaN(portableExp(NaN))).toBe(true);
  });
});
