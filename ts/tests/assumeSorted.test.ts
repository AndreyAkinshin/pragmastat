import {
  center,
  spread,
  shift,
  ratio,
  disparity,
  centerBounds,
  shiftBounds,
  ratioBounds,
  spreadBounds,
  disparityBounds,
} from '../src/estimators';
import { expectBitwise, expectBitwiseBounds } from './bitwise';

function sortedAsc(x: number[]): number[] {
  return [...x].sort((a, b) => a - b);
}

describe('raw API assumeSorted=true roundtrip', () => {
  // Unsorted, all-positive inputs (positivity is required by ratio/ratioBounds).
  const x = [3, 1, 2, 5, 4, 8, 6, 7];
  const y = [13, 11, 12, 15, 14, 18, 16, 17];
  const xs = sortedAsc(x);
  const ys = sortedAsc(y);
  const misrate = 0.3;
  const seed = 'assume-sorted-seed';

  /**
   * Every comparison below is BITWISE (`expectBitwise`, a binary64 payload
   * comparison), never `toBeCloseTo`.
   *
   * `assumeSorted` selects which of two routes reaches the sort: the caller's
   * (`true`) or the estimator's own copying sort (`false`). Both hand the kernel
   * the same ascending array, so the two sides run the identical sequence of
   * floating-point operations on the identical data. There is no arithmetic
   * between them for a rounding to enter: they agree to the last bit, or one
   * route does something the other does not, which is the defect this file
   * exists to catch. A tolerance would report that defect as a pass.
   *
   * This holds for `ratio` and `ratioBounds` too. They are `exp(shift(log x,
   * log y))` and therefore approximate in the cross-language sense, but here
   * BOTH sides take that same route: `log` is monotone and elementwise, so
   * `log(sort(x))` is `sort(log(x))` element for element, and the shared `exp`
   * closes on the same argument.
   */
  describe('order-independent estimators: sorted+true === unsorted+false', () => {
    it('center', () => {
      expectBitwise('center', center(xs, true), center(x, false));
    });

    it('spread', () => {
      expectBitwise('spread', spread(xs, true), spread(x, false));
    });

    it('shift', () => {
      expectBitwise('shift', shift(xs, ys, true), shift(x, y, false));
    });

    it('ratio', () => {
      expectBitwise('ratio', ratio(xs, ys, true), ratio(x, y, false));
    });

    it('disparity', () => {
      expectBitwise('disparity', disparity(xs, ys, true), disparity(x, y, false));
    });

    it('centerBounds', () => {
      expectBitwiseBounds(
        'centerBounds',
        centerBounds(xs, misrate, true),
        centerBounds(x, misrate, false),
      );
    });

    it('shiftBounds', () => {
      expectBitwiseBounds(
        'shiftBounds',
        shiftBounds(xs, ys, misrate, true),
        shiftBounds(x, y, misrate, false),
      );
    });

    it('ratioBounds', () => {
      expectBitwiseBounds(
        'ratioBounds',
        ratioBounds(xs, ys, misrate, true),
        ratioBounds(x, y, misrate, false),
      );
    });
  });

  describe('shuffle-based bounds: flag never changes the result (same array, same seed)', () => {
    // The disjoint-pair shuffle always runs on the passed order, so assumeSorted
    // never affects the shuffle. assumeSorted is INERT only on SORTED input: on
    // UNSORTED input passing assumeSorted=true is undefined behavior — exactly
    // like every other estimator — because the sparity (spread>0) check runs
    // spreadImpl(x, assumeSorted), feeding unsorted data to a sorted-only kernel
    // (it may hit the iteration cap and ERROR, or pass only by luck). So the
    // fair true-vs-false comparison is on a genuinely SORTED array, where both
    // flag values must agree.
    it('spreadBounds: assumeSorted=true === assumeSorted=false on a SORTED array', () => {
      expectBitwiseBounds(
        'spreadBounds',
        spreadBounds(xs, misrate, seed, true),
        spreadBounds(xs, misrate, seed, false),
      );
    });

    it('disparityBounds: assumeSorted=true === assumeSorted=false on the same arrays', () => {
      expectBitwiseBounds(
        'disparityBounds',
        disparityBounds(xs, ys, misrate, seed, true),
        disparityBounds(xs, ys, misrate, seed, false),
      );
    });
  });
});
