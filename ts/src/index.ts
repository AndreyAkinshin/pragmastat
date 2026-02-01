/**
 * Pragmastat: Unified Statistical Toolkit
 *
 * A collection of robust statistical estimators for real-world data analysis.
 */

// Assumptions
export { AssumptionId, AssumptionError, type Subject, type Violation } from './assumptions';

// Estimators
export {
  center,
  spread,
  relSpread,
  shift,
  ratio,
  avgSpread,
  disparity,
  shiftBounds,
  type Bounds,
} from './estimators';

export { median } from './utils';

export { pairwiseMargin } from './pairwiseMargin';

export { Rng } from './rng';

export { type Distribution, Uniform, Additive, Multiplic, Exp, Power } from './distributions/index';
