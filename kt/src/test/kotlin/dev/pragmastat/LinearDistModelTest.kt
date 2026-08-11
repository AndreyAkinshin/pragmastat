package dev.pragmastat

import dev.pragmastat.distributions.LinearDistModel
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Every expected value here is computed by hand against the model's own definition: piecewise
 * linear interpolation of the sorted sample at anchors (i-1)/(n-1), so the checks double as a
 * derivation, not just a pin.
 */
class LinearDistModelTest {
    private val x = listOf(1.0, 2.0, 4.0, 8.0)

    @Test
    fun quantileAtLandsOnEachObservationAtItsAnchor() {
        val model = LinearDistModel(x)
        assertEquals(1.0, model.quantileAt(0.0))
        assertEquals(2.0, model.quantileAt(1.0 / 3.0))
        assertEquals(4.0, model.quantileAt(2.0 / 3.0))
        assertEquals(8.0, model.quantileAt(1.0))
    }

    @Test
    fun quantileAtInterpolatesBetweenTwoObservations() {
        val model = LinearDistModel(x)
        assertEquals(3.0, model.quantileAt(0.5))
    }

    @Test
    fun quantileAtClampsOutsideTheUnitInterval() {
        val model = LinearDistModel(x)
        assertEquals(1.0, model.quantileAt(-0.1))
        assertEquals(8.0, model.quantileAt(1.1))
    }

    @Test
    fun cumulativeAtInvertsQuantileAt() {
        val model = LinearDistModel(x)
        assertEquals(0.5, model.cumulativeAt(3.0))
        assertEquals(1.0 / 3.0, model.cumulativeAt(2.0))
        assertEquals(2.0 / 3.0, model.cumulativeAt(4.0))
    }

    @Test
    fun cumulativeAtIsZeroBelowAndOneAboveTheSupport() {
        val model = LinearDistModel(x)
        assertEquals(0.0, model.cumulativeAt(1.0))
        assertEquals(0.0, model.cumulativeAt(-5.0))
        assertEquals(1.0, model.cumulativeAt(8.0))
        assertEquals(1.0, model.cumulativeAt(20.0))
    }

    @Test
    fun densityAtIsMassOverWidthInsideEachCell() {
        val model = LinearDistModel(x)
        assertEquals(1.0 / 3.0, model.densityAt(1.5))
        assertEquals(1.0 / 6.0, model.densityAt(3.0))
        assertEquals(1.0 / 12.0, model.densityAt(6.0))
        assertEquals(1.0 / 12.0, model.densityAt(8.0)) // The right edge belongs to the last cell.
    }

    @Test
    fun densityAtIsZeroOutsideTheSupport() {
        val model = LinearDistModel(x)
        assertEquals(0.0, model.densityAt(0.5))
        assertEquals(0.0, model.densityAt(9.0))
    }

    @Test
    fun scalarQueriesPropagateNaN() {
        val model = LinearDistModel(x)
        assertTrue(model.quantileAt(Double.NaN).isNaN())
        assertTrue(model.cumulativeAt(Double.NaN).isNaN())
        assertTrue(model.densityAt(Double.NaN).isNaN())
    }

    @Test
    fun getLandscapeAgreesWithTheScalarQueriesBitForBit() {
        val model = LinearDistModel(x)
        val landscape = model.getLandscape()

        assertEquals(x, landscape.boundaries)
        assertEquals(3, landscape.cellCount)
        for (i in x.indices) {
            assertEquals(model.cumulativeAt(x[i]), landscape.cumulative[i])
        }
        for (j in 0 until landscape.cellCount) {
            val midpoint = (x[j] + x[j + 1]) / 2.0
            assertEquals(model.densityAt(midpoint), landscape.densities[j])
        }
    }

    @Test
    fun sampleIsQuantileAtOfARawDrawnCumulance() {
        val model = LinearDistModel(x)
        val drawing = Rng("identity-seed")
        val inverting = Rng("identity-seed")
        for (i in 0 until 100) {
            val drawn = model.sample(drawing)
            val u = inverting.uniformDouble()
            assertEquals(model.quantileAt(u).toRawBits(), drawn.toRawBits())
        }
    }

    @Test
    fun constructorRejectsFewerThanTwoObservations() {
        val ex = kotlin.test.assertFailsWith<AssumptionException> { LinearDistModel(listOf(1.0)) }
        assertEquals(AssumptionId.DOMAIN, ex.violation?.id)
    }

    @Test
    fun constructorRejectsARepeatedValue() {
        val ex = kotlin.test.assertFailsWith<AssumptionException> { LinearDistModel(listOf(1.0, 1.0, 2.0)) }
        assertEquals(AssumptionId.DISTINCTNESS, ex.violation?.id)
    }

    @Test
    fun quantileRejectsACumulanceOutsideTheUnitInterval() {
        val model = LinearDistModel(x)
        val ex = kotlin.test.assertFailsWith<AssumptionException> { model.quantile(listOf(1.5)) }
        assertEquals(AssumptionId.CUMULANCE, ex.violation?.id)
    }

    @Test
    fun anEmptyQueryVectorIsNotAViolation() {
        val model = LinearDistModel(x)
        assertTrue(model.quantile(emptyList()).isEmpty())
        assertTrue(model.cumulative(emptyList()).isEmpty())
        assertTrue(model.density(emptyList()).isEmpty())
    }

    @Test
    fun sampleConstructorTakesTheSamplesUnit() {
        val meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
        val sample = Sample.of(x, meter)
        val model = LinearDistModel(sample)
        assertEquals(meter, model.unit)
    }

    @Test
    fun sampleConstructorRefusesAWeightedSample() {
        val sample = Sample.weighted(listOf(1.0, 2.0), listOf(1.0, 2.0))
        kotlin.test.assertFailsWith<AssumptionException> { LinearDistModel(sample) }
    }

    @Test
    fun measuredQuantileReturnsAMeasurementInTheModelsUnit() {
        val meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
        val model = LinearDistModel(Sample.of(x, meter))
        val result = model.measured.quantile(listOf(Probability(0.5)))
        assertEquals(3.0, result[0].value)
        assertEquals(meter, result[0].unit)
    }

    @Test
    fun measuredDensityReturnsAPlainNumber() {
        val meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
        val model = LinearDistModel(Sample.of(x, meter))
        val result = model.measured.density(listOf(Measurement(3.0, meter)))
        assertEquals(1.0 / 6.0, result[0])
    }
}
