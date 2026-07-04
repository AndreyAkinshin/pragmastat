import { centerImpl } from '../src/centerImpl';
import { center, Bounds } from '../src/estimators';
import { MeasurementUnit } from '../src/measurement-unit';

/**
 * centerImpl's Monahan-selection loop must terminate even when
 * `assumeSorted = true` is misused on UNSORTED input. Without an iteration cap
 * the loop can spin forever, wedging the process. The cap turns that into a
 * deterministic, fast "convergence failure" error (a plain Error, NOT an
 * AssumptionError — this is UB misuse, not an assumption violation).
 */
describe('centerImpl convergence guard', () => {
  // Adversarial: strongly anti-sorted input passed with assumeSorted=true.
  const unsorted = [9, 1, 8, 2, 7, 3, 6, 4, 5, 0, 10, 100, 50, 25, 75, 12, 88, 33, 66, 99];

  it('raises a (non-hanging) convergence error on unsorted input with assumeSorted=true', () => {
    const start = Date.now();
    let threw = false;
    try {
      centerImpl(unsorted, true);
    } catch (e) {
      threw = true;
      expect((e as Error).message).toMatch(/[Cc]onvergence failure/);
    }
    // It must FAIL FAST: either it raised the convergence error, or it returned
    // a (meaningless but finite) value quickly. Either way it must not hang.
    const elapsedMs = Date.now() - start;
    expect(elapsedMs).toBeLessThan(2000);
    // For this adversarial input the guard fires.
    expect(threw).toBe(true);
  });

  it('still converges correctly on valid sorted input (guard never triggers)', () => {
    const sorted = [...unsorted].sort((a, b) => a - b);
    // assumeSorted=true on genuinely sorted input must equal the unsorted path.
    expect(centerImpl(sorted, true)).toBeCloseTo(center(unsorted, false), 9);
  });
});

/**
 * Minimal coverage for Bounds.contains (the lower <= v <= upper predicate),
 * so the method is exercised rather than left dead.
 */
describe('Bounds.contains', () => {
  const b = new Bounds(1, 3, MeasurementUnit.NUMBER);

  it('includes both endpoints (inclusive interval)', () => {
    expect(b.contains(1)).toBe(true);
    expect(b.contains(3)).toBe(true);
  });

  it('includes an interior point', () => {
    expect(b.contains(2)).toBe(true);
  });

  it('excludes points outside the interval', () => {
    expect(b.contains(0.999)).toBe(false);
    expect(b.contains(3.001)).toBe(false);
  });
});
