import { MeasurementUnit } from '../src/measurement-unit';

/**
 * The constructor refuses malformed unit definitions. A non-positive baseUnits poisons every
 * conversion through the unit and makes finer prefer it over any well-formed unit; an empty
 * family makes two unrelated units compatible, because isCompatible compares families.
 * An empty abbreviation stays legal: the three dimensionless units have none.
 */
describe('MeasurementUnit construction', () => {
  it('rejects an empty id', () => {
    expect(() => new MeasurementUnit('', 'Time', 'ms', 'Millisecond', 1e6)).toThrow(
      'id cannot be empty',
    );
  });

  it('rejects an empty family', () => {
    expect(() => new MeasurementUnit('ms', '', 'ms', 'Millisecond', 1e6)).toThrow(
      'family cannot be empty',
    );
  });

  it('rejects zero baseUnits', () => {
    expect(() => new MeasurementUnit('ms', 'Time', 'ms', 'Millisecond', 0)).toThrow(
      'baseUnits must be positive',
    );
  });

  it('rejects negative baseUnits', () => {
    expect(() => new MeasurementUnit('ms', 'Time', 'ms', 'Millisecond', -1)).toThrow(
      'baseUnits must be positive',
    );
  });

  it('accepts a well-formed unit', () => {
    const ms = new MeasurementUnit('ms', 'Time', 'ms', 'Millisecond', 1e6);
    expect(ms.id).toBe('ms');
    expect(ms.family).toBe('Time');
    expect(ms.abbreviation).toBe('ms');
    expect(ms.fullName).toBe('Millisecond');
    expect(ms.baseUnits).toBe(1e6);
  });

  it('rejects a fractional baseUnits', () => {
    expect(() => new MeasurementUnit('half', 'Time', 'h', 'Half', 0.5)).toThrow('whole number');
  });

  it('rejects a baseUnits that is not a finite number', () => {
    // The positivity test lets both through: Infinity > 0, and NaN <= 0 is false.
    expect(() => new MeasurementUnit('inf', 'Time', 'i', 'Inf', Infinity)).toThrow('whole number');
    expect(() => new MeasurementUnit('nan', 'Time', 'n', 'NaN', NaN)).toThrow();
  });

  it('keeps the empty abbreviation of the dimensionless units', () => {
    const units: [string, MeasurementUnit][] = [
      ['number', MeasurementUnit.NUMBER],
      ['ratio', MeasurementUnit.RATIO],
      ['disparity', MeasurementUnit.DISPARITY],
    ];
    for (const [id, unit] of units) {
      expect(unit.id).toBe(id);
      expect(unit.abbreviation).toBe('');
      expect(unit.baseUnits).toBe(1);
    }
  });
});
