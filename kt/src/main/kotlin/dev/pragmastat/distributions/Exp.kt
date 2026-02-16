package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng
import kotlin.math.ln

/**
 * Exponential distribution with given rate parameter.
 *
 * The mean of this distribution is 1/rate.
 * Naturally arises from memoryless processes.
 *
 * @property rate Rate parameter (lambda > 0).
 * @throws IllegalArgumentException If rate <= 0.
 */
class Exp(private val rate: Double) : Distribution {
    init {
        require(rate > 0) { "rate must be positive" }
    }

    override fun sample(rng: Rng): Double {
        // Inverse CDF method: -ln(1 - U) / rate
        var u = rng.uniformDouble()
        // Avoid log(0) - use machine epsilon for cross-language consistency
        if (u == 1.0) {
            u = 1.0 - Constants.MACHINE_EPSILON
        }
        return -ln(1.0 - u) / rate
    }
}
