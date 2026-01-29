package dev.pragmastat.distributions

import dev.pragmastat.Rng
import kotlin.math.exp

/**
 * Multiplicative (Log-Normal) distribution.
 *
 * The logarithm of samples follows an Additive (Normal) distribution.
 * Historically called 'Log-Normal' or 'Galton' distribution.
 *
 * @property logMean Mean of log values (location parameter).
 * @property logStdDev Standard deviation of log values (scale parameter).
 * @throws IllegalArgumentException If logStdDev <= 0.
 */
class Multiplic(private val logMean: Double, private val logStdDev: Double) : Distribution {
    private val additive: Additive

    init {
        require(logStdDev > 0) { "logStdDev must be positive" }
        additive = Additive(logMean, logStdDev)
    }

    override fun sample(rng: Rng): Double {
        return exp(additive.sample(rng))
    }
}
