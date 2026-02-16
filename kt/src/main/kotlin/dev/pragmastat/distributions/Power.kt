package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng
import kotlin.math.pow

/**
 * Power (Pareto) distribution with minimum value and shape parameter.
 *
 * Follows a power-law distribution where large values are rare but possible.
 * Historically called 'Pareto' distribution.
 *
 * @property min Minimum value (lower bound, > 0).
 * @property shape Shape parameter (alpha > 0, controls tail heaviness).
 * @throws IllegalArgumentException If min <= 0 or shape <= 0.
 */
class Power(private val min: Double, private val shape: Double) : Distribution {
    init {
        require(min > 0) { "min must be positive" }
        require(shape > 0) { "shape must be positive" }
    }

    override fun sample(rng: Rng): Double {
        // Inverse CDF method: min / (1 - U)^(1/shape)
        var u = rng.uniformDouble()
        // Avoid division by zero - use machine epsilon for cross-language consistency
        if (u == 1.0) {
            u = 1.0 - Constants.MACHINE_EPSILON
        }
        return min / (1.0 - u).pow(1.0 / shape)
    }
}
