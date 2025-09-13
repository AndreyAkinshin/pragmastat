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
    fun testRelSpread() {
        assertEquals(0.0, relSpread(listOf(2.0)))
        assertFailsWith<IllegalArgumentException> { relSpread(listOf(0.0, 0.0)) }

        // Test with known values
        val x = listOf(10.0, 20.0)
        assertClose(10.0 / 15.0, relSpread(x))
    }

    @Test
    fun testShift() {
        assertFailsWith<IllegalArgumentException> { shift(emptyList(), listOf(1.0)) }
        assertFailsWith<IllegalArgumentException> { shift(listOf(1.0), emptyList()) }

        assertEquals(2.0, shift(listOf(3.0), listOf(1.0)))
        assertEquals(-2.0, shift(listOf(1.0), listOf(3.0)))

        // Test with multiple values
        val x = listOf(4.0, 5.0, 6.0)
        val y = listOf(1.0, 2.0, 3.0)
        assertClose(3.0, shift(x, y))
    }

    @Test
    fun testRatio() {
        assertFailsWith<IllegalArgumentException> { ratio(emptyList(), listOf(1.0)) }
        assertFailsWith<IllegalArgumentException> { ratio(listOf(1.0), emptyList()) }

        assertEquals(2.0, ratio(listOf(4.0), listOf(2.0)))
        assertEquals(0.5, ratio(listOf(2.0), listOf(4.0)))

        // Test with zero in denominator
        assertFailsWith<IllegalArgumentException> { ratio(listOf(1.0, 2.0), listOf(0.0)) }

        // Test with multiple values
        val x = listOf(4.0, 6.0, 8.0)
        val y = listOf(2.0, 3.0, 4.0)
        assertClose(2.0, ratio(x, y))
    }

    @Test
    fun testAvgSpread() {
        assertFailsWith<IllegalArgumentException> { avgSpread(emptyList(), emptyList()) }

        // Test with identical samples
        val x = listOf(1.0, 2.0, 3.0)
        val y = listOf(1.0, 2.0, 3.0)
        val spreadX = spread(x)
        assertClose(spreadX, avgSpread(x, y))

        // Test with different samples
        val a = listOf(1.0, 3.0)
        val b = listOf(2.0, 6.0)
        val spreadA = spread(a)
        val spreadB = spread(b)
        val expected = (2.0 * spreadA + 2.0 * spreadB) / 4.0
        assertClose(expected, avgSpread(a, b))
    }

    @Test
    fun testDisparity() {
        assertFailsWith<IllegalArgumentException> { disparity(emptyList(), emptyList()) }

        // Test with zero spread
        val x = listOf(1.0)
        val y = listOf(2.0)
        assertEquals(Double.POSITIVE_INFINITY, disparity(x, y))

        // Test normal case
        val a = listOf(1.0, 2.0, 3.0)
        val b = listOf(4.0, 5.0, 6.0)
        val shiftVal = shift(a, b)
        val spreadVal = avgSpread(a, b)
        assertClose(shiftVal / spreadVal, disparity(a, b))
    }
}