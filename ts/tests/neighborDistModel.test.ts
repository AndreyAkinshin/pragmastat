import { Rng } from '../src/rng';
import { NeighborDistModel } from '../src/distributions';
import { Sample } from '../src/sample';
import { MeasurementUnit } from '../src/measurement-unit';
import { Measurement } from '../src/measurement';
import { AssumptionError, AssumptionId, Subject } from '../src/assumptions';
import { distNextDown, distNextUp } from '../src/distNumerics';
import { expectBitwise } from './bitwise';

const NEIGHBOR_X = [0.0, 1.0, 2.0, 4.0, 8.0];

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

describe('NeighborDistModel', () => {
  it('supports the min and max observation', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.supportMin).toBe(0.0);
    expect(dist.supportMax).toBe(8.0);
  });

  it('has at most as many cells as observations', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.cellCount).toBe(NEIGHBOR_X.length);
  });

  it('is zero and one exactly at the support', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.cumulativeAt(dist.supportMin)).toBe(0.0);
    expect(dist.cumulativeAt(dist.supportMax)).toBe(1.0);
  });

  it('sums cell masses to exactly one', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    const landscape = dist.getLandscape();
    const sum = landscape.masses.reduce((acc, m) => acc + m, 0);
    expect(sum).toBe(1.0);
  });

  it('is non-decreasing across the boundaries', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    const cumulative = dist.getLandscape().cumulative;
    for (let i = 1; i < cumulative.length; i++) {
      expect(cumulative[i]).toBeGreaterThan(cumulative[i - 1]);
    }
  });

  it('inverts cumulativeAt at each boundary', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    const landscape = dist.getLandscape();
    for (let i = 0; i < landscape.boundaries.length; i++) {
      expectBitwise(
        `quantileAt(cumulative[${i}])`,
        dist.quantileAt(landscape.cumulative[i]),
        landscape.boundaries[i],
      );
    }
  });

  it('clamps outside the unit interval', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.quantileAt(-0.1)).toBe(dist.supportMin);
    expect(dist.quantileAt(1.1)).toBe(dist.supportMax);
  });

  it('is zero outside the support', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.densityAt(dist.supportMin - 1)).toBe(0.0);
    expect(dist.densityAt(dist.supportMax + 1)).toBe(0.0);
  });

  it('propagates NaN on every scalar query', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(Number.isNaN(dist.quantileAt(NaN))).toBe(true);
    expect(Number.isNaN(dist.cumulativeAt(NaN))).toBe(true);
    expect(Number.isNaN(dist.densityAt(NaN))).toBe(true);
  });

  it('agrees with the scalar queries bit for bit in its landscape', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    const landscape = dist.getLandscape();
    for (let i = 0; i < landscape.boundaries.length; i++) {
      expectBitwise(
        `cumulativeAt(boundaries[${i}])`,
        dist.cumulativeAt(landscape.boundaries[i]),
        landscape.cumulative[i],
      );
    }
    for (let j = 0; j < landscape.cellCount; j++) {
      const midpoint = (landscape.boundaries[j] + landscape.boundaries[j + 1]) / 2;
      expectBitwise(`densityAt(midpoint ${j})`, dist.densityAt(midpoint), landscape.densities[j]);
    }
  });

  it('exposes scaleIndex and radius', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    const landscape = dist.getLandscape();
    // n = 5: distCeilSqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = floor(3 / 2) = 1.
    expect(landscape.scaleIndex).toBe(3);
    expect(landscape.radius).toBe(1);
  });

  it('maps each cell source to its observation', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.getLandscape().sources).toEqual([0, 1, 2, 3, 4]);
  });

  it('drops a zero-width cell from a collapsed midpoint', () => {
    // The center observation sits exactly one ULP from each neighbor on either side. Both
    // adjacent midpoints round to 1.0 itself under round-to-nearest-even (each is exactly halfway
    // between two representable doubles, and 1.0 has the even mantissa), so the center cell's two
    // boundaries collapse onto the same point and the cell is dropped.
    const center = 1.0;
    const below = distNextDown(center);
    const above = distNextUp(center);
    const values = [0.0, below, center, above, 10.0];

    const dist = new NeighborDistModel(values);
    const landscape = dist.getLandscape();

    expect(landscape.cellCount).toBe(values.length - 1);
    expect(landscape.sources).not.toContain(2);
  });

  it('draws a sample as quantileAt of a raw drawn cumulance', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    const drawing = new Rng('identity-seed');
    const inverting = new Rng('identity-seed');
    for (let i = 0; i < 100; i++) {
      const drawn = dist.sample(drawing);
      const u = inverting.uniformFloat();
      expectBitwise(`sample ${i}`, drawn, dist.quantileAt(u));
    }
  });

  it('rejects fewer than two observations', () => {
    expectViolation(() => new NeighborDistModel([1.0]), AssumptionId.DOMAIN, Subject.X);
  });

  it('rejects a repeated value', () => {
    expectViolation(
      () => new NeighborDistModel([1.0, 1.0, 2.0]),
      AssumptionId.DISTINCTNESS,
      Subject.X,
    );
  });

  it('rejects a cumulance outside the unit interval', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expectViolation(() => dist.quantile([-0.1]), AssumptionId.CUMULANCE, Subject.Y);
  });

  it('does not treat an empty query vector as a violation', () => {
    const dist = new NeighborDistModel(NEIGHBOR_X);
    expect(dist.quantile([])).toEqual([]);
    expect(dist.cumulative([])).toEqual([]);
    expect(dist.density([])).toEqual([]);
  });

  it('terminates on a broken assumeSorted promise', () => {
    // distPrepare's distinctness check only looks at ADJACENT elements of the (unsorted, since
    // assumeSorted is a promise it trusts) vector, so a non-adjacent repeat -- x[2] and x[6] both
    // 5.0 here -- slips through it. That repeat then makes one window's span exactly zero while
    // its cell keeps a nonzero width, so the cell's unnormalized mass divides by zero and the
    // total the whole reconstruction divides by is not a usable number. This is not a caller
    // input the model ever validates against -- it is reachable only through the broken
    // assumeSorted promise -- so it throws a plain Error, not an AssumptionError.
    const scrambled = [0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0];
    let thrown: unknown = null;
    try {
      new NeighborDistModel(scrambled, true);
    } catch (e) {
      thrown = e;
    }
    expect(thrown).not.toBeNull();
    expect(thrown).not.toBeInstanceOf(AssumptionError);
  });

  it('takes the unit of the sample it was built from', () => {
    const meter = new MeasurementUnit('meter', 'Length', 'm', 'Meter', 1);
    const dist = new NeighborDistModel(new Sample(NEIGHBOR_X, null, meter));
    expect(dist.unit).toBe(meter);
  });

  it('returns a measured density as a plain number', () => {
    const meter = new MeasurementUnit('meter', 'Length', 'm', 'Meter', 1);
    const dist = new NeighborDistModel(new Sample(NEIGHBOR_X, null, meter));
    const result = dist.measured.density([new Measurement(3.0, meter)]);
    expect(result[0]).toBe(dist.densityAt(3.0));
  });
});
