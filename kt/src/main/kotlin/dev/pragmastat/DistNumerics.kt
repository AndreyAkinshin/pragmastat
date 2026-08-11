package dev.pragmastat

import kotlin.math.max
import kotlin.math.min

/**
 * Binary64 primitives shared by the distribution estimation models.
 *
 * Every operation here is written out explicitly so that all seven language
 * implementations evaluate the same graph and produce identical bits. None of
 * these helpers may be replaced by a fused, vectorized or library variant
 * without a new algorithm identifier.
 */
internal object DistNumerics {
    /**
     * Next representable binary64 value toward positive infinity.
     *
     * Implemented on the bit pattern rather than through [Math.nextUp], so that the
     * evaluation graph is visible and identical in every port.
     */
    fun nextUp(value: Double): Double {
        if (value.isNaN() || value == Double.POSITIVE_INFINITY) {
            return value
        }
        if (value == 0.0) {
            return Double.MIN_VALUE
        }

        val bits = value.toRawBits() + if (value > 0.0) 1L else -1L
        return Double.fromBits(bits)
    }

    /**
     * Next representable binary64 value toward negative infinity.
     */
    fun nextDown(value: Double): Double {
        if (value.isNaN() || value == Double.NEGATIVE_INFINITY) {
            return value
        }
        if (value == 0.0) {
            return -Double.MIN_VALUE
        }

        val bits = value.toRawBits() + if (value > 0.0) -1L else 1L
        return Double.fromBits(bits)
    }

    /**
     * Overflow-safe midpoint of two finite values.
     */
    fun midpoint(
        a: Double,
        b: Double,
    ): Double {
        if (a < 0.0 && 0.0 < b) {
            return a / 2.0 + b / 2.0
        }
        return a + (b - a) / 2.0
    }

    /**
     * Interpolates between [a] and [b], reproducing both endpoints exactly.
     */
    fun lerp(
        a: Double,
        b: Double,
        t: Double,
    ): Double {
        if (t == 0.0) {
            return a
        }
        if (t == 1.0) {
            return b
        }
        if (a < 0.0 && 0.0 < b) {
            return (1.0 - t) * a + t * b
        }
        return a + t * (b - a)
    }

    /**
     * Ratio clamped to the unit interval. Callers must have proved 0 <= num <= den and den > 0.
     */
    fun unitRatio(
        num: Double,
        den: Double,
    ): Double {
        val t = num / den
        if (t.isNaN()) {
            throw IllegalStateException("Numerical conformance failure: unitRatio produced NaN")
        }
        return min(1.0, max(0.0, t))
    }

    /**
     * Smallest integer k with k >= ceil(n / k), computed without forming k*k.
     */
    fun ceilSqrt(n: Int): Int {
        var k = 1
        while (k < (n + k - 1) / k) {
            k++
        }
        return k
    }

    /**
     * First index i in [from, to) with values[i] >= key, over an ascending array.
     */
    fun lowerBound(
        values: DoubleArray,
        from: Int,
        to: Int,
        key: Double,
    ): Int {
        var lo = from
        var hi = to
        while (lo < hi) {
            val mid = lo + (hi - lo) / 2
            if (values[mid] < key) {
                lo = mid + 1
            } else {
                hi = mid
            }
        }
        return lo
    }

    /**
     * First index i with values[i] > key, over an ascending array.
     */
    fun upperBound(
        values: DoubleArray,
        key: Double,
    ): Int {
        var lo = 0
        var hi = values.size
        while (lo < hi) {
            val mid = lo + (hi - lo) / 2
            if (values[mid] <= key) {
                lo = mid + 1
            } else {
                hi = mid
            }
        }
        return lo
    }
}
