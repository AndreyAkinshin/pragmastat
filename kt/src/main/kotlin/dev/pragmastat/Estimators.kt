package dev.pragmastat

import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min

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
    val spreadVal = fastSpread(x)
    if (spreadVal <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    return spreadVal
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
@Deprecated("Use spread(x) / abs(center(x)) instead.", ReplaceWith("spread(x) / abs(center(x))"))
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
fun shift(
    x: List<Double>,
    y: List<Double>,
): Double {
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
fun ratio(
    x: List<Double>,
    y: List<Double>,
): Double {
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
internal fun avgSpread(
    x: List<Double>,
    y: List<Double>,
): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)

    val n = x.size
    val m = y.size
    val spreadX = fastSpread(x)
    if (spreadX <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    val spreadY = fastSpread(y)
    if (spreadY <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.Y))
    }

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
fun disparity(
    x: List<Double>,
    y: List<Double>,
): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)

    val n = x.size
    val m = y.size

    val spreadX = fastSpread(x)
    if (spreadX <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    val spreadY = fastSpread(y)
    if (spreadY <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.Y))
    }

    // Calculate shift (we know inputs are valid)
    val shiftVal = fastShift(x, y)[0]
    val avgSpreadVal = (n * spreadX + m * spreadY) / (n + m).toDouble()

    return shiftVal / avgSpreadVal
}

const val DEFAULT_MISRATE = 1e-3

/**
 * Represents an interval with lower and upper bounds and an associated measurement unit.
 *
 * @property lower The lower bound of the interval
 * @property upper The upper bound of the interval
 * @property unit The measurement unit associated with these bounds
 */
data class Bounds(
    val lower: Double,
    val upper: Double,
    val unit: MeasurementUnit = NumberUnit,
) {
    /** Returns true if [value] is within [lower, upper]. */
    fun contains(value: Double): Boolean = lower <= value && value <= upper

    /** Returns a copy of this Bounds with the given [unit]. */
    internal fun withUnit(unit: MeasurementUnit): Bounds = copy(unit = unit)
}

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
fun shiftBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
): Bounds {
    // Check validity for x
    checkValidity(x, Subject.X)
    // Check validity for y
    checkValidity(y, Subject.Y)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    val m = y.size

    val minMisrate = minAchievableMisrateTwoSample(n, m)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

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
fun ratioBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
): Bounds {
    checkValidity(x, Subject.X)
    checkValidity(y, Subject.Y)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val minMisrate = minAchievableMisrateTwoSample(x.size, y.size)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    // Log-transform samples (includes positivity check)
    val logX = log(x, Subject.X)
    val logY = log(y, Subject.Y)

    // Delegate to shiftBounds in log-space
    val logBounds = shiftBounds(logX, logY, misrate)

    // Exp-transform back to ratio-space
    return Bounds(
        kotlin.math.exp(logBounds.lower),
        kotlin.math.exp(logBounds.upper),
    )
}

/**
 * Provides exact distribution-free bounds on the center (Hodges-Lehmann pseudomedian)
 * with specified misclassification rate.
 *
 * Uses Wilcoxon signed-rank distribution for exact coverage.
 * Requires weak symmetry assumption: distribution symmetric around unknown center.
 *
 * @param x Sample data
 * @param misrate Misclassification rate (probability that true center falls outside bounds)
 * @return A Bounds object containing the lower and upper bounds
 * @throws AssumptionException if sample size < 2 or misrate is below minimum achievable
 */
fun centerBounds(
    x: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
): Bounds {
    checkValidity(x, Subject.X)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    if (n < 2) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }

    val minMisrate = minAchievableMisrateOneSample(n)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val margin = signedRankMargin(n, misrate)
    val totalPairs = n.toLong() * (n + 1) / 2

    var halfMargin: Long = (margin / 2).toLong()
    val maxHalfMargin = (totalPairs - 1) / 2
    if (halfMargin > maxHalfMargin) {
        halfMargin = maxHalfMargin
    }

    val kLeft = halfMargin + 1L
    val kRight = totalPairs - halfMargin

    val sorted = x.sorted()
    val (lo, hi) = FastCenterQuantiles.bounds(sorted, kLeft, kRight)

    return Bounds(lo, hi)
}

/**
 * Provides distribution-free bounds on the Spread estimator using
 * disjoint pairs with sign-test inversion.
 *
 * @param x Sample data
 * @param misrate Misclassification rate (probability that true spread falls outside bounds)
 * @param seed Optional string seed for deterministic randomization
 * @return A Bounds object containing the lower and upper bounds
 */
fun spreadBounds(
    x: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
): Bounds {
    checkValidity(x, Subject.X)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    val m = n / 2
    val minMisrate = minAchievableMisrateOneSample(m)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    if (x.size < 2) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    if (fastSpread(x) <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }

    val rng = if (seed != null) Rng(seed) else Rng()
    val margin = signMarginRandomized(m, misrate, rng)
    var halfMargin = margin / 2
    val maxHalfMargin = (m - 1) / 2
    if (halfMargin > maxHalfMargin) halfMargin = maxHalfMargin
    val kLeft = halfMargin + 1
    val kRight = m - halfMargin

    val indices = (0 until n).toList()
    val shuffled = rng.shuffle(indices)
    val diffs =
        DoubleArray(m) { i ->
            val a = shuffled[2 * i]
            val b = shuffled[2 * i + 1]
            abs(x[a] - x[b])
        }
    diffs.sort()

    return Bounds(diffs[kLeft - 1], diffs[kRight - 1])
}

/**
 * Provides distribution-free bounds for the Disparity estimator (Shift / AvgSpread)
 * using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate
 * @param seed Optional string seed for deterministic randomization
 * @return A Bounds object containing the lower and upper bounds
 */
fun disparityBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
): Bounds {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    checkValidity(y, Subject.Y)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    val m = y.size
    if (n < 2) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }
    if (m < 2) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.Y))
    }

    val minShift = minAchievableMisrateTwoSample(n, m)
    val minX = minAchievableMisrateOneSample(n / 2)
    val minY = minAchievableMisrateOneSample(m / 2)
    val minAvg = 2.0 * max(minX, minY)

    if (misrate < minShift + minAvg) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val extra = misrate - (minShift + minAvg)
    val alphaShift = minShift + extra / 2.0
    val alphaAvg = minAvg + extra / 2.0

    if (fastSpread(x) <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    if (fastSpread(y) <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.Y))
    }

    val sb = shiftBounds(x, y, alphaShift)
    val ab = avgSpreadBounds(x, y, alphaAvg, seed)

    val la = ab.lower
    val ua = ab.upper
    val ls = sb.lower
    val us = sb.upper

    if (la > 0.0) {
        val r1 = ls / la
        val r2 = ls / ua
        val r3 = us / la
        val r4 = us / ua
        val lower = min(min(r1, r2), min(r3, r4))
        val upper = max(max(r1, r2), max(r3, r4))
        return Bounds(lower, upper)
    }

    if (ua <= 0.0) {
        if (ls == 0.0 && us == 0.0) return Bounds(0.0, 0.0)
        if (ls >= 0.0) return Bounds(0.0, Double.POSITIVE_INFINITY)
        if (us <= 0.0) return Bounds(Double.NEGATIVE_INFINITY, 0.0)
        return Bounds(Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY)
    }

    // Default: ua > 0 && la <= 0
    if (ls > 0.0) return Bounds(ls / ua, Double.POSITIVE_INFINITY)
    if (us < 0.0) return Bounds(Double.NEGATIVE_INFINITY, us / ua)
    if (ls == 0.0 && us == 0.0) return Bounds(0.0, 0.0)
    if (ls == 0.0 && us > 0.0) return Bounds(0.0, Double.POSITIVE_INFINITY)
    if (ls < 0.0 && us == 0.0) return Bounds(Double.NEGATIVE_INFINITY, 0.0)

    return Bounds(Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY)
}

/**
 * Provides distribution-free bounds for AvgSpread using Bonferroni combination.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate (probability that true avg_spread falls outside bounds)
 * @param seed Optional string seed for deterministic randomization
 * @return A Bounds object containing the lower and upper bounds
 */
internal fun avgSpreadBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
): Bounds {
    checkValidity(x, Subject.X)
    checkValidity(y, Subject.Y)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    val m = y.size
    if (n < 2) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }
    if (m < 2) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.Y))
    }

    val alpha = misrate / 2.0
    val minX = minAchievableMisrateOneSample(n / 2)
    val minY = minAchievableMisrateOneSample(m / 2)
    if (alpha < minX || alpha < minY) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    if (fastSpread(x) <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    if (fastSpread(y) <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.Y))
    }

    val boundsX = spreadBounds(x, alpha, seed)
    val boundsY = spreadBounds(y, alpha, seed)

    val weightX = n.toDouble() / (n + m).toDouble()
    val weightY = m.toDouble() / (n + m).toDouble()

    return Bounds(
        weightX * boundsX.lower + weightY * boundsY.lower,
        weightX * boundsX.upper + weightY * boundsY.upper,
    )
}
