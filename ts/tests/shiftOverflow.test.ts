import { shift } from '../src/estimators';
import { expectBitwise } from './bitwise';

// Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
// +-Infinity on extreme finite input, turning the midpoint into NaN and returning
// +-Infinity instead of the true finite shift.
const MAX = Number.MAX_VALUE;

describe('shift overflow', () => {
  it('returns 0 for symmetric extremes', () => {
    // Bitwise: the sign of this zero is part of the claim, and +0 and -0 are
    // indistinguishable in decimal.
    expectBitwise('shift([-MAX,MAX], [-MAX,MAX])', shift([-MAX, MAX], [-MAX, MAX], true), 0);
  });

  it('returns MAX for one-sided extremes', () => {
    expectBitwise('shift([0,MAX], [-MAX,0])', shift([0, MAX], [-MAX, 0], true), MAX);
  });
});
