/**
 * Measured queries against a built distribution reconstruction model.
 *
 * Only quantile carries a unit -- the sample's own. cumulative answers in probabilities and
 * density is a terminal value read, plotted, or compared against another density of the same
 * model, never converted or combined, so both come back as plain numbers.
 */

import { Measurement } from '../measurement';

export interface DistMeasuredView {
  /** Values at the requested cumulances, in the unit of the sample. */
  quantile(cumulances: readonly number[]): Measurement[];

  /** Probabilities of not exceeding the requested measurements. */
  cumulative(values: readonly Measurement[]): number[];

  /** Densities at the requested measurements, as plain numbers. */
  density(values: readonly Measurement[]): number[];
}
