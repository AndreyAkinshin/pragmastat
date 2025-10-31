package dev.pragmastat

import kotlin.math.abs
import kotlin.random.Random

/**
 * Fast O(n log n) implementation of the Spread (Shamos) estimator.
 * Based on Monahan's selection algorithm adapted for pairwise differences.
 *
 * Internal implementation - not part of public API.
 */
internal fun fastSpread(values: List<Double>): Double {
    val n = values.size
    require(n > 0) { "Input list cannot be empty" }
    if (n == 1) return 0.0
    if (n == 2) return abs(values[1] - values[0])

    val a = values.sorted()
    val N = (n.toLong() * (n - 1)) / 2
    val kLow = (N + 1) / 2
    val kHigh = (N + 2) / 2

    val L = IntArray(n) { i -> minOf(i + 1, n) }
    val R = IntArray(n) { n - 1 }

    for (i in 0 until n) {
        if (L[i] > R[i]) {
            L[i] = 1
            R[i] = 0
        }
    }

    val rowCounts = IntArray(n)
    var pivot = a[n / 2] - a[(n - 1) / 2]
    var prevCountBelow = -1L

    while (true) {
        var countBelow = 0L
        var largestBelow = Double.NEGATIVE_INFINITY
        var smallestAtOrAbove = Double.POSITIVE_INFINITY

        var j = 1
        for (i in 0 until n - 1) {
            if (j < i + 1) j = i + 1
            while (j < n && a[j] - a[i] < pivot) j++

            val cntRow = maxOf(0, j - (i + 1))
            rowCounts[i] = cntRow
            countBelow += cntRow

            if (cntRow > 0) largestBelow = maxOf(largestBelow, a[j - 1] - a[i])
            if (j < n) smallestAtOrAbove = minOf(smallestAtOrAbove, a[j] - a[i])
        }

        val atTarget = countBelow == kLow || countBelow == kHigh - 1

        if (atTarget) {
            return if (kLow < kHigh) {
                0.5 * (largestBelow + smallestAtOrAbove)
            } else {
                if (countBelow == kLow) largestBelow else smallestAtOrAbove
            }
        }

        if (countBelow == prevCountBelow) {
            var minActive = Double.POSITIVE_INFINITY
            var maxActive = Double.NEGATIVE_INFINITY
            var active = 0

            for (i in 0 until n - 1) {
                if (L[i] > R[i]) continue
                minActive = minOf(minActive, a[L[i]] - a[i])
                maxActive = maxOf(maxActive, a[R[i]] - a[i])
                active += R[i] - L[i] + 1
            }

            if (active <= 0) {
                return if (kLow < kHigh) {
                    0.5 * (largestBelow + smallestAtOrAbove)
                } else {
                    if (countBelow >= kLow) largestBelow else smallestAtOrAbove
                }
            }

            if (maxActive <= minActive) return minActive

            val mid = 0.5 * (minActive + maxActive)
            pivot = if (mid > minActive && mid <= maxActive) mid else maxActive
            prevCountBelow = countBelow
            continue
        }

        if (countBelow < kLow) {
            for (i in 0 until n - 1) {
                val newL = i + 1 + rowCounts[i]
                if (newL > L[i]) L[i] = newL
                if (L[i] > R[i]) {
                    L[i] = 1
                    R[i] = 0
                }
            }
        } else {
            for (i in 0 until n - 1) {
                val newR = i + rowCounts[i]
                if (newR < R[i]) R[i] = newR
                if (R[i] < i + 1) {
                    L[i] = 1
                    R[i] = 0
                }
            }
        }

        prevCountBelow = countBelow

        val activeSize = (0 until n - 1).filter { L[it] <= R[it] }.sumOf { (R[it] - L[it] + 1).toLong() }

        if (activeSize <= 2) {
            var minRem = Double.POSITIVE_INFINITY
            var maxRem = Double.NEGATIVE_INFINITY
            for (i in 0 until n - 1) {
                if (L[i] > R[i]) continue
                minRem = minOf(minRem, a[L[i]] - a[i])
                maxRem = maxOf(maxRem, a[R[i]] - a[i])
            }

            if (activeSize <= 0) {
                return if (kLow < kHigh) {
                    0.5 * (largestBelow + smallestAtOrAbove)
                } else {
                    if (countBelow >= kLow) largestBelow else smallestAtOrAbove
                }
            }

            return if (kLow < kHigh) {
                0.5 * (minRem + maxRem)
            } else {
                if (abs((kLow - 1) - countBelow) <= abs(countBelow - kLow)) minRem else maxRem
            }
        } else {
            val t = Random.nextLong(activeSize)
            var acc = 0L
            var row = 0
            for (r in 0 until n - 1) {
                if (L[r] > R[r]) continue
                val size = R[r] - L[r] + 1
                if (t < acc + size) {
                    row = r
                    break
                }
                acc += size
            }

            val col = (L[row] + R[row]) / 2
            pivot = a[col] - a[row]
        }
    }
}
