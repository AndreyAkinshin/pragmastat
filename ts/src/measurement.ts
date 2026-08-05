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
}
