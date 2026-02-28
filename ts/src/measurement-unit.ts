/**
 * Represents a unit of measurement with identity, family, and conversion support.
 */
export class MeasurementUnit {
  constructor(
    readonly id: string,
    readonly family: string,
    readonly abbreviation: string,
    readonly fullName: string,
    readonly baseUnits: number,
  ) {}

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
