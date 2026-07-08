package dev.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class PairwiseMarginConsistencyTest {
    // Regression: the exact binomial coefficient must use integer arithmetic. Float
    // accumulation overflowed 2^53 in the partial products for C(56,27), giving a
    // margin of 784 instead of the correct 782 at misrate 1.0.
    @Test
    fun `pairwise margin matches exact-integer binomial`() {
        assertEquals(782, pairwiseMargin(29, 27, 1.0))
    }
}
