package dev.pragmastat

import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min

/**
 * Efficiently computes quantiles from all pairwise averages (x[i] + x[j]) / 2 for i <= j.
 * Uses binary search with counting function to avoid materializing all N(N+1)/2 pairs.
 *
 * Internal implementation - not part of public API.
 */
internal object FastCenterQuantiles {
    /** Relative epsilon for floating-point comparisons in binary search convergence. */
    private const val RELATIVE_EPSILON = 1e-14

    /**
     * Compute both lower and upper bounds from pairwise averages.
     *
     * @param sorted Sorted input array
     * @param marginLo Rank of lower bound (1-based)
     * @param marginHi Rank of upper bound (1-based)
     * @return Pair of (lower, upper) quantiles
     */
    fun bounds(sorted: List<Double>, marginLo: Long, marginHi: Long): Pair<Double, Double> {
        val n = sorted.size
        val totalPairs = n.toLong() * (n + 1) / 2

        val clampedLo = max(1, min(marginLo, totalPairs))
        val clampedHi = max(1, min(marginHi, totalPairs))

        val lo = findExactQuantile(sorted, clampedLo)
        val hi = findExactQuantile(sorted, clampedHi)

        return Pair(min(lo, hi), max(lo, hi))
    }

    /**
     * Count pairwise averages <= target value.
     */
    private fun countPairsLessOrEqual(sorted: List<Double>, target: Double): Long {
        val n = sorted.size
        var count = 0L
        // j is not reset: as i increases, threshold decreases monotonically
        var j = n - 1

        for (i in 0 until n) {
            val threshold = 2 * target - sorted[i]

            while (j >= 0 && sorted[j] > threshold) {
                j--
            }

            if (j >= i) {
                count += j - i + 1
            }
        }

        return count
    }

    /**
     * Find exact k-th pairwise average using selection algorithm.
     */
    private fun findExactQuantile(sorted: List<Double>, k: Long): Double {
        val n = sorted.size
        val totalPairs = n.toLong() * (n + 1) / 2

        if (n == 1) {
            return sorted[0]
        }

        if (k == 1L) {
            return sorted[0]
        }

        if (k == totalPairs) {
            return sorted[n - 1]
        }

        var lo = sorted[0]
        var hi = sorted[n - 1]
        val eps = RELATIVE_EPSILON

        while (hi - lo > eps * max(1.0, max(abs(lo), abs(hi)))) {
            val mid = (lo + hi) / 2
            val countLessOrEqual = countPairsLessOrEqual(sorted, mid)

            if (countLessOrEqual >= k) {
                hi = mid
            } else {
                lo = mid
            }
        }

        val target = (lo + hi) / 2
        val candidates = ArrayList<Double>(n)

        for (i in 0 until n) {
            val threshold = 2 * target - sorted[i]

            var left = i
            var right = n

            while (left < right) {
                val m = (left + right) / 2
                if (sorted[m] < threshold - eps) {
                    left = m + 1
                } else {
                    right = m
                }
            }

            if (left < n && left >= i && abs(sorted[left] - threshold) < eps * max(1.0, abs(threshold))) {
                candidates.add((sorted[i] + sorted[left]) / 2)
            }

            if (left > i) {
                val avgBefore = (sorted[i] + sorted[left - 1]) / 2
                if (avgBefore <= target + eps) {
                    candidates.add(avgBefore)
                }
            }
        }

        if (candidates.isEmpty()) {
            return target
        }

        candidates.sort()

        for (candidate in candidates) {
            val countAtCandidate = countPairsLessOrEqual(sorted, candidate)
            if (countAtCandidate >= k) {
                return candidate
            }
        }

        return target
    }
}
