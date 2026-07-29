package dev.pragmastat

import kotlin.test.Test
import kotlin.test.assertTrue

/**
 * [additiveCumulative] has no answer for a NaN, and both of its range comparisons are false for one, so
 * without an explicit guard it leaves the tail branch as a finite 0 or 1: an undefined input
 * answered rather than reported. The approximation it replaced propagated NaN, so losing that
 * would be a regression on behavior every port shares.
 *
 * The assertions compare PAYLOADS. `==` reads the two zeros as equal and every NaN as unequal,
 * neither of which is the claim being made here.
 */
class AdditiveCumulativeTest {
    @Test
    fun carriesNaNThroughAndAnswersBothInfinities() {
        assertTrue(additiveCumulative(Double.NaN).isNaN(), "additiveCumulative(NaN) should be NaN")
        assertBitwise(1.0, additiveCumulative(Double.POSITIVE_INFINITY), "additiveCumulative(+Inf)")
        assertBitwise(0.0, additiveCumulative(Double.NEGATIVE_INFINITY), "additiveCumulative(-Inf)")
        assertBitwise(0.5, additiveCumulative(0.0), "additiveCumulative(+0)")
        assertBitwise(0.5, additiveCumulative(-0.0), "additiveCumulative(-0)")
    }

    /**
     * The values outside the reduction band, which the shared fixture cannot carry: JSON has no
     * way to express an infinity, so the generated suite stops at 709.78 and these arguments are
     * covered here or nowhere.
     */
    @Test
    fun expFunctionReturnsTheDeclaredValuesOutsideTheReductionBand() {
        assertTrue(expFunction(Double.NaN).isNaN(), "expFunction(NaN) should be NaN")
        assertBitwise(Double.POSITIVE_INFINITY, expFunction(709.8), "expFunction(709.8)")
        assertBitwise(Double.POSITIVE_INFINITY, expFunction(Double.POSITIVE_INFINITY), "expFunction(+Inf)")
        assertBitwise(0.0, expFunction(-745.3), "expFunction(-745.3)")
        assertBitwise(0.0, expFunction(Double.NEGATIVE_INFINITY), "expFunction(-Inf)")
        assertBitwise(1.0, expFunction(0.0), "expFunction(0)")
    }
}
