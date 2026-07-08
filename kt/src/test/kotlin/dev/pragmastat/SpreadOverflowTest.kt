package dev.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class SpreadOverflowTest {
    // Regression: the internal `active` pair counter must be Long. For n >= 65537 the
    // number of pairs exceeds Int.MAX, and an Int counter wrapped negative, tripping the
    // `active <= 0` guard and returning -Infinity instead of 0 for a constant vector.
    @Test
    fun `spreadImpl does not overflow the active counter on large constant input`() {
        val n = 65537
        val x = List(n) { 42.0 }
        assertEquals(0.0, spreadImpl(x, assumeSorted = true))
    }
}
