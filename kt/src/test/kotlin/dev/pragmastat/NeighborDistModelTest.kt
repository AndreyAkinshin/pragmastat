package dev.pragmastat

import dev.pragmastat.distributions.NeighborDistModel
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

class NeighborDistModelTest {
    private val x = listOf(0.0, 1.0, 2.0, 4.0, 8.0)

    @Test
    fun supportMatchesTheMinAndMaxObservation() {
        val model = NeighborDistModel(x)
        assertEquals(0.0, model.supportMin)
        assertEquals(8.0, model.supportMax)
    }

    @Test
    fun cellCountIsAtMostOneFewerThanTheObservationCount() {
        val model = NeighborDistModel(x)
        assertEquals(x.size, model.cellCount)
    }

    @Test
    fun cumulativeAtTheSupportIsZeroAndOneExactly() {
        val model = NeighborDistModel(x)
        assertEquals(0.0, model.cumulativeAt(model.supportMin))
        assertEquals(1.0, model.cumulativeAt(model.supportMax))
    }

    @Test
    fun cellMassesSumToExactlyOne() {
        val model = NeighborDistModel(x)
        val landscape = model.getLandscape()
        val sum = landscape.masses.sum()
        assertEquals(1.0, sum)
    }

    @Test
    fun cumulativeAtIsNonDecreasingAcrossTheBoundaries() {
        val model = NeighborDistModel(x)
        val cumulative = model.getLandscape().cumulative
        for (i in 1 until cumulative.size) {
            assertTrue(cumulative[i] > cumulative[i - 1])
        }
    }

    @Test
    fun quantileAtInvertsCumulativeAtAtEachBoundary() {
        val model = NeighborDistModel(x)
        val landscape = model.getLandscape()
        for (i in landscape.boundaries.indices) {
            assertEquals(landscape.boundaries[i], model.quantileAt(landscape.cumulative[i]))
        }
    }

    @Test
    fun quantileAtClampsOutsideTheUnitInterval() {
        val model = NeighborDistModel(x)
        assertEquals(model.supportMin, model.quantileAt(-0.1))
        assertEquals(model.supportMax, model.quantileAt(1.1))
    }

    @Test
    fun densityAtIsZeroOutsideTheSupport() {
        val model = NeighborDistModel(x)
        assertEquals(0.0, model.densityAt(model.supportMin - 1.0))
        assertEquals(0.0, model.densityAt(model.supportMax + 1.0))
    }

    @Test
    fun scalarQueriesPropagateNaN() {
        val model = NeighborDistModel(x)
        assertTrue(model.quantileAt(Double.NaN).isNaN())
        assertTrue(model.cumulativeAt(Double.NaN).isNaN())
        assertTrue(model.densityAt(Double.NaN).isNaN())
    }

    @Test
    fun getLandscapeAgreesWithTheScalarQueriesBitForBit() {
        val model = NeighborDistModel(x)
        val landscape = model.getLandscape()
        for (i in landscape.boundaries.indices) {
            assertEquals(landscape.cumulative[i], model.cumulativeAt(landscape.boundaries[i]))
        }
        for (j in 0 until landscape.cellCount) {
            val midpoint = (landscape.boundaries[j] + landscape.boundaries[j + 1]) / 2.0
            assertEquals(landscape.densities[j], model.densityAt(midpoint))
        }
    }

    @Test
    fun getLandscapeExposesScaleIndexAndRadius() {
        val model = NeighborDistModel(x)
        val landscape = model.getLandscape()
        // n = 5: ceilSqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = 3 / 2 = 1.
        assertEquals(3, landscape.scaleIndex)
        assertEquals(1, landscape.radius)
    }

    @Test
    fun getLandscapeSourcesMapEachCellToItsObservation() {
        val model = NeighborDistModel(x)
        assertEquals(listOf(0, 1, 2, 3, 4), model.getLandscape().sources)
    }

    @Test
    fun aZeroWidthCellFromACollapsedMidpointIsDropped() {
        // The center observation sits exactly one ULP from each neighbor on either side. Both
        // adjacent midpoints round to 1.0 itself under round-to-nearest-even (each is exactly
        // halfway between two representable doubles, and 1.0 has the even mantissa), so the
        // center cell's two boundaries collapse onto the same point and the cell is dropped.
        val center = 1.0
        val below = Double.fromBits(center.toRawBits() - 1)
        val above = Double.fromBits(center.toRawBits() + 1)
        val values = listOf(0.0, below, center, above, 10.0)

        val model = NeighborDistModel(values)
        val landscape = model.getLandscape()

        assertEquals(values.size - 1, landscape.cellCount)
        assertTrue(2 !in landscape.sources)
    }

    @Test
    fun sampleIsQuantileAtOfARawDrawnCumulance() {
        val model = NeighborDistModel(x)
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
        val ex = assertFailsWith<AssumptionException> { NeighborDistModel(listOf(1.0)) }
        assertEquals(AssumptionId.DOMAIN, ex.violation?.id)
    }

    @Test
    fun constructorRejectsARepeatedValue() {
        val ex = assertFailsWith<AssumptionException> { NeighborDistModel(listOf(1.0, 1.0, 2.0)) }
        assertEquals(AssumptionId.DISTINCTNESS, ex.violation?.id)
    }

    @Test
    fun quantileRejectsACumulanceOutsideTheUnitInterval() {
        val model = NeighborDistModel(x)
        val ex = assertFailsWith<AssumptionException> { model.quantile(listOf(-0.1)) }
        assertEquals(AssumptionId.CUMULANCE, ex.violation?.id)
    }

    @Test
    fun anEmptyQueryVectorIsNotAViolation() {
        val model = NeighborDistModel(x)
        assertTrue(model.quantile(emptyList()).isEmpty())
        assertTrue(model.cumulative(emptyList()).isEmpty())
        assertTrue(model.density(emptyList()).isEmpty())
    }

    @Test
    fun aBrokenAssumeSortedPromiseIsANumericalConformanceFailure() {
        // DistSampleGuard's distinctness check only looks at ADJACENT elements of the (unsorted,
        // since assumeSorted is a promise it trusts) list, so a non-adjacent repeat -- x[2] and
        // x[6] both 5.0 here -- slips through it. That repeat then makes one window's span
        // exactly zero while its cell keeps a nonzero width, so the cell's unnormalized mass
        // divides by zero and the total the whole reconstruction divides by is not usable.
        val scrambled = listOf(0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0)
        assertFailsWith<IllegalStateException> { NeighborDistModel(scrambled, assumeSorted = true) }
    }

    @Test
    fun sampleConstructorTakesTheSamplesUnit() {
        val meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
        val model = NeighborDistModel(Sample.of(x, meter))
        assertEquals(meter, model.unit)
    }

    @Test
    fun measuredDensityReturnsAPlainNumber() {
        val meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
        val model = NeighborDistModel(Sample.of(x, meter))
        val result = model.measured.density(listOf(Measurement(3.0, meter)))
        assertEquals(model.densityAt(3.0), result[0])
    }
}
