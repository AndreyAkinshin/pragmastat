package dev.pragmastat.distributions

import dev.pragmastat.Rng

/**
 * Uniform distribution on [min, max).
 *
 * All values within the interval have equal probability.
 *
 * @property min Lower bound (inclusive).
 * @property max Upper bound (exclusive).
 * @throws IllegalArgumentException If min or max is not finite, or if min is not less than max.
 */
class Uniform(
    private val min: Double,
    private val max: Double,
) : Distribution {
    init {
        require(min.isFinite()) { "min must be finite" }
        require(max.isFinite()) { "max must be finite" }
        require(min < max) { "min must be less than max" }
    }

    override fun sample(rng: Rng): Double = quantileAt(rng.uniformDouble())

    /**
     * The support is half-open, so the density is zero at [max] and positive at [min]. This is the
     * one family assembled from arithmetic alone: no primitive of the toolkit appears on any of
     * its three paths.
     */
    override fun densityAt(x: Double): Double =
        // NaN is refused before the bounds are compared. This is the only one of the fifteen
        // functions whose result does not depend on its argument, so a NaN reaching the comparison
        // satisfies neither half of it and falls through to an ordinary density.
        when {
            x.isNaN() -> x
            x < min || x >= max -> 0.0
            else -> 1.0 / (max - min)
        }

    override fun cumulativeAt(x: Double): Double =
        when {
            x < min -> 0.0
            x >= max -> 1.0
            else -> (x - min) / (max - min)
        }

    override fun quantileAt(p: Double): Double =
        when {
            p <= 0.0 -> min
            p >= 1.0 -> max
            else -> min + p * (max - min)
        }
}
