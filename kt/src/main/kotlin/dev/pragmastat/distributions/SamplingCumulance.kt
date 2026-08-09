package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng

/**
 * The cumulance Additive and Multiplic consume, the two families whose `quantileAt(0)` is not a
 * sample: Additive returns negative infinity there, Multiplic returns the zero its own support
 * excludes. The other three families' `quantileAt(0)` is the finite value their support already
 * includes, so they draw [Rng.uniformDouble] directly and never call this.
 *
 * A drawn 0 is lifted to the smallest positive subnormal. [Rng.uniformDouble] never returns 1, so
 * the other endpoint needs no guard.
 */
internal fun samplingCumulance(rng: Rng): Double {
    val u = rng.uniformDouble()
    return if (u == 0.0) Constants.SMALLEST_POSITIVE_SUBNORMAL else u
}
