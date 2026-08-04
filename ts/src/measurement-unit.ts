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
