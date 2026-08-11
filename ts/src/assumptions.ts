/**
 * Assumption validation framework for Pragmastat estimators.
 *
 * Assumption IDs (canonical priority order):
 *   1. validity - non-empty input with finite defined real values
 *   2. domain - parameter is outside its valid domain
 *   3. positivity - values must be strictly positive
 *   4. sparity / distinctness - share a priority level: no estimator checks both
 *      - sparity - sample must be non tie-dominant (Spread > 0)
 *      - distinctness - sample must hold no repeated value
 *   5. cumulance - a query value must lie in the closed unit interval [0, 1]
 *
 * When multiple assumptions are violated, the violation with highest priority
 * is reported. For two-sample functions, subject X is checked before Y.
 */

import { logFunction } from './logFunction';

export const AssumptionId = {
  VALIDITY: 'validity',
  DOMAIN: 'domain',
  POSITIVITY: 'positivity',
  SPARITY: 'sparity',
  DISTINCTNESS: 'distinctness',
  CUMULANCE: 'cumulance',
} as const;

export type AssumptionId = (typeof AssumptionId)[keyof typeof AssumptionId];

export const Subject = {
  X: 'x',
  Y: 'y',
  MISRATE: 'misrate',
} as const;

export type Subject = (typeof Subject)[keyof typeof Subject];

export interface Violation {
  id: AssumptionId;
  subject: Subject;
}

export class AssumptionError extends Error {
  readonly violation: Violation | null;

  constructor(violationOrMessage: Violation | string) {
    if (typeof violationOrMessage === 'string') {
      super(violationOrMessage);
      this.violation = null;
    } else {
      const violationStr = `${violationOrMessage.id}(${violationOrMessage.subject})`;
      super(violationStr);
      this.violation = violationOrMessage;
    }
    this.name = 'AssumptionError';
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

  static distinctness(subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.DISTINCTNESS, subject });
  }

  static cumulance(subject: Subject): AssumptionError {
    return new AssumptionError({ id: AssumptionId.CUMULANCE, subject });
  }
}

export function checkValidity(values: readonly number[], subject: Subject): void {
  if (values.length === 0) {
    throw AssumptionError.validity(subject);
  }
  if (!values.every((v) => Number.isFinite(v))) {
    throw AssumptionError.validity(subject);
  }
}

export function checkPositivity(values: readonly number[], subject: Subject): void {
  if (values.some((v) => v <= 0)) {
    throw AssumptionError.positivity(subject);
  }
}

/**
 * Checks the validity assumption for a query vector passed to a distribution model: any
 * non-finite element is a violation. Unlike checkValidity, an empty vector is not a violation
 * here, since a query vector is questions asked of a model, and asking none is not malformed
 * input.
 */
export function checkQueryValidity(values: readonly number[], subject: Subject): void {
  if (!values.every((v) => Number.isFinite(v))) {
    throw AssumptionError.validity(subject);
  }
}

/**
 * Checks that every value in a query vector lies in the closed unit interval [0, 1], the domain
 * a cumulance (a quantile's probability argument) is defined on.
 */
export function checkCumulance(values: readonly number[], subject: Subject): void {
  if (values.some((v) => !(v >= 0 && v <= 1))) {
    throw AssumptionError.cumulance(subject);
  }
}

/**
 * Checks that a sorted collection holds no repeated value. -0 and +0 compare equal, so a sample
 * holding both is rejected the same as any other duplicate.
 */
export function checkDistinctness(sortedValues: readonly number[], subject: Subject): void {
  for (let i = 1; i < sortedValues.length; i++) {
    if (sortedValues[i - 1] === sortedValues[i]) {
      throw AssumptionError.distinctness(subject);
    }
  }
}

/**
 * Log-transforms an array. Throws AssumptionError if any value is non-positive.
 */
export function log(values: readonly number[], subject: Subject): number[] {
  const result = new Array<number>(values.length);
  for (let i = 0; i < values.length; i++) {
    if (values[i] <= 0) {
      throw AssumptionError.positivity(subject);
    }
    result[i] = logFunction(values[i]);
  }
  return result;
}
