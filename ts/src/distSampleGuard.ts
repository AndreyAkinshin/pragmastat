/**
 * Shared sample preparation for the distribution estimation models.
 *
 * Runs the two assumptions that follow validity in the canonical order: domain(x) (at least two
 * observations and a finite representable range) and distinctness(x) (no repeated value; -0 and
 * +0 count as repeated). The caller's array is never retained: the model always owns a private
 * copy.
 */

import { AssumptionError, Subject, checkDistinctness } from './assumptions';
import { Measurement } from './measurement';
import { MeasurementUnit } from './measurement-unit';

export function distPrepare(values: readonly number[], assumeSorted: boolean): number[] {
  const copy = [...values];
  if (!assumeSorted) {
    copy.sort((a, b) => a - b);
  }

  const n = copy.length;
  if (n < 2) {
    throw AssumptionError.domain(Subject.X);
  }

  const range = copy[n - 1] - copy[0];
  if (!Number.isFinite(range)) {
    throw AssumptionError.domain(Subject.X);
  }

  checkDistinctness(copy, Subject.X);

  return copy;
}

/** Attaches a unit to every element of a query result. */
export function distToMeasurements(
  values: readonly number[],
  unit: MeasurementUnit,
): Measurement[] {
  return values.map((v) => new Measurement(v, unit));
}

/**
 * Converts a measured query vector into the model's own unit.
 *
 * A query in a compatible but different unit is converted rather than rejected, matching the
 * two-sample estimators, which reconcile their inputs instead of demanding an exact match. An
 * incompatible unit is a caller error and is reported as one.
 */
export function distToRaw(values: readonly Measurement[], unit: MeasurementUnit): number[] {
  return values.map((m) => {
    if (!m.unit.isCompatible(unit)) {
      throw new AssumptionError(`can't convert ${m.unit.fullName} to ${unit.fullName}`);
    }
    return m.value * MeasurementUnit.conversionFactor(m.unit, unit);
  });
}
