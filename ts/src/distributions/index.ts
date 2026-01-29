/**
 * Statistical distributions for sampling.
 *
 * This module provides five distributions for generating random samples:
 * - Uniform: uniform distribution on a bounded interval
 * - Additive: normal (Gaussian) distribution
 * - Multiplic: log-normal distribution
 * - Exp: exponential distribution
 * - Power: Pareto (power-law) distribution
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
