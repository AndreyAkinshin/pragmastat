package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng
import dev.pragmastat.expFunction
import dev.pragmastat.logFunction
import kotlin.math.ln

/**
 * Exponential distribution with given rate parameter.
 *
 * The mean of this distribution is 1/rate.
 * Naturally arises from memoryless processes.
 *
 * @property rate Rate parameter (lambda > 0).
 * @throws IllegalArgumentException If rate is not finite and positive.
 */
class Exp(
    private val rate: Double,
) : Distribution {
    init {
        require(rate.isFinite() && rate > 0) { "rate must be finite and positive" }
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

    override fun densityAt(x: Double): Double = if (x < 0.0) 0.0 else rate * expFunction(-rate * x)

    override fun cumulativeAt(x: Double): Double = if (x < 0.0) 0.0 else 1.0 - expFunction(-rate * x)

    override fun quantileAt(p: Double): Double =
        when {
            p <= 0.0 -> 0.0
            p >= 1.0 -> Double.POSITIVE_INFINITY
            else -> -logFunction(1.0 - p) / rate
        }
}
