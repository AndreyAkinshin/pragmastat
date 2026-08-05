package dev.pragmastat

import kotlin.test.Test
import kotlin.test.assertTrue

/**
 * The arguments outside the reduction's domain, which the shared log-function fixture cannot
 * carry: JSON has no way to write a negative infinity or a NaN, so the generated suite holds only
 * the positive finite range and these are covered here or nowhere.
 *
 * These are the cases IEEE 754 leaves a platform no freedom about, so agreeing on them is not a
 * choice this toolkit makes; it is what makes the replacement a logarithm at all.
 *
 * The assertions compare PAYLOADS, for the reason [AdditiveCumulativeTest] gives: `==` reads the
 * two zeros as equal and every NaN as unequal, so it answers a different question in both
 * directions.
 */
class LogFunctionTest {
    @Test
    fun returnsTheDeclaredValuesOutsideTheReductionDomain() {
        assertBitwise(Double.NEGATIVE_INFINITY, logFunction(0.0), "logFunction(+0)")
        assertBitwise(Double.NEGATIVE_INFINITY, logFunction(-0.0), "logFunction(-0)")
        assertTrue(logFunction(-1.0).isNaN(), "logFunction(-1) should be NaN")
        assertTrue(logFunction(Double.NaN).isNaN(), "logFunction(NaN) should be NaN")
        assertBitwise(Double.POSITIVE_INFINITY, logFunction(Double.POSITIVE_INFINITY), "logFunction(+Inf)")
    }

    /**
     * Exactly +0, not -0: ratio exponentiates this result, and the two zeros are distinct doubles
     * that the payload comparison in every other suite would tell apart.
     */
    @Test
    fun returnsPositiveZeroAtOne() {
        assertBitwise(0.0, logFunction(1.0), "logFunction(1)")
    }
}
