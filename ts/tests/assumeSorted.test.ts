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

  describe('order-independent estimators: sorted+true === unsorted+false', () => {
    it('center', () => {
      expect(center(xs, true)).toBeCloseTo(center(x, false), 9);
    });

    it('spread', () => {
      expect(spread(xs, true)).toBeCloseTo(spread(x, false), 9);
    });

    it('shift', () => {
      expect(shift(xs, ys, true)).toBeCloseTo(shift(x, y, false), 9);
    });

    it('ratio', () => {
      expect(ratio(xs, ys, true)).toBeCloseTo(ratio(x, y, false), 9);
    });

    it('disparity', () => {
      expect(disparity(xs, ys, true)).toBeCloseTo(disparity(x, y, false), 9);
    });

    it('centerBounds', () => {
      const a = centerBounds(xs, misrate, true);
      const b = centerBounds(x, misrate, false);
      expect(a.lower).toBeCloseTo(b.lower, 9);
      expect(a.upper).toBeCloseTo(b.upper, 9);
    });

    it('shiftBounds', () => {
      const a = shiftBounds(xs, ys, misrate, true);
      const b = shiftBounds(x, y, misrate, false);
      expect(a.lower).toBeCloseTo(b.lower, 9);
      expect(a.upper).toBeCloseTo(b.upper, 9);
    });

    it('ratioBounds', () => {
      const a = ratioBounds(xs, ys, misrate, true);
      const b = ratioBounds(x, y, misrate, false);
      expect(a.lower).toBeCloseTo(b.lower, 9);
      expect(a.upper).toBeCloseTo(b.upper, 9);
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
