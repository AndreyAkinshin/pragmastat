package dev.pragmastat

/**
 * Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b (halve before summing; never overflows;
 * operand order is irrelevant).
 */
private fun midpoint(
    a: Double,
    b: Double,
): Double = 0.5 * a + 0.5 * b

/**
 * O(n log n) implementation of the Center (Hodges-Lehmann) estimator.
 * Based on Monahan's Algorithm 616 (1984).
 *
 * Internal implementation - not part of public API.
 */
internal fun centerImpl(
    values: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    val n = values.size
    require(n > 0) { "Input list cannot be empty" }
    if (n == 1) return values[0]
    if (n == 2) return midpoint(values[0], values[1])

    // Create deterministic RNG from input values
    val rng = Rng(deriveSeed(values))

    val sortedValues = if (assumeSorted) values else values.sorted()
    val totalPairs = (n.toLong() * (n + 1)) / 2
    val medianRankLow = (totalPairs + 1) / 2
    val medianRankHigh = (totalPairs + 2) / 2

    val leftBounds = IntArray(n) { it + 1 }
    val rightBounds = IntArray(n) { n }

    var pivot = sortedValues[(n - 1) / 2] + sortedValues[n / 2]
    var activeSetSize = totalPairs
    var previousCount = 0L

    // Bound the selection loop. On valid sorted input the Monahan selection
    // converges in O(log n) iterations; this cap is far higher than ever
    // needed for sorted input but guarantees termination on misuse (e.g.,
    // assumeSorted=true on UNSORTED input, which is undefined behavior and
    // would otherwise spin forever). The cap scales with n so large valid
    // inputs are never starved. We also track no-progress (stall) on the
    // active set to bail out deterministically.
    val baseIterations = 256
    val maxIterations = baseIterations + 4 * n
    var prevActiveSetSize = -1L
    var stallCount = 0
    val maxStall = 8
    var iterations = 0

    while (true) {
        if (iterations++ >= maxIterations) {
            throw IllegalStateException("Convergence failure (pathological input)")
        }

        var countBelowPivot = 0L
        var currentColumn = n
        val partitionCounts = IntArray(n)

        for (row in 1..n) {
            while (currentColumn >= row &&
                sortedValues[row - 1] + sortedValues[currentColumn - 1] >= pivot
            ) {
                currentColumn--
            }
            val elementsBelow = if (currentColumn >= row) currentColumn - row + 1 else 0
            partitionCounts[row - 1] = elementsBelow
            countBelowPivot += elementsBelow
        }

        if (countBelowPivot == previousCount) {
            var minActiveSum = Double.POSITIVE_INFINITY
            var maxActiveSum = Double.NEGATIVE_INFINITY

            for (i in 0 until n) {
                if (leftBounds[i] > rightBounds[i]) continue
                val rowValue = sortedValues[i]
                minActiveSum = minOf(minActiveSum, sortedValues[leftBounds[i] - 1] + rowValue)
                maxActiveSum = maxOf(maxActiveSum, sortedValues[rightBounds[i] - 1] + rowValue)
            }

            pivot = 0.5 * minActiveSum + 0.5 * maxActiveSum
            if (pivot <= minActiveSum || pivot > maxActiveSum) pivot = maxActiveSum
            if (minActiveSum == maxActiveSum || activeSetSize <= 2) return pivot / 2
            continue
        }

        val atTargetRank =
            countBelowPivot == medianRankLow ||
                countBelowPivot == medianRankHigh - 1

        if (atTargetRank) {
            var largestBelowPivot = Double.NEGATIVE_INFINITY
            var smallestAtOrAbovePivot = Double.POSITIVE_INFINITY

            for (i in 0 until n) {
                val countInRow = partitionCounts[i]
                val rowValue = sortedValues[i]
                val totalInRow = n - i

                if (countInRow > 0) {
                    val lastBelowValue = rowValue + sortedValues[i + countInRow - 1]
                    largestBelowPivot = maxOf(largestBelowPivot, lastBelowValue)
                }
                if (countInRow < totalInRow) {
                    val firstAtOrAboveValue = rowValue + sortedValues[i + countInRow]
                    smallestAtOrAbovePivot = minOf(smallestAtOrAbovePivot, firstAtOrAboveValue)
                }
            }

            return if (medianRankLow < medianRankHigh) {
                // Even total: average the two middle values. Overflow-safe: quarter each
                // pair-sum before summing (both operands can be near the double max).
                0.25 * smallestAtOrAbovePivot + 0.25 * largestBelowPivot
            } else {
                val needLargest = countBelowPivot == medianRankLow
                (if (needLargest) largestBelowPivot else smallestAtOrAbovePivot) / 2
            }
        }

        if (countBelowPivot < medianRankLow) {
            for (i in 0 until n) leftBounds[i] = i + partitionCounts[i] + 1
        } else {
            for (i in 0 until n) rightBounds[i] = i + partitionCounts[i]
        }

        previousCount = countBelowPivot
        activeSetSize = (0 until n).sumOf { maxOf(0, rightBounds[it] - leftBounds[it] + 1).toLong() }

        // Stall detection: on valid sorted input the active set strictly
        // shrinks toward the target. If it fails to shrink for several
        // consecutive iterations, the input is pathological (e.g.,
        // assumeSorted=true on unsorted data) and we bail deterministically.
        if (activeSetSize >= prevActiveSetSize && prevActiveSetSize >= 0) {
            stallCount++
            if (stallCount >= maxStall) {
                throw IllegalStateException("Convergence failure (pathological input)")
            }
        } else {
            stallCount = 0
        }
        prevActiveSetSize = activeSetSize

        if (activeSetSize > 2) {
            val targetIndex = rng.uniformLong(0, activeSetSize)
            var cumulativeSize = 0L
            var selectedRow = 0

            for (i in 0 until n) {
                val rowSize = maxOf(0, rightBounds[i] - leftBounds[i] + 1)
                if (rowSize > 0) {
                    if (targetIndex < cumulativeSize + rowSize) {
                        selectedRow = i
                        break
                    }
                    cumulativeSize += rowSize
                }
            }

            val medianColumnInRow = (leftBounds[selectedRow] + rightBounds[selectedRow]) / 2
            pivot = sortedValues[selectedRow] + sortedValues[medianColumnInRow - 1]
        } else {
            var minRemainingSum = Double.POSITIVE_INFINITY
            var maxRemainingSum = Double.NEGATIVE_INFINITY

            for (i in 0 until n) {
                if (leftBounds[i] > rightBounds[i]) continue
                val rowValue = sortedValues[i]
                minRemainingSum = minOf(minRemainingSum, sortedValues[leftBounds[i] - 1] + rowValue)
                maxRemainingSum = maxOf(maxRemainingSum, sortedValues[rightBounds[i] - 1] + rowValue)
            }

            pivot = 0.5 * minRemainingSum + 0.5 * maxRemainingSum
            if (pivot <= minRemainingSum || pivot > maxRemainingSum) pivot = maxRemainingSum
            if (minRemainingSum == maxRemainingSum) return pivot / 2
        }
    }
}
