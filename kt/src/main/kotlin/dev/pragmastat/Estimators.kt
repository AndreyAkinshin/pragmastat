package dev.pragmastat

import kotlin.math.abs
import kotlin.math.min

/**
 * Calculates the median of a list of values
 */
fun median(values: List<Double>): Double {
    // Check validity (priority 0)
    checkValidity(values, Subject.X)
    
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
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    return fastCenter(x)
}

/**
 * Estimates data dispersion (Spread)
 *
 * Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
 * More robust than standard deviation and more efficient than MAD.
 * Uses fast O(n log n) algorithm.
 *
 * Assumptions:
 *   - sparity(x) - sample must be non tie-dominant (Spread > 0)
 */
fun spread(x: List<Double>): Double {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    // Check sparity (priority 2)
    checkSparity(x, Subject.X)
    return fastSpread(x)
}

/**
 * Measures the relative dispersion of a sample (RelSpread)
 *
 * Calculates the ratio of Spread to absolute Center.
 * Robust alternative to the coefficient of variation.
 *
 * Assumptions:
 *   - positivity(x) - all values must be strictly positive (ensures Center > 0)
 */
fun relSpread(x: List<Double>): Double {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    // Check positivity (priority 1)
    checkPositivity(x, Subject.X)

    val centerVal = fastCenter(x)
    // Calculate spread (using internal implementation since we already validated)
    val spreadVal = fastSpread(x)
    // center is guaranteed positive because all values are positive
    return spreadVal / abs(centerVal)
}

/**
 * Measures the typical difference between elements of x and y (Shift)
 *
 * Calculates the median of all pairwise differences (x[i] - y[j]).
 * Positive values mean x is typically larger, negative means y is typically larger.
 * Uses fast O((m + n) * log(precision)) algorithm.
 */
fun shift(x: List<Double>, y: List<Double>): Double {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    checkValidity(y, Subject.Y)
    return fastShift(x, y)[0]
}

/**
 * Measures how many times larger x is compared to y (Ratio)
 *
 * Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
 * Equivalent to: exp(Shift(log(x), log(y)))
 * For example, ratio = 1.2 means x is typically 20% larger than y.
 * Uses fast O((m + n) * log(precision)) algorithm.
 *
 * Assumptions:
 *   - positivity(x) - all values in x must be strictly positive
 *   - positivity(y) - all values in y must be strictly positive
 */
fun ratio(x: List<Double>, y: List<Double>): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)
    // Check positivity for x (priority 1, subject x)
    checkPositivity(x, Subject.X)
    // Check positivity for y (priority 1, subject y)
    checkPositivity(y, Subject.Y)

    return fastRatio(x, y)[0]
}

/**
 * Measures the typical variability when considering both samples together (AvgSpread)
 *
 * Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
 *
 * Assumptions:
 *   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
 *   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
 */
fun avgSpread(x: List<Double>, y: List<Double>): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)
    // Check sparity for x (priority 2, subject x)
    checkSparity(x, Subject.X)
    // Check sparity for y (priority 2, subject y)
    checkSparity(y, Subject.Y)

    val n = x.size
    val m = y.size
    // Calculate spreads (using internal implementation since we already validated)
    val spreadX = fastSpread(x)
    val spreadY = fastSpread(y)

    return (n * spreadX + m * spreadY) / (n + m).toDouble()
}

/**
 * Measures effect size: a normalized difference between x and y (Disparity)
 *
 * Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
 *
 * Assumptions:
 *   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
 *   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
 */
fun disparity(x: List<Double>, y: List<Double>): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)
    // Check sparity for x (priority 2, subject x)
    checkSparity(x, Subject.X)
    // Check sparity for y (priority 2, subject y)
    checkSparity(y, Subject.Y)

    val n = x.size
    val m = y.size

    // Calculate shift (we know inputs are valid)
    val shiftVal = fastShift(x, y)[0]
    // Calculate avg_spread (using internal implementation since we already validated)
    val spreadX = fastSpread(x)
    val spreadY = fastSpread(y)
    val avgSpreadVal = (n * spreadX + m * spreadY) / (n + m).toDouble()

    return shiftVal / avgSpreadVal
}

/**
 * Represents an interval with lower and upper bounds
 */
data class Bounds(val lower: Double, val upper: Double)

/**
 * Provides bounds on the Shift estimator with specified misclassification rate (ShiftBounds)
 *
 * The misrate represents the probability that the true shift falls outside the computed bounds.
 * This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate (probability that true shift falls outside bounds)
 * @return A Bounds object containing the lower and upper bounds
 */
fun shiftBounds(x: List<Double>, y: List<Double>, misrate: Double): Bounds {
    // Check validity for x
    checkValidity(x, Subject.X)
    // Check validity for y
    checkValidity(y, Subject.Y)

    val n = x.size
    val m = y.size

    // Sort both arrays
    val xs = x.sorted()
    val ys = y.sorted()

    val total = n.toLong() * m.toLong()

    // Special case: when there's only one pairwise difference, bounds collapse to a single value
    if (total == 1L) {
        val value = xs[0] - ys[0]
        return Bounds(value, value)
    }

    val margin = pairwiseMargin(n, m, misrate)
    val halfMargin = min(margin.toLong() / 2, (total - 1) / 2)
    val kLeft = halfMargin
    val kRight = (total - 1) - halfMargin

    // Compute quantile positions
    val denominator = (total - 1).toDouble().takeIf { it > 0.0 } ?: 1.0
    val p = doubleArrayOf(kLeft.toDouble() / denominator, kRight.toDouble() / denominator)

    val bounds = fastShift(xs, ys, p, assumeSorted = true)

    val lower = minOf(bounds[0], bounds[1])
    val upper = maxOf(bounds[0], bounds[1])

    return Bounds(lower, upper)
}

/**
 * Provides bounds on the Ratio estimator with specified misclassification rate (RatioBounds)
 *
 * Computes bounds via log-transformation and shiftBounds delegation:
 * ratioBounds(x, y, misrate) = exp(shiftBounds(log(x), log(y), misrate))
 *
 * Assumptions:
 *   - positivity(x) - all values in x must be strictly positive
 *   - positivity(y) - all values in y must be strictly positive
 *
 * @param x First sample (must be strictly positive)
 * @param y Second sample (must be strictly positive)
 * @param misrate Misclassification rate (probability that true ratio falls outside bounds)
 * @return A Bounds object containing the lower and upper bounds
 */
fun ratioBounds(x: List<Double>, y: List<Double>, misrate: Double): Bounds {
    checkValidity(x, Subject.X)
    checkValidity(y, Subject.Y)

    // Log-transform samples (includes positivity check)
    val logX = log(x, Subject.X)
    val logY = log(y, Subject.Y)

    // Delegate to shiftBounds in log-space
    val logBounds = shiftBounds(logX, logY, misrate)

    // Exp-transform back to ratio-space
    return Bounds(
        kotlin.math.exp(logBounds.lower),
        kotlin.math.exp(logBounds.upper)
    )
}
