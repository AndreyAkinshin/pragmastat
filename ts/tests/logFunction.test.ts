import { logFunction } from '../src/logFunction';
import { expectBitwise } from './bitwise';

// The arguments outside the reduction's domain, which the shared log-function fixture cannot
// carry: JSON has no way to write a negative infinity or a NaN, so the generated suite holds only
// the positive finite range and these are covered here or nowhere.
//
// These are the cases IEEE 754 leaves a platform no freedom about, so agreeing on them is not a
// choice this toolkit makes; it is what makes the replacement a logarithm at all.
describe('logFunction', () => {
  it('returns the declared values outside the reduction domain', () => {
    expectBitwise('logFunction(+0)', logFunction(0), -Infinity);
    expectBitwise('logFunction(-0)', logFunction(-0), -Infinity);
    expect(Number.isNaN(logFunction(-1))).toBe(true);
    expect(Number.isNaN(logFunction(NaN))).toBe(true);
    expect(logFunction(Infinity)).toBe(Infinity);
  });

  // Exactly +0, not -0: ratio exponentiates this result, and the two zeros are distinct doubles
  // that the payload comparison in every other suite would tell apart. `toBe` would not: it uses
  // Object.is for -0 but === semantics elsewhere, so this goes through the payload helper.
  it('returns positive zero at one', () => {
    expectBitwise('logFunction(1)', logFunction(1), 0);
  });
});
