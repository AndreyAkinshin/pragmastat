import { pairwiseMargin } from '../src/pairwiseMargin';

// Regression: the exact binomial coefficient must use integer arithmetic. Float
// accumulation overflowed 2^53 in the partial products for C(56,27), giving a
// margin of 784 instead of the correct 782 at misrate 1.0.
describe('pairwiseMargin consistency', () => {
  it('matches the exact-integer binomial (782, not float 784)', () => {
    expect(pairwiseMargin(29, 27, 1.0)).toBe(782);
  });
});
