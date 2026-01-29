package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng
import kotlin.math.*

/**
 * Additive (Normal/Gaussian) distribution with given mean and standard deviation.
 *
 * Uses the Box-Muller transform to generate samples.
 * Historically called 'Normal' or 'Gaussian' distribution.
 *
 * @property mean Location parameter (center of the distribution).
 * @property stdDev Scale parameter (standard deviation).
 * @throws IllegalArgumentException If stdDev <= 0.
 */
class Additive(private val mean: Double, private val stdDev: Double) : Distribution {
    init {
        require(stdDev > 0) { "stdDev must be positive" }
    }

    override fun sample(rng: Rng): Double {
        // Box-Muller transform
        var u1 = rng.uniform()
        val u2 = rng.uniform()

        // Avoid log(0) - use smallest positive subnormal for cross-language consistency
        if (u1 == 0.0) {
            u1 = Constants.SMALLEST_POSITIVE_SUBNORMAL
        }

        val r = sqrt(-2.0 * ln(u1))
        val theta = 2.0 * PI * u2

        // Use the first of the two Box-Muller outputs
        val z = r * cos(theta)

        return mean + z * stdDev
    }
}
