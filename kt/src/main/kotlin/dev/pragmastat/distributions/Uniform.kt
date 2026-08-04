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

    override fun sample(rng: Rng): Double = min + rng.uniformDouble() * (max - min)
}
