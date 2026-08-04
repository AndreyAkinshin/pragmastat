import { Rng } from '../src/rng';
import { Additive, Distribution, Exp, Multiplic, Power, Uniform } from '../src/distributions';

/**
 * The five parametric families refuse malformed parameters at construction. Every parameter has to
 * be finite, and every scale or shape parameter has to be positive.
 *
 * Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
 * location constructed a family whose every draw is NaN. And the predicate was written negatively
 * (stdDev <= 0), which is not the negation of stdDev > 0 under IEEE 754: NaN <= 0 is false, so NaN
 * walked past the guard. The predicates are positive now, which refuses NaN without a branch of
 * its own.
 */
describe('distribution parameter validation', () => {
  it.each([NaN, -Infinity, Infinity])('rejects a Uniform min of %p', (min) => {
    expect(() => new Uniform(min, 1)).toThrow('min must be finite');
  });

  it.each([NaN, -Infinity, Infinity])('rejects a Uniform max of %p', (max) => {
    expect(() => new Uniform(0, max)).toThrow('max must be finite');
  });

  it.each([
    [1, 1],
    [2, 1],
  ])('rejects the empty Uniform interval [%p, %p)', (min, max) => {
    expect(() => new Uniform(min, max)).toThrow('min must be less than max');
  });

  it.each([NaN, Infinity, -Infinity])('rejects an Additive mean of %p', (mean) => {
    expect(() => new Additive(mean, 1)).toThrow('mean must be finite');
  });

  it.each([NaN, Infinity, 0, -1])('rejects an Additive stdDev of %p', (stdDev) => {
    expect(() => new Additive(0, stdDev)).toThrow('stdDev must be finite and positive');
  });

  it.each([NaN, Infinity, -Infinity])('rejects a Multiplic logMean of %p', (logMean) => {
    expect(() => new Multiplic(logMean, 1)).toThrow('logMean must be finite');
  });

  it.each([NaN, Infinity, 0, -1])('rejects a Multiplic logStdDev of %p', (logStdDev) => {
    expect(() => new Multiplic(0, logStdDev)).toThrow('logStdDev must be finite and positive');
  });

  it.each([NaN, Infinity, 0, -1])('rejects an Exp rate of %p', (rate) => {
    expect(() => new Exp(rate)).toThrow('rate must be finite and positive');
  });

  it.each([NaN, Infinity, 0, -1])('rejects a Power min of %p', (min) => {
    expect(() => new Power(min, 3)).toThrow('min must be finite and positive');
  });

  it.each([NaN, Infinity, 0, -1])('rejects a Power shape of %p', (shape) => {
    expect(() => new Power(1, shape)).toThrow('shape must be finite and positive');
  });

  it('lets a well-formed family construct and draw', () => {
    const rng = new Rng('distribution-parameters');
    const families: Distribution[] = [
      new Uniform(-2.5, 7.5),
      new Additive(10, 2),
      new Multiplic(1, 0.5),
      new Exp(3),
      new Power(1, 2),
    ];
    for (const family of families) {
      expect(Number.isFinite(family.sample(rng))).toBe(true);
    }
  });
});
