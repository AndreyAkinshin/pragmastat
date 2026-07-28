import {
  center,
  spread,
  shift,
  ratio,
  disparity,
  centerBounds,
  spreadBounds,
  shiftBounds,
  ratioBounds,
  disparityBounds,
  _avgSpread,
  _avgSpreadBounds,
} from '../src/estimators';
import { Sample } from '../src/sample';
import { expectBitwise, expectNoNegativeZero } from './bitwise';

/**
 * No estimator may report a negative zero.
 *
 * A sample holding both `+0` and `-0` leaves the sign of a selected zero to the sorting
 * algorithm rather than to the data, and the seven ports sort their own way. Each returns the
 * sign its sort happened to produce, which breaks the `exact` conformance class. Every
 * estimator therefore normalizes `-0` to `+0` on the way out (see `normalizeZero`).
 *
 * The claim is about the sign bit, so `===` proves nothing here: `-0 === 0`. Every assertion
 * below reads the raw payload.
 *
 * Only outputs are normalized. The samples in this file keep their `-0` values and are still
 * accepted as valid input.
 */

/** Renders a sample so that `-0` is visible in a failure message (`String(-0)` is `"0"`). */
function render(values: number[]): string {
  return `[${values.map((v) => (Object.is(v, -0) ? '-0' : String(v))).join(', ')}]`;
}

/**
 * Pairwise differences over these two put the single `-0 - 0` product exactly at the median
 * (rank 32 of 63): every other pair differs, so no other zero competes for that position, and
 * the two-sided counts around it match. Without normalization `shift`, `disparity` and
 * `shiftBounds` all report that `-0`.
 */
const NEG_ZERO_X = [-0.0, 0.5, 1.5, 3.5, 6.5, 7.5, 8.5];
const NEG_ZERO_Y = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

describe('center never reports a negative zero', () => {
  const samples = [
    [0.0, -0.0, 0.0, -0.0, 1.0],
    [-0.0, -0.0],
    [-0.0, 0.0],
    [0.0, -0.0],
    [-0.0, -0.0, -0.0],
    [-1.0, 1.0],
    [-2.0, -0.0, 2.0],
  ];

  it.each(samples)('center(%#) is +0', (...sample: number[]) => {
    expectBitwise(`center(${render(sample)})`, center(sample), 0);
  });

  it('normalizes on the Sample path too', () => {
    expectBitwise('center(Sample([-0, -0])).value', center(Sample.of([-0.0, -0.0])).value, 0);
  });
});

describe('scalar estimators never report a negative zero', () => {
  it('shift', () => {
    expectBitwise('shift([-0, -0], [0, 0])', shift([-0.0, -0.0], [0.0, 0.0]), 0);
    expectBitwise('shift(NEG_ZERO_X, NEG_ZERO_Y)', shift(NEG_ZERO_X, NEG_ZERO_Y), 0);
  });

  it('disparity', () => {
    expectBitwise('disparity(NEG_ZERO_X, NEG_ZERO_Y)', disparity(NEG_ZERO_X, NEG_ZERO_Y), 0);
  });

  // spread, ratio and avgSpread cannot reach -0 as they stand: spread is rejected unless
  // strictly positive, ratio exponentiates, and avgSpread averages absolute differences. The
  // assertions pin the contract for all six scalar estimators anyway, so a future change to any
  // of those three routes fails here rather than in a conformance run.
  it('spread', () => {
    expectNoNegativeZero('spread([-0, -0, 1])', spread([-0.0, -0.0, 1.0]));
  });

  it('ratio', () => {
    expectNoNegativeZero('ratio([1, 2], [1, 2])', ratio([1.0, 2.0], [1.0, 2.0]));
  });

  it('avgSpread', () => {
    const value = _avgSpread(Sample.of(NEG_ZERO_X), Sample.of(NEG_ZERO_Y)).value;
    expectNoNegativeZero('avgSpread(NEG_ZERO_X, NEG_ZERO_Y)', value);
  });
});

describe('bounds estimators never report a negative zero', () => {
  it('centerBounds', () => {
    const spec = centerBounds([0.0, -0.0, 0.0, -0.0, 1.0, -1.0], 0.3);
    expectBitwise('centerBounds(spec).lower', spec.lower, 0);
    expectBitwise('centerBounds(spec).upper', spec.upper, 0);

    const tied = centerBounds([-0.0, -0.0], 1.0);
    expectBitwise('centerBounds([-0, -0], 1).lower', tied.lower, 0);
    expectBitwise('centerBounds([-0, -0], 1).upper', tied.upper, 0);
  });

  it('shiftBounds', () => {
    const tied = shiftBounds([-0.0, -0.0], [0.0, 0.0], 1.0);
    expectBitwise('shiftBounds([-0, -0], [0, 0], 1).lower', tied.lower, 0);
    expectBitwise('shiftBounds([-0, -0], [0, 0], 1).upper', tied.upper, 0);

    // misrate 1 collapses both endpoints onto the median, which is the -0 pair.
    const median = shiftBounds(NEG_ZERO_X, NEG_ZERO_Y, 1.0);
    expectBitwise('shiftBounds(NEG_ZERO_X, NEG_ZERO_Y, 1).lower', median.lower, 0);
    expectBitwise('shiftBounds(NEG_ZERO_X, NEG_ZERO_Y, 1).upper', median.upper, 0);
  });

  it('disparityBounds', () => {
    // Both samples place the -0 pair on the order statistic the shift bounds select, once for
    // the lower endpoint and once for the upper; dividing it by a positive avg-spread bound
    // keeps the sign.
    const low = disparityBounds([-0.0, 0.5, 5.5, 8.5, 8.5, 8.5, 8.5], NEG_ZERO_Y, 0.9, 'seed');
    expectBitwise('disparityBounds(lower case).lower', low.lower, 0);
    expectNoNegativeZero('disparityBounds(lower case).upper', low.upper);

    const high = disparityBounds([-0.0, 0.5, 0.5, 0.5, 0.5, 5.5, 8.5], NEG_ZERO_Y, 0.9, 'seed');
    expectNoNegativeZero('disparityBounds(upper case).lower', high.lower);
    expectBitwise('disparityBounds(upper case).upper', high.upper, 0);
  });

  // As with spread and ratio above: absolute differences and exponentiation cannot produce -0
  // today, and these assertions keep it that way.
  it('spreadBounds', () => {
    const b = spreadBounds(NEG_ZERO_X, 0.5, 'seed');
    expectNoNegativeZero('spreadBounds(NEG_ZERO_X).lower', b.lower);
    expectNoNegativeZero('spreadBounds(NEG_ZERO_X).upper', b.upper);
  });

  it('ratioBounds', () => {
    const b = ratioBounds([1.0, 2.0, 3.0], [1.0, 2.0, 3.0], 0.9);
    expectNoNegativeZero('ratioBounds.lower', b.lower);
    expectNoNegativeZero('ratioBounds.upper', b.upper);
  });

  it('avgSpreadBounds', () => {
    const b = _avgSpreadBounds(Sample.of(NEG_ZERO_X), Sample.of(NEG_ZERO_Y), 0.9, 'seed');
    expectNoNegativeZero('avgSpreadBounds.lower', b.lower);
    expectNoNegativeZero('avgSpreadBounds.upper', b.upper);
  });
});
