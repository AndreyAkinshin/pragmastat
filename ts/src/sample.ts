/**
 * Sample wraps values with optional weights and a measurement unit.
 */

import { MeasurementUnit } from './measurement-unit';
import { AssumptionError } from './assumptions';
export class Sample {
  readonly values: readonly number[];
  readonly weights: readonly number[] | null;
  readonly unit: MeasurementUnit;
  readonly isWeighted: boolean;
  readonly totalWeight: number;
  readonly weightedSize: number;

  private _sortedValues: number[] | null = null;

  constructor(values: readonly number[], weights: readonly number[] | null, unit: MeasurementUnit) {
    if (values.length === 0) {
      throw AssumptionError.validity('x');
    }
    for (const v of values) {
      if (!Number.isFinite(v)) {
        throw AssumptionError.validity('x');
      }
    }

    this.values = values;
    this.unit = unit;

    if (weights !== null) {
      if (weights.length !== values.length) {
        throw new Error(
          `weights length (${weights.length}) must match values length (${values.length})`,
        );
      }
      let totalWeight = 0;
      let totalWeightSq = 0;
      for (const w of weights) {
        if (w < 0) {
          throw new Error('all weights must be non-negative');
        }
        totalWeight += w;
        totalWeightSq += w * w;
      }
      if (totalWeight < 1e-9) {
        throw new Error('total weight must be positive');
      }
      this.weights = weights;
      this.isWeighted = true;
      this.totalWeight = totalWeight;
      this.weightedSize = (totalWeight * totalWeight) / totalWeightSq;
    } else {
      this.weights = null;
      this.isWeighted = false;
      this.totalWeight = 1.0;
      this.weightedSize = values.length;
    }
  }

  /** Number of values in the sample. */
  get size(): number {
    return this.values.length;
  }

  /** Returns a sorted copy of the values (lazily computed). */
  get sortedValues(): number[] {
    if (this._sortedValues === null) {
      this._sortedValues = [...this.values].sort((a, b) => a - b);
    }
    return this._sortedValues;
  }

  /** Creates an unweighted sample from numeric values. */
  static of(values: number[]): Sample {
    return new Sample(values, null, MeasurementUnit.NUMBER);
  }

  /** Creates an unweighted sample with a specified unit. */
  static withUnit(values: number[], unit: MeasurementUnit): Sample {
    return new Sample(values, null, unit);
  }

  /** Creates a weighted sample with an optional unit. */
  static weighted(
    values: number[],
    weights: number[],
    unit: MeasurementUnit = MeasurementUnit.NUMBER,
  ): Sample {
    return new Sample(values, weights, unit);
  }

  /** Converts the sample to a different (compatible) unit. */
  convertTo(target: MeasurementUnit): Sample {
    if (!this.unit.isCompatible(target)) {
      throw new AssumptionError(`can't convert ${this.unit.fullName} to ${target.fullName}`);
    }
    if (this.unit === target) {
      return this;
    }
    const factor = MeasurementUnit.conversionFactor(this.unit, target);
    const converted = this.values.map((v) => v * factor);
    return new Sample(converted, this.weights !== null ? [...this.weights] : null, target);
  }

  /** Returns a new sample with log-transformed values and NumberUnit. */
  log(): Sample {
    const logValues = new Array<number>(this.values.length);
    for (let i = 0; i < this.values.length; i++) {
      if (this.values[i] <= 0) {
        throw AssumptionError.positivity('x');
      }
      logValues[i] = Math.log(this.values[i]);
    }
    return new Sample(
      logValues,
      this.weights !== null ? [...this.weights] : null,
      MeasurementUnit.NUMBER,
    );
  }
}

/** Checks that a sample is not weighted; throws AssumptionError if it is. */
export function checkNonWeighted(s: Sample): void {
  if (s.isWeighted) {
    throw new AssumptionError('weighted samples are not supported');
  }
}

/** Checks that two samples have compatible units; throws AssumptionError if not. */
export function checkCompatibleUnits(a: Sample, b: Sample): void {
  if (!a.unit.isCompatible(b.unit)) {
    throw new AssumptionError(`can't convert ${a.unit.fullName} to ${b.unit.fullName}`);
  }
}

/** Converts both samples to their finer unit. */
export function convertToFiner(a: Sample, b: Sample): [Sample, Sample] {
  if (a.unit === b.unit) {
    return [a, b];
  }
  const target = MeasurementUnit.finer(a.unit, b.unit);
  return [a.convertTo(target), b.convertTo(target)];
}
