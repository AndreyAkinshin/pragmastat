import { Rng } from '../src/rng';
import { LinearDistModel } from '../src/distributions';
import { Sample } from '../src/sample';
import { MeasurementUnit } from '../src/measurement-unit';
import { Measurement } from '../src/measurement';
import { AssumptionError, AssumptionId, Subject } from '../src/assumptions';
import { expectBitwise } from './bitwise';

/**
 * Every expected value here is computed by hand against the model's own definition: piecewise
 * linear interpolation of the sorted sample at anchors (i-1)/(n-1), so the checks double as a
 * derivation, not just a pin.
 */
const LINEAR_X = [1.0, 2.0, 4.0, 8.0];

function expectViolation(fn: () => unknown, id: AssumptionId, subject: Subject): void {
  let thrown: AssumptionError | null = null;
  try {
    fn();
  } catch (e) {
    if (e instanceof AssumptionError) thrown = e;
  }
  expect(thrown).not.toBeNull();
  expect(thrown!.violation!.id).toBe(id);
  expect(thrown!.violation!.subject).toBe(subject);
}

describe('LinearDistModel', () => {
  it('lands on each observation at its anchor', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.quantileAt(0)).toBe(1.0);
    expect(dist.quantileAt(1.0 / 3.0)).toBe(2.0);
    expect(dist.quantileAt(2.0 / 3.0)).toBe(4.0);
    expect(dist.quantileAt(1.0)).toBe(8.0);
  });

  it('interpolates between two observations', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.quantileAt(0.5)).toBe(3.0);
  });

  it('clamps outside the unit interval', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.quantileAt(-0.1)).toBe(1.0);
    expect(dist.quantileAt(1.1)).toBe(8.0);
  });

  it('cumulativeAt inverts quantileAt', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.cumulativeAt(3.0)).toBe(0.5);
    expect(dist.cumulativeAt(2.0)).toBe(1.0 / 3.0);
    expect(dist.cumulativeAt(4.0)).toBe(2.0 / 3.0);
  });

  it('cumulativeAt is zero below and one above the support', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.cumulativeAt(1.0)).toBe(0.0);
    expect(dist.cumulativeAt(-5.0)).toBe(0.0);
    expect(dist.cumulativeAt(8.0)).toBe(1.0);
    expect(dist.cumulativeAt(20.0)).toBe(1.0);
  });

  it('densityAt is mass over width inside each cell', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.densityAt(1.5)).toBe(1.0 / 3.0);
    expect(dist.densityAt(3.0)).toBe(1.0 / 6.0);
    expect(dist.densityAt(6.0)).toBe(1.0 / 12.0);
    expect(dist.densityAt(8.0)).toBe(1.0 / 12.0); // The right edge belongs to the last cell.
  });

  it('densityAt is zero outside the support', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.densityAt(0.5)).toBe(0.0);
    expect(dist.densityAt(9.0)).toBe(0.0);
  });

  it('propagates NaN on every scalar query', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(Number.isNaN(dist.quantileAt(NaN))).toBe(true);
    expect(Number.isNaN(dist.cumulativeAt(NaN))).toBe(true);
    expect(Number.isNaN(dist.densityAt(NaN))).toBe(true);
  });

  it('agrees with the scalar queries bit for bit in its landscape', () => {
    const dist = new LinearDistModel(LINEAR_X);
    const landscape = dist.getLandscape();

    expect(landscape.boundaries).toEqual(LINEAR_X);
    expect(landscape.cellCount).toBe(3);
    for (let i = 0; i < LINEAR_X.length; i++) {
      expectBitwise(`cumulative[${i}]`, dist.cumulativeAt(LINEAR_X[i]), landscape.cumulative[i]);
    }
    for (let j = 0; j < landscape.cellCount; j++) {
      const midpoint = (LINEAR_X[j] + LINEAR_X[j + 1]) / 2;
      expectBitwise(`density[${j}]`, dist.densityAt(midpoint), landscape.densities[j]);
    }
  });

  it('draws a sample as quantileAt of a raw drawn cumulance', () => {
    const dist = new LinearDistModel(LINEAR_X);
    const drawing = new Rng('identity-seed');
    const inverting = new Rng('identity-seed');
    for (let i = 0; i < 100; i++) {
      const drawn = dist.sample(drawing);
      const u = inverting.uniformFloat();
      expectBitwise(`sample ${i}`, drawn, dist.quantileAt(u));
    }
  });

  it('rejects fewer than two observations', () => {
    expectViolation(() => new LinearDistModel([1.0]), AssumptionId.DOMAIN, Subject.X);
  });

  it('rejects a repeated value', () => {
    expectViolation(
      () => new LinearDistModel([1.0, 1.0, 2.0]),
      AssumptionId.DISTINCTNESS,
      Subject.X,
    );
  });

  it('rejects a cumulance outside the unit interval', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expectViolation(() => dist.quantile([1.5]), AssumptionId.CUMULANCE, Subject.Y);
  });

  it('does not treat an empty query vector as a violation', () => {
    const dist = new LinearDistModel(LINEAR_X);
    expect(dist.quantile([])).toEqual([]);
    expect(dist.cumulative([])).toEqual([]);
    expect(dist.density([])).toEqual([]);
  });

  it('takes the unit of the sample it was built from', () => {
    const meter = new MeasurementUnit('meter', 'Length', 'm', 'Meter', 1);
    const sample = new Sample(LINEAR_X, null, meter);
    const dist = new LinearDistModel(sample);
    expect(dist.unit).toBe(meter);
  });

  it('refuses a weighted sample', () => {
    const sample = Sample.weighted([1.0, 2.0], [1.0, 2.0]);
    expect(() => new LinearDistModel(sample)).toThrow();
  });

  it('returns a measured quantile in the model unit', () => {
    const meter = new MeasurementUnit('meter', 'Length', 'm', 'Meter', 1);
    const dist = new LinearDistModel(new Sample(LINEAR_X, null, meter));
    const result = dist.measured.quantile([0.5]);
    expect(result[0].value).toBe(3.0);
    expect(result[0].unit).toBe(meter);
  });

  it('returns a measured density as a plain number', () => {
    const meter = new MeasurementUnit('meter', 'Length', 'm', 'Meter', 1);
    const dist = new LinearDistModel(new Sample(LINEAR_X, null, meter));
    const result = dist.measured.density([new Measurement(3.0, meter)]);
    expect(result[0]).toBe(1.0 / 6.0);
  });
});
