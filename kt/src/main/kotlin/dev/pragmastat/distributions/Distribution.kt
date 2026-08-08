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
     * Probability density at [x]. Zero outside the support.
     */
    fun densityAt(x: Double): Double

    /**
     * Probability of not exceeding [x]. Zero below the support, one above it.
     */
    fun cumulativeAt(x: Double): Double

    /**
     * The value below which a fraction [p] of the distribution lies.
     *
     * A [p] outside `(0, 1)` returns the endpoints of the support, which are infinities for the
     * unbounded families.
     */
    fun quantileAt(p: Double): Double

    /**
     * Generate multiple samples from this distribution.
     */
    fun samples(
        rng: Rng,
        count: Int,
    ): List<Double> = (0 until count).map { sample(rng) }
}
