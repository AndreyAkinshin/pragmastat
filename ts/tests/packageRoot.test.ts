import * as pragmastat from '../src';

/**
 * `export { type Bounds }` publishes the name and emits no binding, so
 * `require('pragmastat').Bounds` was undefined: `new Bounds(...)` threw, and `instanceof Bounds`
 * threw before it could answer. Reading a Bounds an estimator handed back worked, which is why
 * nothing here saw it. Every test imported the class from '../src/estimators' and none from the
 * package root, so the one path a consumer takes was the one path nothing exercised.
 *
 * These assertions run against the export point rather than the module a class is defined in,
 * because that distinction is the whole defect.
 */
describe('the package root publishes values, not only names', () => {
  const constructible = [
    'Bounds',
    'Measurement',
    'MeasurementUnit',
    'Sample',
    'Threshold',
    'Projection',
    'Rng',
    'Uniform',
    'Additive',
    'Multiplic',
    'Exp',
    'Power',
    'AssumptionError',
  ] as const;

  it.each(constructible)('%s is a value at run time', (name) => {
    expect(typeof (pragmastat as Record<string, unknown>)[name]).toBe('function');
  });

  it('a Bounds from an estimator is an instance of the exported class', () => {
    const sample = pragmastat.Sample.of(Array.from({ length: 200 }, (_, i) => i + 1));
    const bounds = pragmastat.centerBounds(sample, 1e-3);

    expect(bounds).toBeInstanceOf(pragmastat.Bounds);
    expect(new pragmastat.Bounds(1, 2, pragmastat.MeasurementUnit.NUMBER).lower).toBe(1);
  });

  /**
   * The other type-only exports are correct and stay that way: an interface has no run-time form
   * to publish, so a name with no binding is what it should be.
   */
  it.each(['Violation', 'Distribution'])('%s is type-only, as an interface must be', (name) => {
    expect((pragmastat as Record<string, unknown>)[name]).toBeUndefined();
  });
});
