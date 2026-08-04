package dev.pragmastat

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import kotlin.test.assertEquals

/**
 * [MeasurementUnit]'s `init` block refuses malformed unit definitions. A non-positive
 * [MeasurementUnit.baseUnits] poisons every conversion through the unit and makes [finer]
 * prefer it over any well-formed unit; an empty [MeasurementUnit.family] makes two unrelated
 * units compatible, because [MeasurementUnit.isCompatible] compares families.
 * An empty [MeasurementUnit.abbreviation] stays legal: the three dimensionless units have none.
 */
class MeasurementUnitTest {
    @Test
    fun rejectsEmptyId() {
        val ex =
            assertThrows<IllegalArgumentException> {
                MeasurementUnit("", "Time", "ms", "Millisecond", 1_000_000)
            }
        assertEquals("id cannot be empty", ex.message)
    }

    @Test
    fun rejectsEmptyFamily() {
        val ex =
            assertThrows<IllegalArgumentException> {
                MeasurementUnit("ms", "", "ms", "Millisecond", 1_000_000)
            }
        assertEquals("family cannot be empty", ex.message)
    }

    @Test
    fun rejectsZeroBaseUnits() {
        val ex =
            assertThrows<IllegalArgumentException> {
                MeasurementUnit("ms", "Time", "ms", "Millisecond", 0)
            }
        assertEquals("baseUnits must be positive", ex.message)
    }

    @Test
    fun rejectsNegativeBaseUnits() {
        val ex =
            assertThrows<IllegalArgumentException> {
                MeasurementUnit("ms", "Time", "ms", "Millisecond", -1)
            }
        assertEquals("baseUnits must be positive", ex.message)
    }

    @Test
    fun acceptsWellFormedUnit() {
        val ms = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)
        assertEquals("ms", ms.id)
        assertEquals("Time", ms.family)
        assertEquals("ms", ms.abbreviation)
        assertEquals("Millisecond", ms.fullName)
        assertEquals(1_000_000L, ms.baseUnits)
    }

    @Test
    fun dimensionlessUnitsKeepTheirEmptyAbbreviation() {
        val units = listOf("number" to NumberUnit, "ratio" to RatioUnit, "disparity" to DisparityUnit)
        for ((id, unit) in units) {
            assertEquals(id, unit.id)
            assertEquals("", unit.abbreviation)
            assertEquals(1L, unit.baseUnits)
        }
    }
}
