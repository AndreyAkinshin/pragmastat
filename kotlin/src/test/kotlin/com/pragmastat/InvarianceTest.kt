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
    fun testVolatilityLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        
        // Volatility is NOT location invariant in general
        // because it's the ratio of spread to |center|
        // Only test when shift doesn't change sign of center
        val centerX = center(x)
        val centerXShifted = center(xShifted)
        
        // Test only if both centers have the same sign
        if (centerX * centerXShifted > 0) {
            // Calculate expected volatility after shift
            val spreadX = spread(x)
            val expectedVolatility = spreadX / abs(centerXShifted)
            assertClose(expectedVolatility, volatility(xShifted), tolerance = 1e-9)
        }
    }
    
    @Test
    fun testVolatilityScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        
        assertClose(volatility(x), volatility(xScaled))
    }
    
    @Test
    fun testPrecisionLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        
        assertClose(precision(x), precision(xShifted))
    }
    
    @Test
    fun testPrecisionScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        
        assertClose(precision(x) * scale, precision(xScaled))
    }
    
    @Test
    fun testMedShiftLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        val yShifted = y.map { it + shift }
        
        assertClose(medShift(x, y), medShift(xShifted, yShifted))
    }
    
    @Test
    fun testMedShiftScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }
        
        assertClose(medShift(x, y) * scale, medShift(xScaled, yScaled))
    }
    
    @Test
    fun testMedRatioLocationInvariance() {
        val x = listOf(11.0, 12.0, 13.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        val yShifted = y.map { it + shift }
        
        // MedRatio is NOT location invariant in general
        // We test that it changes with location shift
        val ratio1 = medRatio(x, y)
        val ratio2 = medRatio(xShifted, yShifted)
        assertTrue(abs(ratio1 - ratio2) > epsilon)
    }
    
    @Test
    fun testMedRatioScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }
        
        assertClose(medRatio(x, y), medRatio(xScaled, yScaled))
    }
    
    @Test
    fun testMedSpreadLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        val yShifted = y.map { it + shift }
        
        assertClose(medSpread(x, y), medSpread(xShifted, yShifted))
    }
    
    @Test
    fun testMedSpreadScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }
        
        assertClose(medSpread(x, y) * scale, medSpread(xScaled, yScaled))
    }
    
    @Test
    fun testMedDisparityLocationInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val shift = 10.0
        val xShifted = x.map { it + shift }
        val yShifted = y.map { it + shift }
        
        assertClose(medDisparity(x, y), medDisparity(xShifted, yShifted))
    }
    
    @Test
    fun testMedDisparityScaleInvariance() {
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(4.0, 5.0, 6.0)
        val scale = 2.5
        val xScaled = x.map { it * scale }
        val yScaled = y.map { it * scale }
        
        assertClose(medDisparity(x, y), medDisparity(xScaled, yScaled))
    }
}