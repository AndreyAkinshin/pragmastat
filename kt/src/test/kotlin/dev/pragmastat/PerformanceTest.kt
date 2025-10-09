package dev.pragmastat

import kotlin.test.Test
import kotlin.test.assertTrue
import kotlin.math.abs
import kotlin.random.Random
import kotlin.system.measureTimeMillis

class PerformanceTest {

    private fun centerSimple(x: List<Double>): Double {
        val n = x.size
        val pairwiseAverages = mutableListOf<Double>()
        for (i in 0 until n) {
            for (j in i until n) {
                pairwiseAverages.add((x[i] + x[j]) / 2.0)
            }
        }
        return median(pairwiseAverages)
    }

    private fun spreadSimple(x: List<Double>): Double {
        val n = x.size
        if (n == 1) return 0.0
        val pairwiseDiffs = mutableListOf<Double>()
        for (i in 0 until n) {
            for (j in i + 1 until n) {
                pairwiseDiffs.add(abs(x[i] - x[j]))
            }
        }
        return median(pairwiseDiffs)
    }

    @Test
    fun testCenterCorrectness() {
        val random = Random(1729)

        for (n in 1..100) {
            repeat(n) {
                val x = List(n) { random.nextDouble() * 2 - 1 }

                val expected = centerSimple(x)
                val actual = center(x)

                assertTrue(
                    abs(expected - actual) < 1e-9,
                    "Mismatch for n=$n: expected=$expected, actual=$actual"
                )
            }
        }
    }

    @Test
    fun testSpreadCorrectness() {
        val random = Random(1729)

        for (n in 1..100) {
            repeat(n) {
                val x = List(n) { random.nextDouble() * 2 - 1 }

                val expected = spreadSimple(x)
                val actual = spread(x)

                assertTrue(
                    abs(expected - actual) < 1e-9,
                    "Mismatch for n=$n: expected=$expected, actual=$actual"
                )
            }
        }
    }

    @Test
    fun testCenterPerformance() {
        val random = Random(1729)
        val n = 100000
        val x = List(n) { random.nextDouble() * 2 - 1 }

        val elapsed = measureTimeMillis {
            val result = center(x)
            println("\nCenter for n=$n: ${"%.6f".format(result)}")
        }

        println("Elapsed time: ${elapsed}ms")

        assertTrue(
            elapsed < 5000,
            "Performance too slow: ${elapsed}ms"
        )
    }

    @Test
    fun testSpreadPerformance() {
        val random = Random(1729)
        val n = 100000
        val x = List(n) { random.nextDouble() * 2 - 1 }

        val elapsed = measureTimeMillis {
            val result = spread(x)
            println("\nSpread for n=$n: ${"%.6f".format(result)}")
        }

        println("Elapsed time: ${elapsed}ms")

        assertTrue(
            elapsed < 5000,
            "Performance too slow: ${elapsed}ms"
        )
    }
}
