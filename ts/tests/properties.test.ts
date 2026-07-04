/**
 * Coverage for three estimator properties:
 *   - Sample-path bounds re-attach Ratio/Disparity units, while the RAW
 *     (native-array) bounds stay unitless (Number). Center/Spread/Shift Sample
 *     units propagate from x / finer(x, y).
 *   - The RAW (double-misrate) bounds API rejects out-of-[0,1]/NaN misrate with
 *     a domain/misrate AssumptionError.
 *   - The n==2 center midpoint (0.5*a + 0.5*b) is exactly order-symmetric:
 *     center([-5,-1.8]) bit-equals center([-1.8,-5]).
 */

import {
  center,
  centerBounds,
  spreadBounds,
  shiftBounds,
  ratioBounds,
  disparityBounds,
} from '../src/estimators';
import { AssumptionError } from '../src/assumptions';
import { MeasurementUnit } from '../src/measurement-unit';
import { Sample } from '../src/sample';

// A non-default unit to prove center/spread/shift propagate x's (finer) unit
// rather than hard-coding Number.
const SEC = new MeasurementUnit('sec', 'Time', 's', 'Second', 1);

describe('bounds unit re-attachment', () => {
  // Larger samples so the shuffle-based spread/disparity bounds clear their
  // min-achievable-misrate floors at the misrate below.
  const x = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
  const y = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
  const sx = Sample.of(x);
  const sy = Sample.of(y);
  const misrate = 0.5;

  it('ratioBounds(Sample, Sample).unit === Ratio', () => {
    expect(ratioBounds(sx, sy, misrate).unit).toBe(MeasurementUnit.RATIO);
  });

  it('disparityBounds(Sample, Sample).unit === Disparity', () => {
    expect(disparityBounds(sx, sy, misrate).unit).toBe(MeasurementUnit.DISPARITY);
  });

  it('RAW ratioBounds is unitless (Number)', () => {
    expect(ratioBounds(x, y, misrate).unit).toBe(MeasurementUnit.NUMBER);
  });

  it('RAW disparityBounds is unitless (Number)', () => {
    expect(disparityBounds(x, y, misrate).unit).toBe(MeasurementUnit.NUMBER);
  });

  it('centerBounds(Sample).unit propagates x.unit', () => {
    const sxSec = Sample.withUnit(x, SEC);
    expect(centerBounds(sxSec, misrate).unit).toBe(SEC);
  });

  it('spreadBounds(Sample).unit propagates x.unit', () => {
    const sxSec = Sample.withUnit(x, SEC);
    expect(spreadBounds(sxSec, misrate).unit).toBe(SEC);
  });

  it('shiftBounds(Sample, Sample).unit propagates finer(x, y)', () => {
    const sxSec = Sample.withUnit(x, SEC);
    const sySec = Sample.withUnit(y, SEC);
    expect(shiftBounds(sxSec, sySec, misrate).unit).toBe(SEC);
  });

  it('RAW centerBounds / spreadBounds are unitless (Number)', () => {
    expect(centerBounds(x, misrate).unit).toBe(MeasurementUnit.NUMBER);
    expect(spreadBounds(x, misrate).unit).toBe(MeasurementUnit.NUMBER);
  });

  it('RAW shiftBounds is unitless (Number)', () => {
    expect(shiftBounds(x, y, misrate).unit).toBe(MeasurementUnit.NUMBER);
  });
});

describe('RAW bounds reject out-of-domain misrate', () => {
  const x = [1, 2, 3, 4, 5];
  const y = [2, 3, 4, 5, 6];

  function expectDomainMisrate(fn: () => unknown): void {
    let thrown: AssumptionError | null = null;
    try {
      fn();
    } catch (e) {
      if (e instanceof AssumptionError) thrown = e;
    }
    expect(thrown).not.toBeNull();
    expect(thrown!.violation!.id).toBe('domain');
    expect(thrown!.violation!.subject).toBe('misrate');
  }

  // One-sample (centerBounds) via the RAW/double-misrate path.
  it('centerBounds rejects misrate = 2.0', () => {
    expectDomainMisrate(() => centerBounds(x, 2.0));
  });

  it('centerBounds rejects misrate = -0.1', () => {
    expectDomainMisrate(() => centerBounds(x, -0.1));
  });

  it('centerBounds rejects misrate = NaN', () => {
    expectDomainMisrate(() => centerBounds(x, NaN));
  });

  // Two-sample (shiftBounds) via the RAW/double-misrate path.
  it('shiftBounds rejects misrate = 2.0', () => {
    expectDomainMisrate(() => shiftBounds(x, y, 2.0));
  });

  it('shiftBounds rejects misrate = -0.1', () => {
    expectDomainMisrate(() => shiftBounds(x, y, -0.1));
  });

  it('shiftBounds rejects misrate = NaN', () => {
    expectDomainMisrate(() => shiftBounds(x, y, NaN));
  });
});

describe('n==2 center midpoint is order-symmetric', () => {
  // centerImpl handles n==2 with an early return that runs BEFORE the
  // normalizing sort, so both argument orders feed the 0.5*a + 0.5*b midpoint
  // as-is. That formula is exactly order-symmetric: each operand is halved
  // independently, and the final addition commutes. An alternative midpoint
  // such as a + (b - a) * 0.5 is NOT: it yields -3.4 for [-5,-1.8] but
  // -3.4000000000000004 for [-1.8,-5]. Hence exact equality below, not
  // approximate.
  it('center([-5,-1.8]) bit-equals center([-1.8,-5])', () => {
    const forward = center([-5.0, -1.8]);
    const reversed = center([-1.8, -5.0]);
    expect(reversed).toBe(forward);
    expect(forward).toBe(-3.4);
  });
});
