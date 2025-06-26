package com.pragmastat

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertFailsWith
import kotlin.math.abs

class EstimatorsTest {
    
    private val epsilon = 1e-10
    
    private fun assertClose(expected: Double, actual: Double, tolerance: Double = epsilon) {
        assertTrue(abs(expected - actual) < tolerance, 
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})")
    }
    
    @Test
    fun testMedian() {
        assertFailsWith<IllegalArgumentException> { median(emptyList()) }
        assertEquals(1.0, median(listOf(1.0)))
        assertEquals(1.5, median(listOf(1.0, 2.0)))
        assertEquals(2.0, median(listOf(1.0, 2.0, 3.0)))
        assertEquals(2.5, median(listOf(1.0, 2.0, 3.0, 4.0)))
    }
    
    @Test
    fun testCenter() {
        assertFailsWith<IllegalArgumentException> { center(emptyList()) }
        assertEquals(1.0, center(listOf(1.0)))
        assertEquals(2.0, center(listOf(1.0, 3.0)))
        assertClose(2.0, center(listOf(1.0, 2.0, 3.0)))
        
        // Test with known values
        val x = listOf(10.0, 20.0)
        assertClose(15.0, center(x))
    }
    
    @Test
    fun testSpread() {
        assertFailsWith<IllegalArgumentException> { spread(emptyList()) }
        assertEquals(0.0, spread(listOf(1.0)))
        assertEquals(2.0, spread(listOf(1.0, 3.0)))
        
        // Test with known values
        val x = listOf(10.0, 20.0)
        assertClose(10.0, spread(x))
    }
    
    @Test
    fun testVolatility() {
        assertEquals(0.0, volatility(listOf(2.0)))
        assertFailsWith<IllegalArgumentException> { volatility(listOf(0.0, 0.0)) }
        
        // Test with known values
        val x = listOf(10.0, 20.0)
        assertClose(10.0 / 15.0, volatility(x))
    }
    
    @Test
    fun testPrecision() {
        assertFailsWith<IllegalArgumentException> { precision(emptyList()) }
        
        // Test with specific examples
        val x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
        val spreadVal = spread(x)
        val expected = 2.0 * spreadVal / kotlin.math.sqrt(5.0)
        assertClose(expected, precision(x))
    }
    
    @Test
    fun testMedShift() {
        assertFailsWith<IllegalArgumentException> { medShift(emptyList(), listOf(1.0)) }
        assertFailsWith<IllegalArgumentException> { medShift(listOf(1.0), emptyList()) }
        
        assertEquals(2.0, medShift(listOf(3.0), listOf(1.0)))
        assertEquals(-2.0, medShift(listOf(1.0), listOf(3.0)))
        
        // Test with multiple values
        val x = listOf(4.0, 5.0, 6.0)
        val y = listOf(1.0, 2.0, 3.0)
        assertClose(3.0, medShift(x, y))
    }
    
    @Test
    fun testMedRatio() {
        assertFailsWith<IllegalArgumentException> { medRatio(emptyList(), listOf(1.0)) }
        assertFailsWith<IllegalArgumentException> { medRatio(listOf(1.0), emptyList()) }
        
        assertEquals(2.0, medRatio(listOf(4.0), listOf(2.0)))
        assertEquals(0.5, medRatio(listOf(2.0), listOf(4.0)))
        
        // Test with zero in denominator
        assertFailsWith<IllegalArgumentException> { medRatio(listOf(1.0, 2.0), listOf(0.0)) }
        
        // Test with multiple values
        val x = listOf(4.0, 6.0, 8.0)
        val y = listOf(2.0, 3.0, 4.0)
        assertClose(2.0, medRatio(x, y))
    }
    
    @Test
    fun testMedSpread() {
        assertFailsWith<IllegalArgumentException> { medSpread(emptyList(), emptyList()) }
        
        // Test with identical samples
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(1.0, 2.0, 3.0)
        val spreadX = spread(x)
        assertClose(spreadX, medSpread(x, y))
        
        // Test with different samples
        val a = listOf(1.0, 3.0)
        val b = listOf(2.0, 6.0)
        val spreadA = spread(a)
        val spreadB = spread(b)
        val expected = (2.0 * spreadA + 2.0 * spreadB) / 4.0
        assertClose(expected, medSpread(a, b))
    }
    
    @Test
    fun testMedDisparity() {
        assertFailsWith<IllegalArgumentException> { medDisparity(emptyList(), emptyList()) }
        
        // Test with zero spread
        val x = listOf(1.0)
        val y = listOf(2.0)
        assertEquals(Double.POSITIVE_INFINITY, medDisparity(x, y))
        
        // Test normal case
        val a = listOf(1.0, 2.0, 3.0)
        val b = listOf(4.0, 5.0, 6.0)
        val shift = medShift(a, b)
        val spread = medSpread(a, b)
        assertClose(shift / spread, medDisparity(a, b))
    }
}