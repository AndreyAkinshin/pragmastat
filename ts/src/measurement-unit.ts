/**
 * Represents a unit of measurement with identity, family, and conversion support.
 *
 * An empty abbreviation is legal: the three dimensionless units have none.
 */
export class MeasurementUnit {
  readonly id: string;
  readonly family: string;
  readonly abbreviation: string;
  readonly fullName: string;
  readonly baseUnits: number;

  constructor(
    id: string,
    family: string,
    abbreviation: string,
    fullName: string,
    baseUnits: number,
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
    this.id = id;
    this.family = family;
    this.abbreviation = abbreviation;
    this.fullName = fullName;
    this.baseUnits = baseUnits;
    // readonly is a type annotation and survives into no emitted JavaScript, so it protects a
    // consumer compiling against these types and nobody else. Measured on the built package
    // before this line existed: MeasurementUnit.NUMBER.baseUnits = 5 succeeded, and the next
    // conversionFactor(NUMBER, RATIO) answered 5 where it had answered 1. A unit is never
    // revalidated after construction, so nothing could have detected it.
    Object.freeze(this);
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

// Freezing an instance does not protect the binding that names it, and the three shared units are
// reached through bindings: MeasurementUnit.NUMBER = other is a plain property write on the class
// object and rescales the whole session by the other door. These three are read by every
// Measurement, every Sample and every conversion, which is why the class object is frozen and the
// rule stays "freeze the statics the library itself depends on" rather than every static there is.
Object.freeze(MeasurementUnit);
