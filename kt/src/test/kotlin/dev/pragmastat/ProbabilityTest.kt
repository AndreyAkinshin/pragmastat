package dev.pragmastat

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import kotlin.test.assertEquals

/**
 * [Probability] is the typed [0, 1] wrapper used by the Sample-based public APIs.
 * Its `init` block (`require(value in 0.0..1.0)`) must reject out-of-range and NaN
 * inputs with [IllegalArgumentException]. This locks that contract.
 */
class ProbabilityTest {
    @Test
    fun rejectsBelowZero() {
        assertThrows<IllegalArgumentException> { Probability(-0.1) }
    }

    @Test
    fun rejectsAboveOne() {
        assertThrows<IllegalArgumentException> { Probability(1.5) }
    }

    @Test
    fun rejectsNaN() {
        // `x in 0.0..1.0` is false for NaN, so the require fails.
        assertThrows<IllegalArgumentException> { Probability(Double.NaN) }
    }

    @Test
    fun acceptsBoundaryAndInteriorValues() {
        assertEquals(0.0, Probability(0.0).value)
        assertEquals(1.0, Probability(1.0).value)
        assertEquals(0.5, Probability(0.5).value)
    }
}
