package dev.pragmastat

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import kotlin.test.assertEquals

class RatioBoundsErrorPriorityTest {
    @Test
    fun `ratioBounds domain before positivity`() {
        // misrate=-0.1 is invalid (domain), x=-1 is non-positive (positivity)
        // domain(misrate) must take priority over positivity(x)
        val x = Sample.of(listOf(-1.0))
        val y = Sample.of(listOf(1.0))
        val exception =
            assertThrows<AssumptionException> {
                Sample.ratioBounds(x, y, -0.1)
            }
        assertEquals("domain", exception.violation!!.id.id)
        assertEquals("misrate", exception.violation!!.subject.id)
    }

    @Test
    fun `ratioBounds positivity when misrate valid`() {
        // Valid misrate but non-positive x -> positivity(x)
        val x = Sample.of(listOf(-1.0, -2.0, -3.0))
        val y = Sample.of(listOf(1.0, 2.0, 3.0))
        val exception =
            assertThrows<AssumptionException> {
                Sample.ratioBounds(x, y, 0.5)
            }
        assertEquals("positivity", exception.violation!!.id.id)
        assertEquals("x", exception.violation!!.subject.id)
    }
}
