/**
 * Represents a numeric value paired with a measurement unit.
 */

import { MeasurementUnit } from './measurement-unit';

export class Measurement {
  readonly value: number;
  readonly unit: MeasurementUnit;

  constructor(value: number, unit: MeasurementUnit = MeasurementUnit.NUMBER) {
    this.value = value;
    this.unit = unit;
  }

  /**
   * Renders the value, followed by the unit's abbreviation when it has one. This is the form
   * every other port prints, and the digits match for the same reason Bounds' do.
   */
  toString(): string {
    return this.unit.abbreviation.length > 0
      ? `${this.value} ${this.unit.abbreviation}`
      : `${this.value}`;
  }
}
