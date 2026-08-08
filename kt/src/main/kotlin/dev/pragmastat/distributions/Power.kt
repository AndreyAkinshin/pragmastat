package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng
import dev.pragmastat.expFunction
import dev.pragmastat.logFunction
import kotlin.math.pow

/**
 * Power (Pareto) distribution with minimum value and shape parameter.
 *
 * Follows a power-law distribution where large values are rare but possible.
 * Historically called 'Pareto' distribution.
 *
 * @property min Minimum value (lower bound, > 0).
 * @property shape Shape parameter (alpha > 0, controls tail heaviness).
 * @throws IllegalArgumentException If min or shape is not finite and positive.
 */
class Power(
    private val min: Double,
    private val shape: Double,
) : Distribution {
    init {
        require(min.isFinite() && min > 0) { "min must be finite and positive" }
        require(shape.isFinite() && shape > 0) { "shape must be finite and positive" }
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

    /**
     * `shape * min^shape / x^(shape+1)`, rearranged as
     * `(shape / x) * exp(shape * (log min - log x))`. The toolkit owns an exponential and a
     * logarithm and owns no general power, so a real exponent is spelled as the exponential of a
     * logarithm. This family is the reason the logarithm had to exist before the trio could.
     *
     * The support is `[min, infinity)`, closed at the left, so the density at [min] is
     * `shape / min` rather than zero, while the cumulative there is zero. Uniform reads the same
     * way at its own lower endpoint.
     */
    override fun densityAt(x: Double): Double = if (x < min) 0.0 else (shape / x) * expFunction(shape * (logFunction(min) - logFunction(x)))

    override fun cumulativeAt(x: Double): Double = if (x <= min) 0.0 else 1.0 - expFunction(shape * (logFunction(min) - logFunction(x)))

    override fun quantileAt(p: Double): Double =
        when {
            p <= 0.0 -> min
            p >= 1.0 -> Double.POSITIVE_INFINITY
            else -> min * expFunction(-logFunction(1.0 - p) / shape)
        }
}
