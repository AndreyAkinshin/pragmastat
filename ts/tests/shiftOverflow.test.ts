import { shift } from '../src/estimators';

// Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
// +-Infinity on extreme finite input, turning the midpoint into NaN and returning
// +-Infinity instead of the true finite shift.
const MAX = Number.MAX_VALUE;

describe('shift overflow', () => {
  it('returns 0 for symmetric extremes', () => {
    expect(shift([-MAX, MAX], [-MAX, MAX], true)).toBe(0);
  });

  it('returns MAX for one-sided extremes', () => {
    expect(shift([0, MAX], [-MAX, 0], true)).toBe(MAX);
  });
});
