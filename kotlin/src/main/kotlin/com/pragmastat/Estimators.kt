package com.pragmastat

import kotlin.math.abs
import kotlin.math.sqrt

/**
 * Calculates the median of a list of values
 */
fun median(values: List<Double>): Double {
    require(values.isNotEmpty()) { "Input list cannot be empty" }
    
    val sorted = values.sorted()
    val n = sorted.size
    
    return if (n % 2 == 0) {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

/**
 * Estimates the central value of the data (Center)
 *
 * Calculates the median of all pairwise averages (x[i] + x[j])/2.
 * More robust than the mean and more efficient than the median.
 * Uses fast O(n log n) algorithm.
 */
fun center(x: List<Double>): Double {
    return fastCenter(x)
}

/**
 * Estimates data dispersion (Spread)
 *
 * Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
 * More robust than standard deviation and more efficient than MAD.
 * Uses fast O(n log n) algorithm.
 */
fun spread(x: List<Double>): Double {
    return fastSpread(x)
}

/**
 * Measures the relative dispersion of a sample (RelSpread)
 *
 * Calculates the ratio of Spread to absolute Center.
 * Robust alternative to the coefficient of variation.
 */
fun relSpread(x: List<Double>): Double {
    val centerVal = center(x)
    require(centerVal != 0.0) { "RelSpread is undefined when Center equals zero" }

    return spread(x) / abs(centerVal)
}

/**
 * Measures the typical difference between elements of x and y (Shift)
 *
 * Calculates the median of all pairwise differences (x[i] - y[j]).
 * Positive values mean x is typically larger, negative means y is typically larger.
 */
fun shift(x: List<Double>, y: List<Double>): Double {
    require(x.isNotEmpty() && y.isNotEmpty()) { "Input lists cannot be empty" }

    val pairwiseShifts = mutableListOf<Double>()
    for (xi in x) {
        for (yj in y) {
            pairwiseShifts.add(xi - yj)
        }
    }

    return median(pairwiseShifts)
}

/**
 * Measures how many times larger x is compared to y (Ratio)
 *
 * Calculates the median of all pairwise ratios (x[i] / y[j]).
 * For example, ratio = 1.2 means x is typically 20% larger than y.
 */
fun ratio(x: List<Double>, y: List<Double>): Double {
    require(x.isNotEmpty() && y.isNotEmpty()) { "Input lists cannot be empty" }
    require(y.all { it > 0 }) { "All values in y must be strictly positive" }

    val pairwiseRatios = mutableListOf<Double>()
    for (xi in x) {
        for (yj in y) {
            pairwiseRatios.add(xi / yj)
        }
    }

    return median(pairwiseRatios)
}

/**
 * Measures the typical variability when considering both samples together (AvgSpread)
 *
 * Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
 */
fun avgSpread(x: List<Double>, y: List<Double>): Double {
    require(x.isNotEmpty() && y.isNotEmpty()) { "Input lists cannot be empty" }

    val n = x.size
    val m = y.size
    val spreadX = spread(x)
    val spreadY = spread(y)

    return (n * spreadX + m * spreadY) / (n + m).toDouble()
}

/**
 * Measures effect size: a normalized difference between x and y (Disparity)
 *
 * Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
 * Returns infinity if avgSpread is zero.
 */
fun disparity(x: List<Double>, y: List<Double>): Double {
    val avgSpreadVal = avgSpread(x, y)
    if (avgSpreadVal == 0.0) return Double.POSITIVE_INFINITY

    return shift(x, y) / avgSpreadVal
}