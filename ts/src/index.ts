/**
 * Pragmastat: Unified Statistical Toolkit
 *
 * A collection of robust statistical estimators for real-world data analysis.
 */

// Assumptions
export { AssumptionId, AssumptionError, type Subject, type Violation } from './assumptions';

// Estimators
export {
  DEFAULT_MISRATE,
  center,
  spread,
  relSpread,
  shift,
  ratio,
  avgSpread,
  disparity,
  shiftBounds,
  ratioBounds,
  centerBounds,
  type Bounds,
} from './estimators';

export { Rng } from './rng';

export { type Distribution, Uniform, Additive, Multiplic, Exp, Power } from './distributions/index';
