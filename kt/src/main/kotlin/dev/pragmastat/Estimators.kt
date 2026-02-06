package dev.pragmastat

import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min
import kotlin.math.pow

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

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

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

/**
 * Provides distribution-free bounds on the median with specified misclassification rate.
 *
 * Uses order statistics with binomial distribution to compute exact coverage.
 * No distributional assumptions required.
 *
 * @param x Sample data
 * @param misrate Misclassification rate (probability that true median falls outside bounds)
 * @return A Bounds object containing the lower and upper bounds
 * @throws AssumptionException if sample size < 2 or misrate is below minimum achievable
 */
fun medianBounds(x: List<Double>, misrate: Double): Bounds {
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

    val sorted = x.sorted()
    val (loIdx, hiIdx) = computeOrderStatisticIndices(n, misrate)

    return Bounds(sorted[loIdx], sorted[hiIdx])
}

/**
 * Find order statistic indices that achieve the specified coverage.
 * Uses binomial distribution: the interval [X_{(lo+1)}, X_{(hi+1)}] (1-based)
 * has coverage 1 - 2*P(Bin(n,0.5) <= lo).
 */
private fun computeOrderStatisticIndices(n: Int, misrate: Double): Pair<Int, Int> {
    val alpha = misrate / 2

    // Find the largest k where P(Bin(n,0.5) <= k) <= alpha
    // This gives us the tightest confidence interval with coverage >= 1-misrate
    var lo = 0
    for (k in 0 until (n + 1) / 2) {
        val tailProb = binomialTailProbability(n, k)
        if (tailProb <= alpha) {
            lo = k // k is valid, update to largest valid k
        } else {
            break // Once we exceed alpha, all subsequent k will too
        }
    }

    // Symmetric interval: hi = n - 1 - lo
    var hi = n - 1 - lo

    // Ensure valid bounds
    if (hi < lo) {
        hi = lo
    }
    if (hi >= n) {
        hi = n - 1
    }

    return Pair(lo, hi)
}

/**
 * Compute P(X <= k) for X ~ Binomial(n, 0.5).
 * Uses incremental binomial coefficient computation for efficiency.
 * Note: 2^n overflows Double for n > 1024.
 */
private fun binomialTailProbability(n: Int, k: Int): Double {
    if (k < 0) return 0.0
    if (k >= n) return 1.0

    // Normal approximation with continuity correction for large n
    // (2^n overflows Double for n > 1024)
    if (n > 1023) {
        val mean = n.toDouble() / 2.0
        val std = kotlin.math.sqrt(n.toDouble() / 4.0)
        val z = (k.toDouble() + 0.5 - mean) / std
        return gaussCdf(z)
    }

    val total = 2.0.pow(n.toDouble())
    var sum = 0.0
    var coef = 1.0 // C(n, 0) = 1

    for (i in 0..k) {
        sum += coef
        // C(n, i+1) = C(n, i) * (n-i) / (i+1)
        coef = coef * (n - i).toDouble() / (i + 1).toDouble()
    }

    return sum / total
}

/**
 * Compute binomial coefficient C(n, k).
 */
private fun binomialCoefficient(n: Int, k: Int): Double {
    if (k < 0 || k > n) return 0.0
    if (k == 0 || k == n) return 1.0

    // Use the smaller k for efficiency
    val k2 = if (k > n - k) n - k else k

    var result = 1.0
    for (i in 0 until k2) {
        result *= (n - i).toDouble() / (i + 1).toDouble()
    }
    return result
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
fun centerBounds(x: List<Double>, misrate: Double): Bounds {
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

private const val DEFAULT_ITERATIONS = 10000
private const val MAX_SUBSAMPLE_SIZE = 5000
private const val DEFAULT_SEED = "center-bounds-approx"

/**
 * Provides bootstrap-based nominal bounds on the center (Hodges-Lehmann pseudomedian)
 * with specified misclassification rate.
 *
 * Uses bootstrap percentile method. No symmetry requirement, but provides only
 * nominal (not exact) coverage.
 *
 * WARNING: Bootstrap percentile method has known undercoverage for small samples.
 * When requesting 95% confidence (misrate = 0.05), actual coverage is typically 85-92% for n < 30.
 *
 * @param x Sample data
 * @param misrate Misclassification rate (probability that true center falls outside bounds)
 * @return A Bounds object containing the lower and upper bounds
 * @throws AssumptionException if sample size < 2 or misrate is below minimum achievable
 */
fun centerBoundsApprox(x: List<Double>, misrate: Double): Bounds =
    centerBoundsApprox(x, misrate, null, DEFAULT_ITERATIONS)

/**
 * Provides bootstrap-based nominal bounds on the center with specified seed.
 *
 * @param x Sample data
 * @param misrate Misclassification rate
 * @param seed Optional seed for reproducibility (null uses default seed)
 * @return A Bounds object containing the lower and upper bounds
 */
fun centerBoundsApprox(x: List<Double>, misrate: Double, seed: String?): Bounds =
    centerBoundsApprox(x, misrate, seed, DEFAULT_ITERATIONS)

/**
 * Provides bootstrap-based nominal bounds on the center with specified seed and iterations.
 *
 * @param x Sample data
 * @param misrate Misclassification rate
 * @param seed Optional seed for reproducibility (null uses default seed)
 * @param iterations Number of bootstrap iterations
 * @return A Bounds object containing the lower and upper bounds
 */
internal fun centerBoundsApprox(x: List<Double>, misrate: Double, seed: String?, iterations: Int): Bounds {
    checkValidity(x, Subject.X)
    require(iterations > 0) { "iterations must be positive" }

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    if (n < 2) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }

    val minMisrate = maxOf(2.0 / iterations, minAchievableMisrateOneSample(n))
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    // Sort for permutation invariance
    val sorted = x.sorted()

    // Use default seed for cross-language determinism when no seed provided
    val rng = Rng(seed ?: DEFAULT_SEED)

    // m-out-of-n subsampling: cap at MAX_SUBSAMPLE_SIZE for performance
    val m = min(n, MAX_SUBSAMPLE_SIZE)
    val centers = DoubleArray(iterations)

    for (i in 0 until iterations) {
        val resample = rng.resample(sorted, m)
        centers[i] = fastCenter(resample)
    }

    centers.sort()

    val alpha = misrate / 2
    val hiIdx = min(iterations - 1, kotlin.math.ceil((1 - alpha) * iterations).toInt() - 1)
    val loIdx = min(max(0, kotlin.math.floor(alpha * iterations).toInt()), hiIdx)

    var bootstrapLo = centers[loIdx]
    var bootstrapHi = centers[hiIdx]

    // Scale bounds to full n using asymptotic sqrt(n) rate
    if (m < n) {
        val centerVal = fastCenter(sorted)
        val scaleFactor = kotlin.math.sqrt(m.toDouble() / n)
        bootstrapLo = centerVal + (bootstrapLo - centerVal) / scaleFactor
        bootstrapHi = centerVal + (bootstrapHi - centerVal) / scaleFactor
    }

    return Bounds(bootstrapLo, bootstrapHi)
}
