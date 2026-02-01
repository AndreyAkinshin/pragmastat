import { fastSpread } from './fastSpread';

/**
 * Assumption validation framework for Pragmastat estimators.
 *
 * Assumption IDs (canonical priority order):
 *   1. validity - non-empty input with finite defined real values
 *   2. positivity - values must be strictly positive
 *   3. sparity - sample must be non tie-dominant (Spread > 0)
 *
 * When multiple assumptions are violated, the violation with highest priority
 * is reported. For two-sample functions, subject X is checked before Y.
 */

export const AssumptionId = {
  VALIDITY: 'validity',
  POSITIVITY: 'positivity',
  SPARITY: 'sparity',
} as const;

export type AssumptionId = (typeof AssumptionId)[keyof typeof AssumptionId];

export type Subject = 'x' | 'y';

export interface Violation {
  id: AssumptionId;
  subject: Subject;
}

export class AssumptionError extends Error {
  readonly violation: Violation;

  constructor(violation: Violation) {
    super(`${violation.id}(${violation.subject})`);
    this.name = 'AssumptionError';
    this.violation = violation;
  }

  static validity(_functionName: string, subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.VALIDITY, subject });
  }

  static positivity(_functionName: string, subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.POSITIVITY, subject });
  }

  static sparity(_functionName: string, subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.SPARITY, subject });
  }
}

export function checkValidity(values: number[], subject: Subject, functionName: string): void {
  if (values.length === 0) {
    throw AssumptionError.validity(functionName, subject);
  }
  if (!values.every((v) => Number.isFinite(v))) {
    throw AssumptionError.validity(functionName, subject);
  }
}

export function checkPositivity(values: number[], subject: Subject, functionName: string): void {
  if (values.some((v) => v <= 0)) {
    throw AssumptionError.positivity(functionName, subject);
  }
}

export function checkSparity(values: number[], subject: Subject, functionName: string): void {
  if (values.length < 2) {
    throw AssumptionError.sparity(functionName, subject);
  }
  const spread = fastSpread(values);
  if (spread <= 0) {
    throw AssumptionError.sparity(functionName, subject);
  }
}
