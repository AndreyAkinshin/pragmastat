package dev.pragmastat

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import kotlin.test.assertEquals

/**
 * The misrate out-of-[0,1] / NaN domain branch lives on the RAW (native List,
 * plain [Double] misrate) bounds entry points. The typed Sample path funnels
 * misrate through [Probability], which rejects out-of-range values before the
 * estimator is even reached, so this branch is only observable via the raw API.
 *
 * Asserts: the raw bounds API rejects misrate = 2.0, -0.1, and NaN with the
 * domain/misrate [AssumptionException] (id=domain, subject=misrate), for both a
 * one-sample ([centerBounds]) and a two-sample ([shiftBounds]) estimator.
 */
class RawMisrateDomainTest {
    private val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)
    private val y = listOf(9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0)

    private val badMisrates = listOf(2.0, -0.1, Double.NaN)

    private fun assertDomainMisrate(block: () -> Unit) {
        val ex = assertThrows<AssumptionException> { block() }
        assertEquals("domain", ex.violation!!.id.id)
        assertEquals("misrate", ex.violation!!.subject.id)
    }

    @Test
    fun rawCenterBoundsRejectsOutOfRangeMisrate() {
        for (m in badMisrates) {
            assertDomainMisrate { centerBounds(x, m) }
        }
    }

    @Test
    fun rawShiftBoundsRejectsOutOfRangeMisrate() {
        for (m in badMisrates) {
            assertDomainMisrate { shiftBounds(x, y, m) }
        }
    }
}
