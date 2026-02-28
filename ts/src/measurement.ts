/**
 * Represents a numeric value paired with a measurement unit.
 */

import { MeasurementUnit } from './measurement-unit';

export class Measurement {
  constructor(
    readonly value: number,
    readonly unit: MeasurementUnit,
  ) {}
}
