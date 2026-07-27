package dev.pragmastat

import org.junit.jupiter.api.Test

/**
 * Pins exact order symmetry of the n==2 center midpoint.
 *
 * For n==2 the center is the midpoint of the two values, computed as
 * `0.5 * a + 0.5 * b`: overflow-safe AND order-independent, the same form the
 * Go and Rust implementations use. This guard exists because the obvious
 * alternative overflow-safe formula `a + (b - a) * 0.5` is ORDER-DEPENDENT:
 * for a=-5.0, b=-1.8 it yields exactly -3.4, but for the reversed order
 * (a=-1.8, b=-5.0) it yields -3.4000000000000004 (a 1-ULP discrepancy). Nobody
 * gets to swap such a form in without this test going red.
 *
 * assumeSorted=true is required so the midpoint sees the RAW argument order; the
 * normalizing sort would otherwise hide the asymmetry.
 *
 * The comparison is BITWISE, through [assertBitwise]. The whole point of this
 * guard is a ONE-ULP difference between two orderings of the same two values,
 * which is exactly what any tolerance absorbs: an epsilon here would accept the
 * order-dependent formula the test exists to reject. Bitwise also pins the sign
 * of zero, which a primitive `==` would not.
 */
class CenterMidpointSymmetryTest {
    @Test
    fun centerN2MidpointIsOrderSymmetric() {
        val forward = center(listOf(-5.0, -1.8), assumeSorted = true)
        val reversed = center(listOf(-1.8, -5.0), assumeSorted = true)

        assertBitwise(forward, reversed, "center([-5.0, -1.8]) vs center([-1.8, -5.0])")
        assertBitwise(-3.4, forward, "center([-5.0, -1.8])")
        assertBitwise(-3.4, reversed, "center([-1.8, -5.0])")
    }
}
