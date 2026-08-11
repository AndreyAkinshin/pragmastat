/**
 * Statistical distributions for sampling.
 *
 * This module provides five parametric distributions for generating random samples:
 * - Uniform: uniform distribution on a bounded interval
 * - Additive: normal (Gaussian) distribution
 * - Multiplic: log-normal distribution
 * - Exp: exponential distribution
 * - Power: Pareto (power-law) distribution
 *
 * It also provides a model reconstructed from a sample rather than from parameters:
 * - LinearDistModel: piecewise-linear (Hyndman-Fan type 7) reconstruction
 *
 * All distributions produce identical sequences across all Pragmastat language
 * implementations when using the same seed.
 */

export { Distribution } from './distribution';
export { Uniform } from './uniform';
export { Additive } from './additive';
export { Multiplic } from './multiplic';
export { Exp } from './exp';
export { Power } from './power';
export { DistMeasuredView } from './distMeasuredView';
export { LinearDistModel } from './linearDistModel';
export { LinearLandscape } from './linearLandscape';
