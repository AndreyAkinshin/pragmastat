/**
 * Pragmastat: Unified Statistical Toolkit
 *
 * A collection of robust statistical estimators for real-world data analysis.
 */

// Assumptions
export { AssumptionId, AssumptionError, type Subject, type Violation } from './assumptions';

// Metrology
export { MeasurementUnit } from './measurement-unit';
export { Measurement } from './measurement';
export { Sample } from './sample';
export { UnitRegistry } from './unit-registry';

// Estimators
export {
  DEFAULT_MISRATE,
  center,
  spread,
  shift,
  ratio,
  disparity,
  shiftBounds,
  ratioBounds,
  centerBounds,
  spreadBounds,
  disparityBounds,
  type Bounds,
} from './estimators';

export { Rng } from './rng';

export { type Distribution, Uniform, Additive, Multiplic, Exp, Power } from './distributions/index';
