package dev.pragmastat.distributions

import dev.pragmastat.Constants
import dev.pragmastat.Rng
import dev.pragmastat.additiveCumulative
import dev.pragmastat.additiveQuantile
import dev.pragmastat.expFunction
import kotlin.math.PI
import kotlin.math.cos
import kotlin.math.ln
import kotlin.math.sqrt

/**
 * Additive (Normal/Gaussian) distribution with given mean and standard deviation.
 *
 * Uses the Box-Muller transform to generate samples.
 * Historically called 'Normal' or 'Gaussian' distribution.
 *
 * @property mean Location parameter (center of the distribution).
 * @property stdDev Scale parameter (standard deviation).
 * @throws IllegalArgumentException If mean is not finite, or if stdDev is not finite and positive.
 */
class Additive(
    private val mean: Double,
    private val stdDev: Double,
) : Distribution {
    init {
        require(mean.isFinite()) { "mean must be finite" }
        require(stdDev.isFinite() && stdDev > 0) { "stdDev must be finite and positive" }
    }

    override fun sample(rng: Rng): Double {
        // Box-Muller transform
        var u1 = rng.uniformDouble()
        val u2 = rng.uniformDouble()

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

    override fun densityAt(x: Double): Double {
        val z = (x - mean) / stdDev
        return expFunction(-z * z / 2) / (stdDev * SQRT_2PI)
    }

    override fun cumulativeAt(x: Double): Double = additiveCumulative((x - mean) / stdDev)

    /**
     * The inverse goes through [additiveQuantile], which is what that function was built for. It
     * replaces two Newton steps taken against an error function accurate to 1.5e-07, which had
     * left this quantile eight orders of magnitude coarser than the cumulative beside it. The
     * infinities at the ends are the ones additiveQuantile already returns.
     */
    override fun quantileAt(p: Double): Double = mean + stdDev * additiveQuantile(p)
}
