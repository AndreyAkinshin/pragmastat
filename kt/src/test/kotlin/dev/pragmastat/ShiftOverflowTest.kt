package dev.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class ShiftOverflowTest {
    // Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
    // -/+Infinity on extreme finite input, turning the midpoint into NaN and
    // returning +-Infinity instead of the true finite shift.
    @Test
    fun `shift does not overflow search bounds on extreme finite input`() {
        val max = Double.MAX_VALUE
        assertEquals(0.0, shift(listOf(-max, max), listOf(-max, max), assumeSorted = true))
        assertEquals(max, shift(listOf(0.0, max), listOf(-max, 0.0), assumeSorted = true))
    }
}
