/**
 * Compare1 and Compare2: confirmatory analysis for one-sample and two-sample estimators.
 *
 * These high-level APIs compare estimates (Center, Spread, Shift, Ratio, Disparity)
 * against practical thresholds and return verdicts (Less, Greater, or Inconclusive).
 */

import { Measurement } from './measurement';
import { MeasurementUnit } from './measurement-unit';
import { Sample, checkNonWeighted, checkCompatibleUnits, convertToFiner } from './sample';
import { AssumptionError } from './assumptions';
import {
  center,
  spread,
  shift,
  ratio,
  disparity,
  centerBounds,
  spreadBounds,
  shiftBounds,
  ratioBounds,
  disparityBounds,
  Bounds,
} from './estimators';

/**
 * Metric types supported by Compare1 and Compare2.
 */
export enum Metric {
  Center = 'center',
  Spread = 'spread',
  Shift = 'shift',
  Ratio = 'ratio',
  Disparity = 'disparity',
}

/**
 * Verdict from comparing an estimate against a threshold.
 */
export enum ComparisonVerdict {
  Less = 'less',
  Greater = 'greater',
  Inconclusive = 'inconclusive',
}

/**
 * A threshold value with a metric type and misrate for comparison.
 */
export class Threshold {
  constructor(
    readonly metric: Metric,
    readonly value: Measurement,
    readonly misrate: number,
  ) {
    if (!Number.isFinite(misrate) || misrate <= 0.0 || misrate > 1.0) {
      throw AssumptionError.domain('misrate');
    }
    if (!Number.isFinite(value.value)) {
      throw new Error('threshold value must be finite');
    }
  }
}

/**
 * A projection containing estimate, bounds, and verdict for a single threshold.
 */
export class Projection {
  constructor(
    readonly threshold: Threshold,
    readonly estimate: Measurement,
    readonly bounds: Bounds,
    readonly verdict: ComparisonVerdict,
  ) {}
}

/** Function type for validating and normalizing threshold values. */
type ValidateAndNormalizeFn = (threshold: Threshold, x: Sample, y: Sample | null) => Measurement;

/** Function type for computing point estimates. */
type EstimateFn = (x: Sample, y: Sample | null) => Measurement;

/** Function type for computing bounds. */
type BoundsFn = (x: Sample, y: Sample | null, misrate: number, seed?: string) => Bounds;

/** Specification for a metric's validation, estimation, and bounds computation. */
interface MetricSpec {
  metric: Metric;
  validateAndNormalize: ValidateAndNormalizeFn;
  estimate: EstimateFn;
  bounds: BoundsFn;
  seededBounds?: BoundsFn;
}

/** Specification for Compare2 metrics (validation only — estimation is done directly). */
interface Compare2MetricSpec {
  metric: Metric;
  validateAndNormalize: ValidateAndNormalizeFn;
}

/** Validates and normalizes a Center threshold. */
function validateCenter(threshold: Threshold, x: Sample, _y: Sample | null): Measurement {
  if (!threshold.value.unit.isCompatible(x.unit)) {
    throw new AssumptionError(
      `can't convert ${threshold.value.unit.fullName} to ${x.unit.fullName}`,
    );
  }
  const factor = MeasurementUnit.conversionFactor(threshold.value.unit, x.unit);
  return new Measurement(threshold.value.value * factor, x.unit);
}

/** Validates and normalizes a Spread threshold. */
function validateSpread(threshold: Threshold, x: Sample, _y: Sample | null): Measurement {
  return validateCenter(threshold, x, null); // Same validation as Center
}

/** Validates and normalizes a Shift threshold. */
function validateShift(threshold: Threshold, x: Sample, y: Sample | null): Measurement {
  if (!y) {
    throw new Error('Shift requires y sample');
  }
  if (!threshold.value.unit.isCompatible(x.unit)) {
    throw new AssumptionError(
      `can't convert ${threshold.value.unit.fullName} to ${x.unit.fullName}`,
    );
  }
  const target = MeasurementUnit.finer(x.unit, y.unit);
  const factor = MeasurementUnit.conversionFactor(threshold.value.unit, target);
  return new Measurement(threshold.value.value * factor, target);
}

/** Validates and normalizes a Ratio threshold. */
function validateRatio(threshold: Threshold, _x: Sample, _y: Sample | null): Measurement {
  const unit = threshold.value.unit;
  if (unit.id !== 'ratio' && unit.id !== 'number') {
    throw new AssumptionError(`can't convert ${unit.fullName} to Ratio`);
  }
  const value = threshold.value.value;
  if (value <= 0.0) {
    throw new Error('Ratio threshold value must be positive');
  }
  return new Measurement(value, MeasurementUnit.RATIO);
}

/** Validates and normalizes a Disparity threshold. */
function validateDisparity(threshold: Threshold, _x: Sample, _y: Sample | null): Measurement {
  const unit = threshold.value.unit;
  if (unit.id !== 'disparity' && unit.id !== 'number') {
    throw new AssumptionError(`can't convert ${unit.fullName} to Disparity`);
  }
  return new Measurement(threshold.value.value, MeasurementUnit.DISPARITY);
}

/** Compare1 metric specifications. */
const compare1Specs: MetricSpec[] = [
  {
    metric: Metric.Center,
    validateAndNormalize: validateCenter,
    estimate: (x, _y) => center(x),
    bounds: (x, _y, misrate, _seed?) => centerBounds(x, misrate),
  },
  {
    metric: Metric.Spread,
    validateAndNormalize: validateSpread,
    estimate: (x, _y) => spread(x),
    bounds: (x, _y, misrate, seed?) => spreadBounds(x, misrate, seed),
    seededBounds: (x, _y, misrate, seed?) => spreadBounds(x, misrate, seed),
  },
];

/** Compare2 metric specifications. */
const compare2Specs: Compare2MetricSpec[] = [
  { metric: Metric.Shift, validateAndNormalize: validateShift },
  { metric: Metric.Ratio, validateAndNormalize: validateRatio },
  { metric: Metric.Disparity, validateAndNormalize: validateDisparity },
];

/** Computes the verdict by comparing bounds against a threshold value. */
function computeVerdict(bounds: Bounds, thresholdValue: number): ComparisonVerdict {
  if (bounds.lower > thresholdValue) {
    return ComparisonVerdict.Greater;
  }
  if (bounds.upper < thresholdValue) {
    return ComparisonVerdict.Less;
  }
  return ComparisonVerdict.Inconclusive;
}

/** Gets the spec for a given metric from the spec array. */
function getSpec<T extends { metric: Metric }>(specs: T[], metric: Metric): T | undefined {
  return specs.find((s) => s.metric === metric);
}

/** Executes Compare1 logic. */
function executeCompare1(
  x: Sample,
  thresholds: Threshold[],
  normalizedValues: Measurement[],
  seed?: string,
): Projection[] {
  const results: (Projection | null)[] = new Array(thresholds.length).fill(null);

  // Group thresholds by metric
  const byMetric = new Map<
    Metric,
    Array<{ index: number; threshold: Threshold; normalizedValue: Measurement }>
  >();

  for (let i = 0; i < thresholds.length; i++) {
    const metric = thresholds[i].metric;
    if (!byMetric.has(metric)) {
      byMetric.set(metric, []);
    }
    byMetric.get(metric)!.push({
      index: i,
      threshold: thresholds[i],
      normalizedValue: normalizedValues[i],
    });
  }

  // Process each metric spec
  for (const spec of compare1Specs) {
    const entries = byMetric.get(spec.metric);
    if (!entries || entries.length === 0) {
      continue;
    }

    // Compute estimate once per metric
    const estimate = spec.estimate(x, null);

    // Compute bounds and verdict for each threshold of this metric
    for (const entry of entries) {
      const bounds =
        seed && spec.seededBounds
          ? spec.seededBounds(x, null, entry.threshold.misrate, seed)
          : spec.bounds(x, null, entry.threshold.misrate);

      const verdict = computeVerdict(bounds, entry.normalizedValue.value);
      results[entry.index] = new Projection(entry.threshold, estimate, bounds, verdict);
    }
  }

  return results.filter((r): r is Projection => r !== null);
}

/** Executes Compare2 logic. */
function executeCompare2(
  x: Sample,
  y: Sample,
  thresholds: Threshold[],
  normalizedValues: Measurement[],
  seed?: string,
): Projection[] {
  const results: (Projection | null)[] = new Array(thresholds.length).fill(null);

  // Convert both samples to the finer unit
  const [cx, cy] = convertToFiner(x, y);

  // Group thresholds by metric
  const byMetric = new Map<
    Metric,
    Array<{ index: number; threshold: Threshold; normalizedValue: Measurement }>
  >();

  for (let i = 0; i < thresholds.length; i++) {
    const metric = thresholds[i].metric;
    if (!byMetric.has(metric)) {
      byMetric.set(metric, []);
    }
    byMetric.get(metric)!.push({
      index: i,
      threshold: thresholds[i],
      normalizedValue: normalizedValues[i],
    });
  }

  // Process Shift thresholds
  const shiftEntries = byMetric.get(Metric.Shift);
  if (shiftEntries && shiftEntries.length > 0) {
    const estimate = shift(cx, cy);
    for (const entry of shiftEntries) {
      const bounds = shiftBounds(cx, cy, entry.threshold.misrate);
      const verdict = computeVerdict(bounds, entry.normalizedValue.value);
      results[entry.index] = new Projection(entry.threshold, estimate, bounds, verdict);
    }
  }

  // Process Ratio thresholds
  const ratioEntries = byMetric.get(Metric.Ratio);
  if (ratioEntries && ratioEntries.length > 0) {
    const estimate = ratio(cx, cy);
    for (const entry of ratioEntries) {
      const bounds = ratioBounds(cx, cy, entry.threshold.misrate);
      const verdict = computeVerdict(bounds, entry.normalizedValue.value);
      results[entry.index] = new Projection(entry.threshold, estimate, bounds, verdict);
    }
  }

  // Process Disparity thresholds
  const disparityEntries = byMetric.get(Metric.Disparity);
  if (disparityEntries && disparityEntries.length > 0) {
    const estimate = disparity(cx, cy);
    for (const entry of disparityEntries) {
      const bounds = disparityBounds(cx, cy, entry.threshold.misrate, seed);
      const verdict = computeVerdict(bounds, entry.normalizedValue.value);
      results[entry.index] = new Projection(entry.threshold, estimate, bounds, verdict);
    }
  }

  return results.filter((r): r is Projection => r !== null);
}

/**
 * One-sample confirmatory analysis: compares Center/Spread against practical thresholds.
 *
 * @param x The sample to analyze
 * @param thresholds List of thresholds to compare against
 * @returns List of projections in the same order as the input thresholds
 * @throws Error if thresholds is empty or contains unsupported metrics
 * @throws AssumptionError if validation fails
 */
export function compare1(x: Sample, thresholds: Threshold[]): Projection[];
export function compare1(x: Sample, thresholds: Threshold[], seed: string): Projection[];
export function compare1(x: Sample, thresholds: Threshold[], seed?: string): Projection[] {
  checkNonWeighted('x', x);

  if (thresholds.length === 0) {
    throw new Error('thresholds list cannot be empty');
  }

  for (const threshold of thresholds) {
    if (threshold.metric !== Metric.Center && threshold.metric !== Metric.Spread) {
      throw new Error(
        `Metric ${threshold.metric} is not supported by Compare1. Use Compare2 instead.`,
      );
    }
  }

  const normalizedValues: Measurement[] = [];
  for (const threshold of thresholds) {
    const spec = getSpec(compare1Specs, threshold.metric);
    if (!spec) {
      throw new Error(`No spec found for metric ${threshold.metric}`);
    }
    const normalized = spec.validateAndNormalize(threshold, x, null);
    normalizedValues.push(normalized);
  }

  return executeCompare1(x, thresholds, normalizedValues, seed);
}

/**
 * Two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.
 *
 * @param x The first sample
 * @param y The second sample
 * @param thresholds List of thresholds to compare against
 * @returns List of projections in the same order as the input thresholds
 * @throws Error if thresholds is empty or contains unsupported metrics
 * @throws AssumptionError if validation fails
 */
export function compare2(x: Sample, y: Sample, thresholds: Threshold[]): Projection[];
export function compare2(x: Sample, y: Sample, thresholds: Threshold[], seed: string): Projection[];
export function compare2(
  x: Sample,
  y: Sample,
  thresholds: Threshold[],
  seed?: string,
): Projection[] {
  checkNonWeighted('x', x);
  checkNonWeighted('y', y);
  checkCompatibleUnits(x, y);

  if (thresholds.length === 0) {
    throw new Error('thresholds list cannot be empty');
  }

  for (const threshold of thresholds) {
    if (
      threshold.metric !== Metric.Shift &&
      threshold.metric !== Metric.Ratio &&
      threshold.metric !== Metric.Disparity
    ) {
      throw new Error(
        `Metric ${threshold.metric} is not supported by Compare2. Use Compare1 instead.`,
      );
    }
  }

  // Normalize threshold values
  const normalizedValues: Measurement[] = [];
  for (const threshold of thresholds) {
    const spec = getSpec(compare2Specs, threshold.metric);
    if (!spec) {
      throw new Error(`No spec found for metric ${threshold.metric}`);
    }
    const normalized = spec.validateAndNormalize(threshold, x, y);
    normalizedValues.push(normalized);
  }

  return executeCompare2(x, y, thresholds, normalizedValues, seed);
}
