import { spreadImpl } from '../src/spreadImpl';
import { spread } from '../src/estimators';

/**
 * spreadImpl's Monahan-selection loop must terminate even when
 * `assumeSorted = true` is misused on UNSORTED input. Without an iteration cap
 * the loop can spin forever, wedging the process. The cap turns that into a
 * deterministic, fast "convergence failure" error (a plain Error, NOT an
 * AssumptionError — this is UB misuse, not an assumption violation).
 * Parallel to the centerImpl guard test.
 */
describe('spreadImpl convergence guard', () => {
  // Adversarial: strongly anti-sorted input passed with assumeSorted=true.
  const unsorted = [9, 1, 8, 2, 7, 3, 6, 4, 5, 0, 10, 100, 50, 25, 75, 12, 88, 33, 66, 99];

  it('raises a (non-hanging) convergence error on unsorted input with assumeSorted=true', () => {
    const start = Date.now();
    let threw = false;
    try {
      spreadImpl(unsorted, true);
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

  it('also fails fast through the public spread(unsorted, assumeSorted=true) path', () => {
    const start = Date.now();
    expect(() => spread(unsorted, true)).toThrow(/[Cc]onvergence failure/);
    expect(Date.now() - start).toBeLessThan(2000);
  });

  it('still converges correctly on valid sorted input (guard never triggers)', () => {
    const sorted = [...unsorted].sort((a, b) => a - b);
    // assumeSorted=true on genuinely sorted input must equal the unsorted path.
    expect(spreadImpl(sorted, true)).toBeCloseTo(spread(unsorted, false) as number, 9);
  });
});
