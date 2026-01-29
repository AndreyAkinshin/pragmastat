package dev.pragmastat.distributions

import dev.pragmastat.Rng

/**
 * Uniform distribution on [min, max).
 *
 * All values within the interval have equal probability.
 *
 * @property min Lower bound (inclusive).
 * @property max Upper bound (exclusive).
 * @throws IllegalArgumentException If min >= max.
 */
class Uniform(private val min: Double, private val max: Double) : Distribution {
    init {
        require(min < max) { "min must be less than max" }
    }

    override fun sample(rng: Rng): Double {
        return min + rng.uniform() * (max - min)
    }
}
