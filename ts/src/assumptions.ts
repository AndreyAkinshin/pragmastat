import { fastSpread } from './fastSpread';

/**
 * Assumption validation framework for Pragmastat estimators.
 *
 * Assumption IDs (canonical priority order):
 *   1. validity - non-empty input with finite defined real values
 *   2. domain - parameter is outside its valid domain
 *   3. positivity - values must be strictly positive
 *   4. sparity - sample must be non tie-dominant (Spread > 0)
 *
 * When multiple assumptions are violated, the violation with highest priority
 * is reported. For two-sample functions, subject X is checked before Y.
 */

export const AssumptionId = {
  VALIDITY: 'validity',
  DOMAIN: 'domain',
  POSITIVITY: 'positivity',
  SPARITY: 'sparity',
} as const;

export type AssumptionId = (typeof AssumptionId)[keyof typeof AssumptionId];

export type Subject = 'x' | 'y' | 'misrate';

export interface Violation {
  id: AssumptionId;
  subject: Subject;
}

export class AssumptionError extends Error {
  readonly violation: Violation;

  constructor(violation: Violation) {
    const violationStr = `${violation.id}(${violation.subject})`;
    super(violationStr);
    this.name = 'AssumptionError';
    this.violation = violation;
  }

  static validity(subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.VALIDITY, subject });
  }

  static positivity(subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.POSITIVITY, subject });
  }

  static sparity(subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.SPARITY, subject });
  }

  static domain(subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.DOMAIN, subject });
  }
}

export function checkValidity(values: number[], subject: Subject): void {
  if (values.length === 0) {
    throw AssumptionError.validity(subject);
  }
  if (!values.every((v) => Number.isFinite(v))) {
    throw AssumptionError.validity(subject);
  }
}

export function checkPositivity(values: number[], subject: Subject): void {
  if (values.some((v) => v <= 0)) {
    throw AssumptionError.positivity(subject);
  }
}

export function checkSparity(values: number[], subject: Subject): void {
  if (values.length < 2) {
    throw AssumptionError.sparity(subject);
  }
  const spread = fastSpread(values);
  if (spread <= 0) {
    throw AssumptionError.sparity(subject);
  }
}

/**
 * Log-transforms an array. Throws AssumptionError if any value is non-positive.
 */
export function log(values: number[], subject: Subject): number[] {
  const result = new Array<number>(values.length);
  for (let i = 0; i < values.length; i++) {
    if (values[i] <= 0) {
      throw AssumptionError.positivity(subject);
    }
    result[i] = Math.log(values[i]);
  }
  return result;
}
