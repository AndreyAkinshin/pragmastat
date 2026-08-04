package dev.pragmastat

import dev.pragmastat.distributions.Additive
import dev.pragmastat.distributions.Distribution
import dev.pragmastat.distributions.Exp
import dev.pragmastat.distributions.Multiplic
import dev.pragmastat.distributions.Power
import dev.pragmastat.distributions.Uniform
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * The five parametric families refuse malformed parameters at construction. Every parameter has to
 * be finite, and every scale or shape parameter has to be positive.
 *
 * Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
 * location constructed a family whose every draw is NaN. And in the other ports the predicate was
 * written negatively (`stdDev <= 0`), which is not the negation of `stdDev > 0` under IEEE 754:
 * `NaN <= 0` is false, so NaN walked past the guard. Kotlin's `require` already took the positive
 * form, so only the location parameters and the infinite scales were reachable here.
 */
class DistributionParameterTest {
    private fun refusalMessage(construct: () -> Any): String? = assertThrows<IllegalArgumentException> { construct() }.message

    @Test
    fun uniformRefusesNonFiniteBounds() {
        assertEquals("min must be finite", refusalMessage { Uniform(Double.NaN, 1.0) })
        assertEquals("min must be finite", refusalMessage { Uniform(Double.NEGATIVE_INFINITY, 1.0) })
        assertEquals("max must be finite", refusalMessage { Uniform(0.0, Double.NaN) })
        assertEquals("max must be finite", refusalMessage { Uniform(0.0, Double.POSITIVE_INFINITY) })
    }

    @Test
    fun uniformRefusesAnEmptyInterval() {
        assertEquals("min must be less than max", refusalMessage { Uniform(1.0, 1.0) })
        assertEquals("min must be less than max", refusalMessage { Uniform(2.0, 1.0) })
    }

    @Test
    fun additiveRefusesMalformedParameters() {
        assertEquals("mean must be finite", refusalMessage { Additive(Double.NaN, 1.0) })
        assertEquals("mean must be finite", refusalMessage { Additive(Double.POSITIVE_INFINITY, 1.0) })
        val want = "stdDev must be finite and positive"
        assertEquals(want, refusalMessage { Additive(0.0, Double.NaN) })
        assertEquals(want, refusalMessage { Additive(0.0, Double.POSITIVE_INFINITY) })
        assertEquals(want, refusalMessage { Additive(0.0, 0.0) })
        assertEquals(want, refusalMessage { Additive(0.0, -1.0) })
    }

    @Test
    fun multiplicRefusesMalformedParameters() {
        assertEquals("logMean must be finite", refusalMessage { Multiplic(Double.NaN, 1.0) })
        assertEquals("logMean must be finite", refusalMessage { Multiplic(Double.POSITIVE_INFINITY, 1.0) })
        val want = "logStdDev must be finite and positive"
        assertEquals(want, refusalMessage { Multiplic(0.0, Double.NaN) })
        assertEquals(want, refusalMessage { Multiplic(0.0, Double.POSITIVE_INFINITY) })
        assertEquals(want, refusalMessage { Multiplic(0.0, 0.0) })
        assertEquals(want, refusalMessage { Multiplic(0.0, -1.0) })
    }

    @Test
    fun expRefusesMalformedParameters() {
        val want = "rate must be finite and positive"
        assertEquals(want, refusalMessage { Exp(Double.NaN) })
        assertEquals(want, refusalMessage { Exp(Double.POSITIVE_INFINITY) })
        assertEquals(want, refusalMessage { Exp(0.0) })
        assertEquals(want, refusalMessage { Exp(-1.0) })
    }

    @Test
    fun powerRefusesMalformedParameters() {
        val wantMin = "min must be finite and positive"
        assertEquals(wantMin, refusalMessage { Power(Double.NaN, 3.0) })
        assertEquals(wantMin, refusalMessage { Power(Double.POSITIVE_INFINITY, 3.0) })
        assertEquals(wantMin, refusalMessage { Power(0.0, 3.0) })
        assertEquals(wantMin, refusalMessage { Power(-1.0, 3.0) })
        val wantShape = "shape must be finite and positive"
        assertEquals(wantShape, refusalMessage { Power(1.0, Double.NaN) })
        assertEquals(wantShape, refusalMessage { Power(1.0, Double.POSITIVE_INFINITY) })
        assertEquals(wantShape, refusalMessage { Power(1.0, 0.0) })
        assertEquals(wantShape, refusalMessage { Power(1.0, -1.0) })
    }

    @Test
    fun wellFormedFamiliesStillConstructAndDraw() {
        val rng = Rng("distribution-parameters")
        val families: List<Distribution> =
            listOf(
                Uniform(-2.5, 7.5),
                Additive(10.0, 2.0),
                Multiplic(1.0, 0.5),
                Exp(3.0),
                Power(1.0, 2.0),
            )
        for (family in families) {
            assertTrue(family.sample(rng).isFinite())
        }
    }
}
