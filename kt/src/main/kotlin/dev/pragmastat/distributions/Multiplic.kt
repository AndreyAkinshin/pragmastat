package dev.pragmastat.distributions

import dev.pragmastat.Rng
import dev.pragmastat.additiveCumulative
import dev.pragmastat.additiveQuantile
import dev.pragmastat.expFunction
import dev.pragmastat.logFunction

/**
 * Multiplicative (Log-Normal) distribution.
 *
 * The logarithm of samples follows an Additive (Normal) distribution.
 * Historically called 'Log-Normal' or 'Galton' distribution.
 *
 * @property logMean Mean of log values (location parameter).
 * @property logStdDev Standard deviation of log values (scale parameter).
 * @throws IllegalArgumentException If logMean is not finite, or if logStdDev is not finite
 * and positive.
 */
class Multiplic(
    private val logMean: Double,
    private val logStdDev: Double,
) : Distribution {
    init {
        require(logMean.isFinite()) { "logMean must be finite" }
        require(logStdDev.isFinite() && logStdDev > 0) { "logStdDev must be finite and positive" }
    }

    override fun sample(rng: Rng): Double = quantileAt(samplingCumulance(rng))

    /**
     * The support is `x > 0`, and that is the test. It used to be `x < 1e-9`, which is not the
     * edge of the support but an arbitrary point inside it: a `Multiplic(-30.0, 1.0)` has its
     * median at 9.4e-14, so that threshold reported zero density and zero probability across
     * essentially the whole distribution.
     */
    override fun densityAt(x: Double): Double {
        if (x <= 0.0) return 0.0
        val z = (logFunction(x) - logMean) / logStdDev
        return expFunction(-z * z / 2) / (x * logStdDev * SQRT_2PI)
    }

    /**
     * A lognormal variable's distribution function is the normal one applied to log x, so this
     * goes through [additiveCumulative] rather than spelling the same thing out through erf.
     */
    override fun cumulativeAt(x: Double): Double = if (x <= 0.0) 0.0 else additiveCumulative((logFunction(x) - logMean) / logStdDev)

    override fun quantileAt(p: Double): Double =
        when {
            p <= 0.0 -> 0.0
            p >= 1.0 -> Double.POSITIVE_INFINITY
            else -> expFunction(logMean + logStdDev * additiveQuantile(p))
        }
}
