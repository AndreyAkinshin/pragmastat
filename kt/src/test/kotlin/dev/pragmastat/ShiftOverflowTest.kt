package dev.pragmastat

import org.junit.jupiter.api.Test

class ShiftOverflowTest {
    // Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
    // -/+Infinity on extreme finite input, turning the midpoint into NaN and
    // returning +-Infinity instead of the true finite shift.
    //
    // Bitwise, through assertBitwise: the expected values are exact pairwise
    // midpoints, and the failure mode under test is an infinity or a NaN, which
    // only a payload comparison reports legibly.
    @Test
    fun `shift does not overflow search bounds on extreme finite input`() {
        val max = Double.MAX_VALUE
        assertBitwise(
            0.0,
            shift(listOf(-max, max), listOf(-max, max), assumeSorted = true),
            "shift([-MAX, MAX], [-MAX, MAX])",
        )
        assertBitwise(
            max,
            shift(listOf(0.0, max), listOf(-max, 0.0), assumeSorted = true),
            "shift([0, MAX], [-MAX, 0])",
        )
    }
}
