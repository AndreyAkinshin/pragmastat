/**
 * Represents a unit of measurement with identity, family, and conversion support.
 *
 * An empty abbreviation is legal: the three dimensionless units have none.
 */
export class MeasurementUnit {
  constructor(
    readonly id: string,
    readonly family: string,
    readonly abbreviation: string,
    readonly fullName: string,
    readonly baseUnits: number,
  ) {
    // A non-positive baseUnits poisons every conversion through this unit, and it makes finer
    // pick it over any well-formed one. An empty family makes unrelated units compatible.
    if (id.length === 0) {
      throw new Error('id cannot be empty');
    }
    if (family.length === 0) {
      throw new Error('family cannot be empty');
    }
    if (baseUnits <= 0) {
      throw new Error('baseUnits must be positive');
    }
    // TypeScript has one numeric type, so nothing above stops a fractional scale. The four ports
    // that type this as a 64-bit integer cannot express one at all, so a unit that constructs here
    // and nowhere else breaks the promise that one program reads the same in every port.
    // Number.isInteger also refuses Infinity and NaN, which the positivity test lets through.
    if (!Number.isInteger(baseUnits)) {
      throw new Error('baseUnits must be a whole number');
    }
  }

  /** Returns true if both units belong to the same family. */
  isCompatible(other: MeasurementUnit): boolean {
    return this.family === other.family;
  }

  /** Returns the unit with smaller baseUnits (higher precision). */
  static finer(a: MeasurementUnit, b: MeasurementUnit): MeasurementUnit {
    return a.baseUnits <= b.baseUnits ? a : b;
  }

  /** Returns the multiplier to convert from one unit to another. */
  static conversionFactor(from: MeasurementUnit, to: MeasurementUnit): number {
    return from.baseUnits / to.baseUnits;
  }

  static readonly NUMBER = new MeasurementUnit('number', 'Number', '', 'Number', 1);
  static readonly RATIO = new MeasurementUnit('ratio', 'Ratio', '', 'Ratio', 1);
  static readonly DISPARITY = new MeasurementUnit('disparity', 'Disparity', '', 'Disparity', 1);
}
