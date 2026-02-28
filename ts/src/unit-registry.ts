/**
 * Registry for measurement units, enabling lookup by ID.
 */

import { MeasurementUnit } from './measurement-unit';

export class UnitRegistry {
  private readonly byId = new Map<string, MeasurementUnit>();

  /** Adds a unit to the registry. Throws if ID is already registered. */
  register(unit: MeasurementUnit): void {
    if (this.byId.has(unit.id)) {
      throw new Error(`unit with id '${unit.id}' is already registered`);
    }
    this.byId.set(unit.id, unit);
  }

  /** Looks up a unit by ID. Throws if not found. */
  resolve(id: string): MeasurementUnit {
    const unit = this.byId.get(id);
    if (unit === undefined) {
      throw new Error(`unknown unit id: '${id}'`);
    }
    return unit;
  }

  /** Returns a registry pre-populated with Number, Ratio, and Disparity units. */
  static standard(): UnitRegistry {
    const r = new UnitRegistry();
    r.register(MeasurementUnit.NUMBER);
    r.register(MeasurementUnit.RATIO);
    r.register(MeasurementUnit.DISPARITY);
    return r;
  }
}
