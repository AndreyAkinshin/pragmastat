package dev.pragmastat

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

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

    // Bitwise, through [assertBitwise]: the wrapper stores the caller's value, so
    // the claim is that the payload survives the round trip untouched.
    @Test
    fun acceptsBoundaryAndInteriorValues() {
        assertBitwise(0.0, Probability(0.0).value, "Probability(0.0).value")
        assertBitwise(1.0, Probability(1.0).value, "Probability(1.0).value")
        assertBitwise(0.5, Probability(0.5).value, "Probability(0.5).value")
    }
}
