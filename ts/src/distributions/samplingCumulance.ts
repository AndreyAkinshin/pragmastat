/**
 * The cumulance Additive and Multiplic consume, the two families whose quantileAt(0) is not a
 * sample: Additive returns negative infinity there, Multiplic returns the zero its own support
 * excludes. The other three families' quantileAt(0) is the finite value their support already
 * includes, so they draw uniformFloat() directly and never call this.
 *
 * A drawn 0 is lifted to the smallest positive subnormal. uniformFloat() never returns 1, so the
 * other endpoint needs no guard.
 */

import { SMALLEST_POSITIVE_SUBNORMAL } from '../constants';
import { Rng } from '../rng';

export function samplingCumulance(rng: Rng): number {
  const u = rng.uniformFloat();
  return u === 0 ? SMALLEST_POSITIVE_SUBNORMAL : u;
}
