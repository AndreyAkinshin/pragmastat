package dev.pragmastat

import kotlin.test.Test
import kotlin.test.assertTrue
import kotlin.math.abs
import kotlin.system.measureTimeMillis

class PerformanceTest {

    @Test
    fun testCenterPerformance() {
        val n = 100000
        val x = List(n) { i -> (i + 1).toDouble() }

        val elapsed = measureTimeMillis {
            val result = center(x)
            println("\nCenter for n=$n: ${"%.6f".format(result)}")

            val expected = 50000.5
            assertTrue(
                abs(result - expected) < 1e-9,
                "Center for n=$n: expected $expected, got $result"
            )
        }

        println("Elapsed time: ${elapsed}ms")

        assertTrue(
            elapsed < 5000,
            "Performance too slow: ${elapsed}ms"
        )
    }

    @Test
    fun testSpreadPerformance() {
        val n = 100000
        val x = List(n) { i -> (i + 1).toDouble() }

        val elapsed = measureTimeMillis {
            val result = spread(x)
            println("\nSpread for n=$n: ${"%.6f".format(result)}")

            val expected = 29290.0
            assertTrue(
                abs(result - expected) < 1e-9,
                "Spread for n=$n: expected $expected, got $result"
            )
        }

        println("Elapsed time: ${elapsed}ms")

        assertTrue(
            elapsed < 5000,
            "Performance too slow: ${elapsed}ms"
        )
    }

    @Test
    fun testShiftPerformance() {
        val n = 100000
        val x = List(n) { i -> (i + 1).toDouble() }
        val y = List(n) { i -> (i + 1).toDouble() }

        val elapsed = measureTimeMillis {
            val result = shift(x, y)
            println("\nShift for n=m=$n: ${"%.6f".format(result)}")

            val expected = 0.0
            assertTrue(
                abs(result - expected) < 1e-9,
                "Shift for n=m=$n: expected $expected, got $result"
            )
        }

        println("Elapsed time: ${elapsed}ms")

        assertTrue(
            elapsed < 5000,
            "Performance too slow: ${elapsed}ms"
        )
    }
}
