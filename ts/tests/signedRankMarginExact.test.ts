import { signedRankMargin } from '../src/signedRankMargin';
import { centerBounds } from '../src/estimators';
import { expectBitwise } from './bitwise';

/**
 * The exact signed-rank branch must be the same function here as in the other six ports.
 *
 * It was not. This port divided the cumulative count by the total with scaled BigInt arithmetic,
 * `Number(cumulative * 10n ** 18n / total) / 1e18`, which truncates the quotient to 1e-18 and
 * returns exactly zero below that. The other six convert both operands to binary64 and divide
 * once. Below n = 54 the counts fit the exact integer range of a double and the two agree; above
 * it they do not, and at n = 63 they selected a different index for 1020 of the 2017 values of w.
 *
 * The reachable consequence was a different confidence interval: at the achievable misrate floor
 * for 63 observations this port returned [2.5, 61.5] where every other port returned [1, 63].
 * The center-bounds fixtures stop at n = 20, so nothing looked here.
 */
describe('exact signed-rank margin', () => {
  it('agrees with the other ports at the achievable floor for n = 63', () => {
    const x = Array.from({ length: 63 }, (_, i) => i + 1);
    const bounds = centerBounds(x, Math.pow(2, -62));
    expectBitwise('centerBounds lower at the floor', bounds.lower, 1);
    expectBitwise('centerBounds upper at the floor', bounds.upper, 63);
  });

  it('matches the shared margins across the exact branch', () => {
    // Computed by the other six ports, which all use one binary64 division of two counts.
    const expected: Array<[number, number, number]> = [
      [55, 0.05, 1074],
      [55, 0.001, 772],
      [58, 0.05, 1206],
      [60, 0.01, 1136],
      [62, 0.05, 1396],
      [63, 0.05, 1444],
      [63, 0.01, 1270],
      [63, 0.001, 1072],
    ];
    for (const [n, misrate, margin] of expected) {
      expect(signedRankMargin(n, misrate)).toBe(margin);
    }
  });
});
