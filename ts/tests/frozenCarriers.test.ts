import {
  Measurement,
  MeasurementUnit,
  Metric,
  Projection,
  Threshold,
  centerBounds,
  Sample,
} from '../src';

/**
 * `readonly` is a type-system annotation and survives into no emitted JavaScript, so on its own it
 * protects exactly one kind of consumer: one compiling against these types. Most of npm is every
 * other consumer, reaching a bundled dist through require, from plain JavaScript or a REPL.
 *
 * Measured on the built package before the freezes existed: MeasurementUnit.NUMBER.baseUnits = 5
 * succeeded, and the next conversionFactor(NUMBER, RATIO) answered 5 where it had answered 1. Every
 * conversion in that process was then wrong, and nothing could detect it, since a unit is never
 * revalidated after construction.
 *
 * So every write here goes through a cast to a plain property bag, which is the exact shape of what
 * a JavaScript consumer writes. The compile-time half is not what is in doubt: a @ts-expect-error
 * on these lines passed before the freezes as well as after.
 *
 * Both halves of the outcome are asserted. Module code is strict mode, so a write to a frozen
 * object throws; a consumer in sloppy mode gets no throw and no write either, and it is the second
 * half that is the actual protection.
 */
type Bag = Record<string, unknown>;

function refusesWrite(label: string, target: object, key: string, value: unknown): void {
  const before = (target as Bag)[key];
  expect(() => {
    (target as Bag)[key] = value;
  }).toThrow(TypeError);
  expect((target as Bag)[key]).toBe(before);
  expect(label).toBeTruthy();
}

describe('carriers refuse mutation at run time', () => {
  it('MeasurementUnit instance', () => {
    refusesWrite('baseUnits', MeasurementUnit.NUMBER, 'baseUnits', 5);
    expect(MeasurementUnit.conversionFactor(MeasurementUnit.NUMBER, MeasurementUnit.RATIO)).toBe(1);
  });

  it('the class object holding the three shared units', () => {
    refusesWrite('NUMBER', MeasurementUnit, 'NUMBER', MeasurementUnit.RATIO);
    expect(MeasurementUnit.NUMBER.id).toBe('number');
  });

  it('Measurement', () => {
    refusesWrite('value', new Measurement(1), 'value', 42);
  });

  it('Bounds', () => {
    // The package root exports Bounds as a type and not as a value, so a consumer cannot construct
    // one; the only way to hold a Bounds is to be handed one. That is its own gap, and this test
    // reaches the carrier the way a consumer does rather than working around it.
    refusesWrite(
      'lower',
      centerBounds(Sample.of(Array.from({ length: 200 }, (_, i) => i + 1)), 1e-3),
      'lower',
      999,
    );
  });

  it('Threshold', () => {
    const threshold = new Threshold(Metric.CENTER, new Measurement(1), 1e-3);
    refusesWrite('misrate', threshold, 'misrate', 99);
  });

  it('Projection', () => {
    const sample = Sample.of(Array.from({ length: 200 }, (_, i) => i + 1));
    const projection: Projection = new Projection(
      new Threshold(Metric.CENTER, new Measurement(1), 1e-3),
      new Measurement(3),
      centerBounds(sample, 1e-3),
      'greater',
    );
    refusesWrite('verdict', projection, 'verdict', 'less');
  });
});
