package com.pragmastat

import kotlin.math.floor
import kotlin.random.Random

/**
 * Fast O(n log n) implementation of the Center (Hodges-Lehmann) estimator.
 * Based on Monahan's Algorithm 616 (1984).
 *
 * Internal implementation - not part of public API.
 */
internal fun fastCenter(values: List<Double>): Double {
    val n = values.size
    require(n > 0) { "Input list cannot be empty" }
    if (n == 1) return values[0]
    if (n == 2) return (values[0] + values[1]) / 2.0

    val sortedValues = values.sorted()
    val totalPairs = (n * (n + 1)) / 2
    val medianRankLow = (totalPairs + 1) / 2
    val medianRankHigh = (totalPairs + 2) / 2

    val leftBounds = IntArray(n) { it + 1 }
    val rightBounds = IntArray(n) { n }

    var pivot = sortedValues[(n - 1) / 2] + sortedValues[n / 2]
    var activeSetSize = totalPairs
    var previousCount = 0

    while (true) {
        var countBelowPivot = 0
        var currentColumn = n
        val partitionCounts = IntArray(n)

        for (row in 1..n) {
            while (currentColumn >= row &&
                sortedValues[row - 1] + sortedValues[currentColumn - 1] >= pivot) {
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

            pivot = (minActiveSum + maxActiveSum) / 2
            if (pivot <= minActiveSum || pivot > maxActiveSum) pivot = maxActiveSum
            if (minActiveSum == maxActiveSum || activeSetSize <= 2) return pivot / 2
            continue
        }

        val atTargetRank = countBelowPivot == medianRankLow ||
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
                (smallestAtOrAbovePivot + largestBelowPivot) / 4
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
        activeSetSize = (0 until n).sumOf { maxOf(0, rightBounds[it] - leftBounds[it] + 1) }

        if (activeSetSize > 2) {
            val targetIndex = Random.nextInt(activeSetSize)
            var cumulativeSize = 0
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

            pivot = (minRemainingSum + maxRemainingSum) / 2
            if (pivot <= minRemainingSum || pivot > maxRemainingSum) pivot = maxRemainingSum
            if (minRemainingSum == maxRemainingSum) return pivot / 2
        }
    }
}
