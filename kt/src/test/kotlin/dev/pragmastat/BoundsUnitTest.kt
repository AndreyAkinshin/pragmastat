package dev.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/**
 * Covers the Sample-path bounds unit re-attachment (the `.withUnit(...)` calls in
 * [Sample]) and the contract that the RAW (native List) bounds API returns a
 * unitless / [NumberUnit] unit.
 *
 * Sample-path units propagate as:
 *   - centerBounds / spreadBounds -> x.unit
 *   - shiftBounds                 -> finer(x, y)
 *   - ratioBounds                 -> RatioUnit
 *   - disparityBounds             -> DisparityUnit
 *
 * The raw List-based overloads carry no unit semantics: their [Bounds.unit]
 * defaults to [NumberUnit].
 */
class BoundsUnitTest {
    private val sec = MeasurementUnit("s", "Time", "s", "Second", 1_000_000_000)
    private val ms = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)

    // Strictly positive so ratio is defined; n=8 large enough for misrate=0.3.
    private val xValues = listOf(5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0)
    private val yValues = listOf(12.0, 9.0, 15.0, 10.0, 13.0, 11.0, 16.0, 14.0)
    private val misrate = 0.3
    private val seed = "bounds-unit"

    // --- Sample-path: ratio/disparity re-attach their dedicated units ---

    @Test
    fun ratioBoundsSampleUnitIsRatio() {
        val x = Sample.of(xValues, sec)
        val y = Sample.of(yValues, sec)
        assertEquals(RatioUnit, ratioBounds(x, y, Probability(misrate)).unit)
    }

    @Test
    fun disparityBoundsSampleUnitIsDisparity() {
        val x = Sample.of(xValues, sec)
        val y = Sample.of(yValues, sec)
        assertEquals(DisparityUnit, disparityBounds(x, y, Probability(misrate), seed).unit)
    }

    // --- Sample-path: center/spread propagate x.unit, shift the finer(x, y) ---

    @Test
    fun centerBoundsSampleUnitIsXUnit() {
        val x = Sample.of(xValues, sec)
        assertEquals(sec, centerBounds(x, Probability(misrate)).unit)
    }

    @Test
    fun spreadBoundsSampleUnitIsXUnit() {
        val x = Sample.of(xValues, sec)
        assertEquals(sec, spreadBounds(x, Probability(misrate), seed).unit)
    }

    @Test
    fun shiftBoundsSampleUnitIsFiner() {
        // ms (baseUnits=1e6) is finer than sec (baseUnits=1e9).
        val x = Sample.of(xValues, sec)
        val y = Sample.of(yValues, ms)
        assertEquals(ms, shiftBounds(x, y, Probability(misrate)).unit)
    }

    // --- Raw (native List) bounds are unitless (NumberUnit) ---

    @Test
    fun rawCenterBoundsIsUnitless() {
        assertEquals(NumberUnit, centerBounds(xValues, misrate).unit)
    }

    @Test
    fun rawSpreadBoundsIsUnitless() {
        assertEquals(NumberUnit, spreadBounds(xValues, misrate, seed).unit)
    }

    @Test
    fun rawShiftBoundsIsUnitless() {
        assertEquals(NumberUnit, shiftBounds(xValues, yValues, misrate).unit)
    }

    @Test
    fun rawRatioBoundsIsUnitless() {
        assertEquals(NumberUnit, ratioBounds(xValues, yValues, misrate).unit)
    }

    @Test
    fun rawDisparityBoundsIsUnitless() {
        assertEquals(NumberUnit, disparityBounds(xValues, yValues, misrate, seed).unit)
    }
}
