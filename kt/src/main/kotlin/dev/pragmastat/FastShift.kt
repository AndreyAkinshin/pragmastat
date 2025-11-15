package dev.pragmastat

import kotlin.math.ceil
import kotlin.math.floor

/**
 * Fast O((m + n) * log(precision)) implementation of the Shift estimator.
 * Computes quantiles of all pairwise differences { x_i - y_j } without materializing them.
 *
 * Internal implementation - not part of public API.
 */
internal fun fastShift(
    x: List<Double>,
    y: List<Double>,
    probabilities: DoubleArray = doubleArrayOf(0.5),
    assumeSorted: Boolean = false
): DoubleArray {
    require(x.isNotEmpty() && y.isNotEmpty()) { "Input lists cannot be empty" }

    // Validate probabilities
    for (p in probabilities) {
        require(!p.isNaN() && p in 0.0..1.0) { "Probabilities must be within [0, 1]" }
    }

    // Sort inputs if needed
    val xs = if (assumeSorted) x else x.sorted()
    val ys = if (assumeSorted) y else y.sorted()

    val m = xs.size
    val n = ys.size
    val total = m.toLong() * n.toLong()

    // Type-7 quantile: h = 1 + (total-1)*p, then interpolate between floor(h) and ceil(h)
    val requiredRanks = mutableSetOf<Long>()
    val interpolationParams = Array(probabilities.size) { i ->
        val p = probabilities[i]
        val h = 1.0 + (total - 1) * p
        var lowerRank = floor(h).toLong()
        var upperRank = ceil(h).toLong()
        val weight = h - lowerRank

        if (lowerRank < 1) lowerRank = 1
        if (upperRank > total) upperRank = total

        requiredRanks.add(lowerRank)
        requiredRanks.add(upperRank)

        Triple(lowerRank, upperRank, weight)
    }

    // Find all required rank values
    val rankValues = mutableMapOf<Long, Double>()
    for (rank in requiredRanks) {
        rankValues[rank] = selectKthPairwiseDiff(xs, ys, rank)
    }

    // Compute final quantiles using interpolation
    return DoubleArray(probabilities.size) { i ->
        val (lowerRank, upperRank, weight) = interpolationParams[i]
        val lower = rankValues[lowerRank]!!
        val upper = rankValues[upperRank]!!

        if (weight == 0.0) lower
        else (1.0 - weight) * lower + weight * upper
    }
}

/**
 * Binary search in [min_diff, max_diff] that snaps to actual discrete values.
 * Avoids materializing all m*n differences.
 */
internal fun selectKthPairwiseDiff(x: List<Double>, y: List<Double>, k: Long): Double {
    val m = x.size
    val n = y.size
    val total = m.toLong() * n.toLong()

    require(k in 1..total) { "k must be within [1, $total]" }

    var searchMin = x[0] - y[n - 1]
    var searchMax = x[m - 1] - y[0]

    if (searchMin.isNaN() || searchMax.isNaN()) {
        throw IllegalStateException("NaN in input values")
    }

    val maxIterations = 128 // Sufficient for double precision convergence
    var prevMin = Double.NEGATIVE_INFINITY
    var prevMax = Double.POSITIVE_INFINITY

    repeat(maxIterations) {
        if (searchMin == searchMax) return searchMin

        val mid = midpoint(searchMin, searchMax)
        val (countLessOrEqual, closestBelow, closestAbove) = countAndNeighbors(x, y, mid)

        if (closestBelow == closestAbove) {
            return closestBelow
        }

        // No progress means we're stuck between two discrete values
        if (searchMin == prevMin && searchMax == prevMax) {
            return if (countLessOrEqual >= k) closestBelow else closestAbove
        }

        prevMin = searchMin
        prevMax = searchMax

        if (countLessOrEqual >= k) {
            searchMax = closestBelow
        } else {
            searchMin = closestAbove
        }
    }

    if (searchMin != searchMax) {
        throw IllegalStateException("Convergence failure (pathological input)")
    }

    return searchMin
}

/**
 * Two-pointer algorithm: counts pairs where x[i] - y[j] <= threshold, and tracks
 * the closest actual differences on either side of threshold.
 *
 * Returns Triple(countLessOrEqual, closestBelow, closestAbove)
 */
private fun countAndNeighbors(
    x: List<Double>,
    y: List<Double>,
    threshold: Double
): Triple<Long, Double, Double> {
    val m = x.size
    val n = y.size
    var count = 0L
    var maxBelow = Double.NEGATIVE_INFINITY
    var minAbove = Double.POSITIVE_INFINITY

    var j = 0
    for (i in 0 until m) {
        while (j < n && x[i] - y[j] > threshold) {
            j++
        }

        count += (n - j)

        if (j < n) {
            val diff = x[i] - y[j]
            if (diff > maxBelow) maxBelow = diff
        }

        if (j > 0) {
            val diff = x[i] - y[j - 1]
            if (diff < minAbove) minAbove = diff
        }
    }

    // Fallback to actual min/max if no boundaries found (shouldn't happen in normal operation)
    if (maxBelow.isInfinite() && maxBelow < 0) {
        maxBelow = x[0] - y[n - 1]
    }
    if (minAbove.isInfinite() && minAbove > 0) {
        minAbove = x[m - 1] - y[0]
    }

    return Triple(count, maxBelow, minAbove)
}

private fun midpoint(a: Double, b: Double): Double = a + (b - a) * 0.5
