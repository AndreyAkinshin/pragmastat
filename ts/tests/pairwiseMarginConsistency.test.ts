import { binomialCoefficient } from '../src/binomial';
import { minAchievableMisrateTwoSample } from '../src/minMisrate';
import { pairwiseMargin } from '../src/pairwiseMargin';

// Regressions on the binomial coefficient behind the pairwise margin.
//
// Float accumulation overflowed 2^53 in the partial products for C(56,27), giving a
// margin of 784 instead of the correct 782 at misrate 1.0, which is why totals below
// n+m = 62 go through exact integer arithmetic.
describe('pairwiseMargin consistency', () => {
  it('matches the exact-integer binomial (782, not float 784)', () => {
    expect(pairwiseMargin(29, 27, 1.0)).toBe(782);
  });

  // C(n+m, n) and C(n+m, m) are one number, so the two call sites must get one float.
  // At the misrate floor the comparison `1/total >= misrate/2` is an exact tie, and an
  // asymmetric binomial would decide it by whichever of the two rounded higher.
  it('computes a binomial symmetric in k <-> n-k', () => {
    for (let n = 0; n <= 400; n++) {
      for (let k = 0; k <= n; k++) {
        expect(binomialCoefficient(n, k)).toBe(binomialCoefficient(n, n - k));
      }
    }
  });

  // Pins both sides of the switch from exact integers to the recurrence. C(62,31) is
  // 465428353255261088 and the recurrence returns a value 3 ULP above it. That is the
  // contract, not a defect: all seven implementations run this sequence of multiplies
  // and divides and all return this float. An accuracy fix here is a divergence.
  it('pins both sides of the exact/recurrence threshold', () => {
    expect(binomialCoefficient(61, 30)).toBe(2.3271417662763053e17);
    expect(binomialCoefficient(62, 31)).toBe(4.6542835325526125e17);
  });

  // The misrate floor is 2 / C(n+m, n) and the exact pass normalizes its pmf by
  // C(n+m, m). When the two were computed by different routes, 37152 of the 79800
  // sample-size pairs with n+m <= 400 excluded pairs at their own floor.
  it('returns a zero margin at the misrate floor', () => {
    for (let n = 1; n <= 60; n++) {
      for (let m = 1; m <= 60; m++) {
        expect(pairwiseMargin(n, m, minAchievableMisrateTwoSample(n, m))).toBe(0);
      }
    }
  });

  // Past the point where C(n+m, n) leaves binary64 range the floor is 0, not an error.
  it('saturates the misrate floor beyond binary64 range', () => {
    expect(minAchievableMisrateTwoSample(515, 515)).toBe(0);
    const margin = pairwiseMargin(600, 600, 1e-3);
    expect(margin).toBeGreaterThan(0);
    expect(margin % 2).toBe(0);
  });
});
