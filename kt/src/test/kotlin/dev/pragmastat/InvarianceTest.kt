package dev.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.math.abs

class InvarianceTest {

    private val epsilon = 1e-9

    private fun assertClose(expected: Double, actual: Double, tolerance: Double = epsilon) {
        assertTrue(abs(expected - actual) < tolerance,
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})")
    }

    private val sampleSizes = (2..10).toList()

    private fun uniformVec(rng: Rng, n: Int): List<Double> = List(n) { rng.uniformDouble() }

    @Test
    fun testCenterLocationInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val shift = 10.0
            val xShifted = x.map { it + shift }
            assertClose(center(x) + shift, center(xShifted))
        }
    }

    @Test
    fun testCenterScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            assertClose(center(x) * scale, center(xScaled))
        }
    }

    @Test
    fun testSpreadLocationInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val shift = 10.0
            val xShifted = x.map { it + shift }
            assertClose(spread(x), spread(xShifted))
        }
    }

    @Test
    fun testSpreadScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            assertClose(spread(x) * scale, spread(xScaled))
        }
    }
    
    @Suppress("DEPRECATION")
    @Test
    fun testRelSpreadScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            assertClose(relSpread(x), relSpread(xScaled))
        }
    }

    @Test
    fun testShiftLocationInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val shiftVal = 10.0
            val xShifted = x.map { it + shiftVal }
            val yShifted = y.map { it + shiftVal }
            assertClose(shift(x, y), shift(xShifted, yShifted))
        }
    }

    @Test
    fun testShiftScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            val yScaled = y.map { it * scale }
            assertClose(shift(x, y) * scale, shift(xScaled, yScaled))
        }
    }

    @Test
    fun testRatioScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            val yScaled = y.map { it * scale }
            assertClose(ratio(x, y), ratio(xScaled, yScaled))
        }
    }

    @Test
    fun testAvgSpreadLocationInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val shiftVal = 10.0
            val xShifted = x.map { it + shiftVal }
            val yShifted = y.map { it + shiftVal }
            assertClose(avgSpread(x, y), avgSpread(xShifted, yShifted))
        }
    }

    @Test
    fun testAvgSpreadScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            val yScaled = y.map { it * scale }
            assertClose(avgSpread(x, y) * scale, avgSpread(xScaled, yScaled))
        }
    }

    @Test
    fun testDisparityLocationInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val shiftVal = 10.0
            val xShifted = x.map { it + shiftVal }
            val yShifted = y.map { it + shiftVal }
            assertClose(disparity(x, y), disparity(xShifted, yShifted))
        }
    }

    @Test
    fun testDisparityScaleInvariance() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val scale = 2.5
            val xScaled = x.map { it * scale }
            val yScaled = y.map { it * scale }
            assertClose(disparity(x, y), disparity(xScaled, yScaled))
        }
    }

    @Test
    fun testCenterNegate() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val xNeg = x.map { -it }
            assertClose(-center(x), center(xNeg))
        }
    }

    @Test
    fun testSpreadNegate() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val xNeg = x.map { -it }
            assertClose(spread(x), spread(xNeg))
        }
    }

    @Test
    fun testShiftAsymmetricShift() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val xShifted = x.map { it + 3 }
            val yShifted = y.map { it + 2 }
            assertClose(shift(x, y) + 1, shift(xShifted, yShifted))
        }
    }

    @Test
    fun testShiftAntisymmetry() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            assertClose(shift(x, y), -shift(y, x))
        }
    }

    @Test
    fun testRatioAsymmetricScale() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val xScaled = x.map { it * 2 }
            val yScaled = y.map { it * 3 }
            assertClose((2.0 / 3) * ratio(x, y), ratio(xScaled, yScaled))
        }
    }

    @Test
    fun testAvgSpreadEqual() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            assertClose(spread(x), avgSpread(x, x))
        }
    }

    @Test
    fun testAvgSpreadSymmetry() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            assertClose(avgSpread(x, y), avgSpread(y, x))
        }
    }

    @Test
    fun testAvgSpreadAverage() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val x5 = x.map { it * 5 }
            assertClose(3 * spread(x), avgSpread(x, x5))
        }
    }

    @Test
    fun testAvgSpreadScaleNeg() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val xScaled = x.map { it * -2 }
            val yScaled = y.map { it * -2 }
            assertClose(2 * avgSpread(x, y), avgSpread(xScaled, yScaled))
        }
    }

    @Test
    fun testDisparityScaleNeg() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            val xScaled = x.map { it * -2 }
            val yScaled = y.map { it * -2 }
            assertClose(-disparity(x, y), disparity(xScaled, yScaled))
        }
    }

    @Test
    fun testDisparityAntisymmetry() {
        val rng = Rng(1729)
        for (n in sampleSizes) {
            val x = uniformVec(rng, n)
            val y = uniformVec(rng, n)
            assertClose(disparity(x, y), -disparity(y, x))
        }
    }

    @Test
    fun shufflePreservesMultiset() {
        for (n in listOf(1, 2, 5, 10, 100)) {
            val x = (0 until n).map { it.toDouble() }
            val rng = Rng(42)
            val shuffled = rng.shuffle(x)
            assertEquals(x, shuffled.sorted())
        }
    }

    @Test
    fun sampleCorrectSize() {
        val x = (0 until 10).map { it.toDouble() }
        for (k in listOf(1, 3, 5, 10, 15)) {
            val rng = Rng(42)
            val sampled = rng.sample(x, k)
            assertEquals(minOf(k, x.size), sampled.size)
        }
    }

    @Test
    fun sampleElementsFromSource() {
        val x = (0 until 10).map { it.toDouble() }
        val rng = Rng(42)
        val sampled = rng.sample(x, 5)
        for (elem in sampled) {
            assertTrue(elem in x)
        }
    }

    @Test
    fun samplePreservesOrder() {
        val x = (0 until 10).map { it.toDouble() }
        val rng = Rng(42)
        val sampled = rng.sample(x, 5)
        for (i in 1 until sampled.size) {
            assertTrue(sampled[i] > sampled[i - 1])
        }
    }

    @Test
    fun sampleNoDuplicates() {
        for (n in listOf(2, 3, 5, 10, 20)) {
            val source = (0 until n).map { it.toDouble() }
            for (k in listOf(1, n / 2, n)) {
                val rng = Rng(42)
                val sampled = rng.sample(source, k)
                assertEquals(sampled.size, sampled.distinct().size,
                    "Duplicate in sample(n=$n, k=$k)")
            }
        }
    }

    @Test
    fun resampleNegativeK() {
        val rng = Rng(42)
        org.junit.jupiter.api.assertThrows<IllegalArgumentException> {
            rng.resample(listOf(1.0, 2.0, 3.0), -1)
        }
    }

    @Test
    fun resampleElementsFromSource() {
        val x = (0 until 5).map { it.toDouble() }
        val rng = Rng(42)
        val resampled = rng.resample(x, 10)
        for (elem in resampled) {
            assertTrue(elem in x)
        }
    }

    @Test
    fun resampleK0Throws() {
        val rng = Rng(42)
        org.junit.jupiter.api.assertThrows<IllegalArgumentException> {
            rng.resample(listOf(1.0, 2.0, 3.0), 0)
        }
    }

    @Test
    fun shuffleEmptyThrows() {
        val rng = Rng(42)
        org.junit.jupiter.api.assertThrows<IllegalArgumentException> {
            rng.shuffle(emptyList<Double>())
        }
    }

    @Test
    fun sampleK0Throws() {
        val rng = Rng(42)
        org.junit.jupiter.api.assertThrows<IllegalArgumentException> {
            rng.sample(listOf(1.0, 2.0, 3.0), 0)
        }
    }

    @Test
    fun sampleEmptyThrows() {
        val rng = Rng(42)
        org.junit.jupiter.api.assertThrows<IllegalArgumentException> {
            rng.sample(emptyList<Double>(), 1)
        }
    }
}