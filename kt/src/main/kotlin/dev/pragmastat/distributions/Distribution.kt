package dev.pragmastat.distributions

import dev.pragmastat.Rng

/**
 * Base interface for distributions that can generate samples.
 */
interface Distribution {
    /**
     * Generate a single sample from this distribution.
     */
    fun sample(rng: Rng): Double

    /**
     * Generate multiple samples from this distribution.
     */
    fun samples(rng: Rng, count: Int): List<Double> {
        return (0 until count).map { sample(rng) }
    }
}
