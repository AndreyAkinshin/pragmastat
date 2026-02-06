/**
 * Pragmastat: Unified Statistical Toolkit
 *
 * A collection of robust statistical estimators for real-world data analysis.
 */

// Assumptions
export { AssumptionId, AssumptionError, type Subject, type Violation } from './assumptions';

// Estimators
export {
  median,
  center,
  spread,
  relSpread,
  shift,
  ratio,
  avgSpread,
  disparity,
  shiftBounds,
  ratioBounds,
  type Bounds,
} from './estimators';

export { pairwiseMargin } from './pairwiseMargin';
export { signedRankMargin } from './signedRankMargin';
export { minAchievableMisrateOneSample, minAchievableMisrateTwoSample } from './minMisrate';

export { Rng } from './rng';

export { type Distribution, Uniform, Additive, Multiplic, Exp, Power } from './distributions/index';
