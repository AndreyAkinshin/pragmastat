package dev.pragmastat

import org.junit.jupiter.api.Test

class SpreadOverflowTest {
    // Regression: the internal `active` pair counter must be Long. For n >= 65537 the
    // number of pairs exceeds Int.MAX, and an Int counter wrapped negative, tripping the
    // `active <= 0` guard and returning -Infinity instead of 0 for a constant vector.
    //
    // Bitwise, through assertBitwise: the spread of a constant vector is an exact
    // zero, and the failure mode under test is an infinity, which only a payload
    // comparison reports legibly.
    @Test
    fun `spreadImpl does not overflow the active counter on large constant input`() {
        val n = 65537
        val x = List(n) { 42.0 }
        assertBitwise(0.0, spreadImpl(x, assumeSorted = true), "spreadImpl(constant n=$n)")
    }
}
