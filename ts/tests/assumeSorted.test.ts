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
  Bounds,
} from '../src/estimators';

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
   * Every comparison below is BITWISE (`toBe`, i.e. `Object.is`), never
   * `toBeCloseTo`.
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
   *
   * A one-ULP failure stays readable without a bit-pattern dump: JavaScript
   * prints a double with enough digits to round-trip, so two distinct doubles
   * never render as the same string.
   */
  describe('order-independent estimators: sorted+true === unsorted+false', () => {
    it('center', () => {
      expect(center(xs, true)).toBe(center(x, false));
    });

    it('spread', () => {
      expect(spread(xs, true)).toBe(spread(x, false));
    });

    it('shift', () => {
      expect(shift(xs, ys, true)).toBe(shift(x, y, false));
    });

    it('ratio', () => {
      expect(ratio(xs, ys, true)).toBe(ratio(x, y, false));
    });

    it('disparity', () => {
      expect(disparity(xs, ys, true)).toBe(disparity(x, y, false));
    });

    it('centerBounds', () => {
      const a = centerBounds(xs, misrate, true);
      const b = centerBounds(x, misrate, false);
      expect(a.lower).toBe(b.lower);
      expect(a.upper).toBe(b.upper);
    });

    it('shiftBounds', () => {
      const a = shiftBounds(xs, ys, misrate, true);
      const b = shiftBounds(x, y, misrate, false);
      expect(a.lower).toBe(b.lower);
      expect(a.upper).toBe(b.upper);
    });

    it('ratioBounds', () => {
      const a = ratioBounds(xs, ys, misrate, true);
      const b = ratioBounds(x, y, misrate, false);
      expect(a.lower).toBe(b.lower);
      expect(a.upper).toBe(b.upper);
    });
  });

  describe('shuffle-based bounds: flag never changes the result (same array, same seed)', () => {
    function expectIdentical(a: Bounds, b: Bounds): void {
      expect(a.lower).toBe(b.lower);
      expect(a.upper).toBe(b.upper);
    }

    // The disjoint-pair shuffle always runs on the passed order, so assumeSorted
    // never affects the shuffle. assumeSorted is INERT only on SORTED input: on
    // UNSORTED input passing assumeSorted=true is undefined behavior — exactly
    // like every other estimator — because the sparity (spread>0) check runs
    // spreadImpl(x, assumeSorted), feeding unsorted data to a sorted-only kernel
    // (it may hit the iteration cap and ERROR, or pass only by luck). So the
    // fair true-vs-false comparison is on a genuinely SORTED array, where both
    // flag values must agree.
    it('spreadBounds: assumeSorted=true === assumeSorted=false on a SORTED array', () => {
      const t = spreadBounds(xs, misrate, seed, true);
      const f = spreadBounds(xs, misrate, seed, false);
      expectIdentical(t, f);
    });

    it('disparityBounds: assumeSorted=true === assumeSorted=false on the same arrays', () => {
      const t = disparityBounds(xs, ys, misrate, seed, true);
      const f = disparityBounds(xs, ys, misrate, seed, false);
      expectIdentical(t, f);
    });
  });
});
