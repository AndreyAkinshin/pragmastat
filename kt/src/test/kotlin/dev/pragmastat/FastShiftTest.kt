package dev.pragmastat

import org.junit.jupiter.api.Test
import kotlin.math.abs
import kotlin.math.ceil
import kotlin.math.floor
import kotlin.random.Random
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

class FastShiftTest {

    private val tolerance = 1e-9

    private fun assertClose(expected: Double, actual: Double, tol: Double = tolerance) {
        assertTrue(
            abs(expected - actual) < tol,
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})"
        )
    }

    @Test
    fun testSmallArraysMatchNaive() {
        val random = Random(1729)

        for (m in 1..20) {
            for (n in 1..20) {
                repeat(5) {
                    val x = List(m) { random.nextDouble() * 10 }
                    val y = List(n) { random.nextDouble() * 10 }
                    val p = doubleArrayOf(0.0, 0.25, 0.5, 0.75, 1.0)

                    val actual = fastShift(x, y, p)
                    val expected = naiveQuantiles(x, y, p)

                    assertEquals(expected.size, actual.size)
                    for (i in expected.indices) {
                        assertClose(expected[i], actual[i])
                    }
                }
            }
        }
    }

    @Test
    fun testMediumArraysMatchNaive() {
        val random = Random(42)

        for (size in 20..100 step 10) {
            repeat(3) {
                val x = List(size) { random.nextDouble() * 10 }
                val y = List(size / 2) { random.nextDouble() * 10 }
                val p = doubleArrayOf(0.1, 0.5, 0.9)

                val actual = fastShift(x, y, p)
                val expected = naiveQuantiles(x, y, p)

                assertEquals(expected.size, actual.size)
                for (i in expected.indices) {
                    assertClose(expected[i], actual[i])
                }
            }
        }
    }

    @Test
    fun testDifferentDistributionsAllQuantiles() {
        var seed = 2024
        val distributions = listOf(
            Pair(0.0, 1.0),
            Pair(5.0, 2.0),
            Pair(-10.0, 1.0),
            Pair(0.0, 0.1)
        )

        val probabilities = doubleArrayOf(0.0, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 1.0)

        for ((mean, scale) in distributions) {
            val random = Random(seed++)
            repeat(10) {
                val x = List(15) { mean + random.nextDouble() * scale }
                val y = List(10) { mean + random.nextDouble() * scale }

                val actual = fastShift(x, y, probabilities)
                val expected = naiveQuantiles(x, y, probabilities)

                assertEquals(expected.size, actual.size)
                for (i in expected.indices) {
                    assertClose(expected[i], actual[i])
                }
            }
        }
    }

    @Test
    fun testUnsortedInputMatchesSorted() {
        val random = Random(999)

        repeat(50) {
            val xRaw = List(20) { random.nextDouble() * 10 }
            val yRaw = List(15) { random.nextDouble() * 10 }
            val p = doubleArrayOf(0.25, 0.5, 0.75)

            val xSorted = xRaw.sorted()
            val ySorted = yRaw.sorted()

            val xShuffled = xRaw.shuffled(random)
            val yShuffled = yRaw.shuffled(random)

            val resultUnsorted = fastShift(xShuffled, yShuffled, p, assumeSorted = false)
            val resultSorted = fastShift(xSorted, ySorted, p, assumeSorted = true)

            assertEquals(resultSorted.size, resultUnsorted.size)
            for (i in resultSorted.indices) {
                assertClose(resultSorted[i], resultUnsorted[i])
            }
        }
    }

    @Test
    fun testSingleElementReturnsConstant() {
        val random = Random(123)

        repeat(20) {
            val x = listOf(random.nextDouble() * 10)
            val y = listOf(random.nextDouble() * 10)
            val p = doubleArrayOf(0.0, 0.25, 0.5, 0.75, 1.0)

            val result = fastShift(x, y, p)
            val expected = x[0] - y[0]

            for (q in result) {
                assertClose(expected, q)
            }
        }
    }

    @Test
    fun testIdenticalArraysMedianIsZero() {
        val random = Random(456)

        for (size in 1..30) {
            repeat(3) {
                val x = List(size) { random.nextDouble() * 10 }
                val p = doubleArrayOf(0.5)

                val result = fastShift(x, x, p)

                assertClose(0.0, result[0])
            }
        }
    }

    @Test
    fun testAsymmetricSizesCorrectResults() {
        val random = Random(789)

        val configs = listOf(
            Pair(1, 100),
            Pair(100, 1),
            Pair(10, 50),
            Pair(50, 10),
            Pair(5, 200)
        )

        for ((m, n) in configs) {
            val x = List(m) { random.nextDouble() * 10 }
            val y = List(n) { random.nextDouble() * 10 }
            val p = doubleArrayOf(0.0, 0.5, 1.0)

            val actual = fastShift(x, y, p)
            val expected = naiveQuantiles(x, y, p)

            assertEquals(expected.size, actual.size)
            for (i in expected.indices) {
                assertClose(expected[i], actual[i])
            }
        }
    }

    @Test
    fun testExtremeQuantilesMatchMinMax() {
        val random = Random(321)

        repeat(30) { trial ->
            val x = List(10 + trial) { random.nextDouble() * 10 }
            val y = List(8 + trial / 2) { random.nextDouble() * 10 }
            val p = doubleArrayOf(0.0, 1.0)

            val result = fastShift(x, y, p)

            var min = Double.POSITIVE_INFINITY
            var max = Double.NEGATIVE_INFINITY
            for (xi in x) {
                for (yj in y) {
                    val diff = xi - yj
                    if (diff < min) min = diff
                    if (diff > max) max = diff
                }
            }

            assertClose(min, result[0])
            assertClose(max, result[1])
        }
    }

    @Test
    fun testManyProbabilitiesMonotonicIncreasing() {
        val random = Random(654)

        repeat(20) {
            val x = List(25) { random.nextDouble() * 10 }
            val y = List(20) { random.nextDouble() * 10 }

            val p = DoubleArray(21) { i -> i / 20.0 }

            val result = fastShift(x, y, p)

            for (i in 1 until result.size) {
                assertTrue(
                    result[i] >= result[i - 1] - tolerance,
                    "Quantiles must be non-decreasing: result[$i]=${result[i]} < result[${i - 1}]=${result[i - 1]}"
                )
            }
        }
    }

    @Test
    fun testNegativeValuesHandledCorrectly() {
        val random = Random(111)

        repeat(20) {
            val x = List(15) { -50.0 + random.nextDouble() * 10 }
            val y = List(12) { -50.0 + random.nextDouble() * 10 }
            val p = doubleArrayOf(0.25, 0.5, 0.75)

            val actual = fastShift(x, y, p)
            val expected = naiveQuantiles(x, y, p)

            for (i in expected.indices) {
                assertClose(expected[i], actual[i])
            }
        }
    }

    @Test
    fun testDuplicateValuesHandledCorrectly() {
        val random = Random(222)

        repeat(10) {
            val x = List(12) { (random.nextDouble() * 5).toInt() / 5.0 }
            val y = List(10) { (random.nextDouble() * 5).toInt() / 5.0 }
            val p = doubleArrayOf(0.0, 0.5, 1.0)

            val actual = fastShift(x, y, p)
            val expected = naiveQuantiles(x, y, p)

            for (i in expected.indices) {
                assertClose(expected[i], actual[i])
            }
        }
    }

    @Test
    fun testVerySmallValuesNumericalStability() {
        val random = Random(333)

        repeat(10) {
            val x = List(10) { random.nextDouble() * 1e-8 }
            val y = List(10) { random.nextDouble() * 1e-8 }
            val p = doubleArrayOf(0.5)

            val result = fastShift(x, y, p)

            assertTrue(!result[0].isNaN())
            assertTrue(!result[0].isInfinite())
        }
    }

    @Test
    fun testLargeValuesNumericalStability() {
        val random = Random(444)

        repeat(10) {
            val x = List(10) { 1e6 + random.nextDouble() * 1e5 }
            val y = List(10) { 1e6 + random.nextDouble() * 1e5 }
            val p = doubleArrayOf(0.5)

            val result = fastShift(x, y, p)

            assertTrue(!result[0].isNaN())
            assertTrue(!result[0].isInfinite())
        }
    }

    @Test
    fun testZeroSpreadAllSame() {
        val x = List(10) { 5.0 }
        val y = List(8) { 2.0 }
        val p = doubleArrayOf(0.0, 0.25, 0.5, 0.75, 1.0)

        val result = fastShift(x, y, p)

        for (q in result) {
            assertClose(3.0, q)
        }
    }

    @Test
    fun testLargeArraysPerformance() {
        val random = Random(1729)
        val x = List(500) { random.nextDouble() * 10 }
        val y = List(500) { random.nextDouble() * 10 }
        val p = doubleArrayOf(0.5)

        val startTime = System.currentTimeMillis()
        val result = fastShift(x, y, p)
        val elapsedTime = System.currentTimeMillis() - startTime

        println("500x500 arrays: ${elapsedTime}ms")
        assertTrue(elapsedTime < 5000, "Should complete in under 5 seconds")
        assertEquals(1, result.size)
    }

    @Test
    fun testVeryLargeArraysPerformance() {
        val random = Random(9999)
        val x = List(1000) { random.nextDouble() * 10 }
        val y = List(1000) { random.nextDouble() * 10 }
        val p = doubleArrayOf(0.5)

        val startTime = System.currentTimeMillis()
        val result = fastShift(x, y, p)
        val elapsedTime = System.currentTimeMillis() - startTime

        println("1000x1000 arrays (1M pairs): ${elapsedTime}ms")
        assertTrue(elapsedTime < 10000, "Should complete in under 10 seconds")
        assertEquals(1, result.size)
    }

    @Test
    fun testManyQuantilesPerformance() {
        val random = Random(7777)
        val x = List(200) { random.nextDouble() * 10 }
        val y = List(200) { random.nextDouble() * 10 }
        val p = DoubleArray(21) { i -> i / 20.0 }

        val startTime = System.currentTimeMillis()
        val result = fastShift(x, y, p)
        val elapsedTime = System.currentTimeMillis() - startTime

        println("200x200 arrays, 21 quantiles: ${elapsedTime}ms")
        assertTrue(elapsedTime < 5000, "Should complete in under 5 seconds")
        assertEquals(21, result.size)
    }

    @Test
    fun testEmptyArraysThrowsException() {
        val valid = listOf(1.0, 2.0)
        val p = doubleArrayOf(0.5)

        assertFailsWith<IllegalArgumentException> { fastShift(emptyList(), valid, p) }
        assertFailsWith<IllegalArgumentException> { fastShift(valid, emptyList(), p) }
    }

    @Test
    fun testInvalidProbabilitiesThrowsException() {
        val x = listOf(1.0, 2.0)
        val y = listOf(3.0, 4.0)

        assertFailsWith<IllegalArgumentException> {
            fastShift(x, y, doubleArrayOf(-0.1))
        }
        assertFailsWith<IllegalArgumentException> {
            fastShift(x, y, doubleArrayOf(1.1))
        }
        assertFailsWith<IllegalArgumentException> {
            fastShift(x, y, doubleArrayOf(Double.NaN))
        }
    }

    @Test
    fun testNaNInDataThrowsException() {
        val xWithNaN = listOf(1.0, Double.NaN)
        val yWithNaN = listOf(3.0, Double.NaN)
        val valid = listOf(1.0, 2.0)
        val p = doubleArrayOf(0.5)

        assertFailsWith<IllegalStateException> {
            fastShift(xWithNaN, valid, p)
        }
        assertFailsWith<IllegalStateException> {
            fastShift(valid, yWithNaN, p)
        }
    }

    @Test
    fun testEmptyProbabilitiesReturnsEmpty() {
        val x = listOf(1.0, 2.0)
        val y = listOf(3.0, 4.0)
        val p = doubleArrayOf()

        val result = fastShift(x, y, p)

        assertEquals(0, result.size)
    }

    @Test
    fun testShiftInvarianceXShift() {
        val random = Random(555)

        repeat(10) {
            val x = List(15) { random.nextDouble() * 10 }
            val y = List(12) { random.nextDouble() * 10 }
            val p = doubleArrayOf(0.25, 0.5, 0.75)
            val shift = random.nextDouble() * 10

            val result1 = fastShift(x, y, p)
            val xShifted = x.map { it + shift }
            val result2 = fastShift(xShifted, y, p)

            for (i in result1.indices) {
                assertClose(result1[i] + shift, result2[i])
            }
        }
    }

    @Test
    fun testShiftInvarianceYShift() {
        val random = Random(666)

        repeat(10) {
            val x = List(15) { random.nextDouble() * 10 }
            val y = List(12) { random.nextDouble() * 10 }
            val p = doubleArrayOf(0.25, 0.5, 0.75)
            val shift = random.nextDouble() * 10

            val result1 = fastShift(x, y, p)
            val yShifted = y.map { it + shift }
            val result2 = fastShift(x, yShifted, p)

            for (i in result1.indices) {
                assertClose(result1[i] - shift, result2[i])
            }
        }
    }

    @Test
    fun testScaleInvariance() {
        val random = Random(777)

        repeat(10) {
            val x = List(15) { random.nextDouble() * 10 }
            val y = List(12) { random.nextDouble() * 10 }
            val p = doubleArrayOf(0.5)
            val scale = 2.0

            val result1 = fastShift(x, y, p)
            val xScaled = x.map { it * scale }
            val yScaled = y.map { it * scale }
            val result2 = fastShift(xScaled, yScaled, p)

            for (i in result1.indices) {
                assertClose(result1[i] * scale, result2[i], 1e-6)
            }
        }
    }

    private fun naiveQuantiles(x: List<Double>, y: List<Double>, p: DoubleArray): DoubleArray {
        val diffs = mutableListOf<Double>()
        for (xi in x) {
            for (yj in y) {
                diffs.add(xi - yj)
            }
        }

        diffs.sort()

        return DoubleArray(p.size) { i ->
            val n = diffs.size
            val h = 1.0 + (n - 1) * p[i]
            val lo = floor(h).toLong().coerceIn(1, n.toLong())
            val hi = ceil(h).toLong().coerceIn(1, n.toLong())
            val gamma = h - lo

            val a = diffs[(lo - 1).toInt()]
            val b = diffs[(hi - 1).toInt()]

            if (gamma == 0.0) a else (1.0 - gamma) * a + gamma * b
        }
    }
}
