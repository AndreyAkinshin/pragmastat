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
 */
fun center(x: List<Double>): Double {
    require(x.isNotEmpty()) { "Input list cannot be empty" }
    
    val pairwiseAverages = mutableListOf<Double>()
    for (i in x.indices) {
        for (j in i until x.size) {
            pairwiseAverages.add((x[i] + x[j]) / 2.0)
        }
    }
    
    return median(pairwiseAverages)
}

/**
 * Estimates data dispersion (Spread)
 * 
 * Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
 * More robust than standard deviation and more efficient than MAD.
 */
fun spread(x: List<Double>): Double {
    require(x.isNotEmpty()) { "Input list cannot be empty" }
    if (x.size == 1) return 0.0
    
    val pairwiseDiffs = mutableListOf<Double>()
    for (i in x.indices) {
        for (j in i + 1 until x.size) {
            pairwiseDiffs.add(abs(x[i] - x[j]))
        }
    }
    
    return median(pairwiseDiffs)
}

/**
 * Measures the relative dispersion of a sample (Volatility)
 * 
 * Calculates the ratio of Spread to absolute Center.
 * Robust alternative to the coefficient of variation.
 */
fun volatility(x: List<Double>): Double {
    val centerVal = center(x)
    require(centerVal != 0.0) { "Volatility is undefined when Center equals zero" }
    
    return spread(x) / abs(centerVal)
}

/**
 * Measures precision: the distance between two estimations of independent random samples (Precision)
 * 
 * Calculated as 2 * Spread / sqrt(n). The interval center ± precision forms a range
 * that probably contains the true center value.
 */
fun precision(x: List<Double>): Double {
    require(x.isNotEmpty()) { "Input list cannot be empty" }
    
    return 2.0 * spread(x) / sqrt(x.size.toDouble())
}

/**
 * Measures the typical difference between elements of x and y (MedShift)
 * 
 * Calculates the median of all pairwise differences (x[i] - y[j]).
 * Positive values mean x is typically larger, negative means y is typically larger.
 */
fun medShift(x: List<Double>, y: List<Double>): Double {
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
 * Measures how many times larger x is compared to y (MedRatio)
 * 
 * Calculates the median of all pairwise ratios (x[i] / y[j]).
 * For example, medRatio = 1.2 means x is typically 20% larger than y.
 */
fun medRatio(x: List<Double>, y: List<Double>): Double {
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
 * Measures the typical variability when considering both samples together (MedSpread)
 * 
 * Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
 */
fun medSpread(x: List<Double>, y: List<Double>): Double {
    require(x.isNotEmpty() && y.isNotEmpty()) { "Input lists cannot be empty" }
    
    val n = x.size
    val m = y.size
    val spreadX = spread(x)
    val spreadY = spread(y)
    
    return (n * spreadX + m * spreadY) / (n + m).toDouble()
}

/**
 * Measures effect size: a normalized absolute difference between x and y (MedDisparity)
 * 
 * Calculated as MedShift / MedSpread. Robust alternative to Cohen's d.
 * Returns infinity if medSpread is zero.
 */
fun medDisparity(x: List<Double>, y: List<Double>): Double {
    val medSpreadVal = medSpread(x, y)
    if (medSpreadVal == 0.0) return Double.POSITIVE_INFINITY
    
    return medShift(x, y) / medSpreadVal
}