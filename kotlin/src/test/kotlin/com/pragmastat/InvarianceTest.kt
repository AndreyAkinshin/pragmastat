package com.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.math.abs

class InvarianceTest {
    
    private val epsilon = 1e-10
    
    private fun assertClose(expected: Double, actual: Double, tolerance: Double = epsilon) {
        assertTrue(abs(expected - actual) < tolerance, 
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})")
    }
    
    @Test
    fun testCenterLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        
        assertClose(center(x) + shift, center(xShifted))
    }
    
    @Test
    fun testCenterScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        
        assertClose(center(x) * scale, center(xScaled))
    }
    
    @Test
    fun testSpreadLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        
        assertClose(spread(x), spread(xShifted))
    }
    
    @Test
    fun testSpreadScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        
        assertClose(spread(x) * scale, spread(xScaled))
    }
    
    @Test
    fun testRelSpreadLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }

        // RelSpread is NOT location invariant in general
        // because it's the ratio of spread to |center|
        // Only test when shift doesn't change sign of center
        val centerX = center(x)
        val centerXShifted = center(xShifted)

        // Test only if both centers have the same sign
        if (centerX * centerXShifted > 0) {
            // Calculate expected relSpread after shift
            val spreadX = spread(x)
            val expectedRelSpread = spreadX / abs(centerXShifted)
            assertClose(expectedRelSpread, relSpread(xShifted), tolerance = 1e-9)
        }
    }

    @Test
    fun testRelSpreadScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }

        assertClose(relSpread(x), relSpread(xScaled))
    }

    @Test
    fun testShiftLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        val yShifted = y.map { it + shift }

        assertClose(shift(x, y), shift(xShifted, yShifted))
    }

    @Test
    fun testShiftScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }

        assertClose(shift(x, y) * scale, shift(xScaled, yScaled))
    }

    @Test
    fun testRatioLocationInvariance() {
        val x = listOf(11.0, 12.0, 13.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shiftVal = 10.0
        val xShifted = x.map { it + shiftVal }
        val yShifted = y.map { it + shiftVal }

        // Ratio is NOT location invariant in general
        // We test that it changes with location shift
        val ratio1 = ratio(x, y)
        val ratio2 = ratio(xShifted, yShifted)
        assertTrue(abs(ratio1 - ratio2) > epsilon)
    }

    @Test
    fun testRatioScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }

        assertClose(ratio(x, y), ratio(xScaled, yScaled))
    }

    @Test
    fun testAvgSpreadLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shiftVal = 10.0
        val xShifted = x.map { it + shiftVal }
        val yShifted = y.map { it + shiftVal }

        assertClose(avgSpread(x, y), avgSpread(xShifted, yShifted))
    }

    @Test
    fun testAvgSpreadScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }

        assertClose(avgSpread(x, y) * scale, avgSpread(xScaled, yScaled))
    }

    @Test
    fun testDisparityLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shiftVal = 10.0
        val xShifted = x.map { it + shiftVal }
        val yShifted = y.map { it + shiftVal }

        assertClose(disparity(x, y), disparity(xShifted, yShifted))
    }

    @Test
    fun testDisparityScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }

        assertClose(disparity(x, y), disparity(xScaled, yScaled))
    }
}