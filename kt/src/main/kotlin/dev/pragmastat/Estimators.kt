package dev.pragmastat

import kotlin.math.abs
import kotlin.math.exp
import kotlin.math.max
import kotlin.math.min

/**
 * Estimates the central value of the data (Center)
 *
 * Calculates the median of all pairwise averages (x[i] + x[j])/2.
 * More robust than the mean and more efficient than the median.
 * Uses O(n log n) algorithm.
 *
 * This is the raw native-array API: it accepts a plain [List] and returns a
 * unitless [Double]. The [Sample]-based [center] overload is a thin adapter
 * over this function.
 *
 * @param x Sample data
 * @param assumeSorted When true, the caller guarantees [x] is already sorted
 *   ascending, so the internal sort is skipped. Passing true on unsorted input
 *   is a contract violation (undefined behavior): the result is unspecified,
 *   but termination is guaranteed (the selection loop is bounded and fails
 *   with a deterministic convergence error on pathological input).
 */
fun center(
    x: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    return centerImpl(x, assumeSorted)
}

/**
 * Estimates data dispersion (Spread)
 *
 * Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
 * More robust than standard deviation and more efficient than MAD.
 * Uses O(n log n) algorithm.
 *
 * This is the raw native-array API: it accepts a plain [List] and returns a
 * unitless [Double]. The [Sample]-based [spread] overload is a thin adapter
 * over this function.
 *
 * Assumptions:
 *   - sparity(x) - sample must be non tie-dominant (Spread > 0)
 *
 * @param x Sample data
 * @param assumeSorted When true, the caller guarantees [x] is already sorted
 *   ascending, so the internal sort is skipped. Passing true on unsorted input
 *   is a contract violation (undefined behavior): the result is unspecified,
 *   but termination is guaranteed (the selection loop is bounded and fails
 *   with a deterministic convergence error on pathological input).
 */
fun spread(
    x: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    val spreadVal = spreadImpl(x, assumeSorted)
    if (spreadVal <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    return spreadVal
}

/**
 * Measures the typical difference between elements of x and y (Shift)
 *
 * Calculates the median of all pairwise differences (x[i] - y[j]).
 * Positive values mean x is typically larger, negative means y is typically larger.
 * Uses O((m + n) * log(precision)) algorithm.
 *
 * This is the raw native-array API: it accepts plain [List]s and returns a
 * unitless [Double]. The [Sample]-based [shift] overload is a thin adapter
 * over this function.
 *
 * @param x First sample
 * @param y Second sample
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the internal sorts are skipped. Passing true
 *   on unsorted input is undefined behavior.
 */
fun shift(
    x: List<Double>,
    y: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    // Check validity (priority 0)
    checkValidity(x, Subject.X)
    checkValidity(y, Subject.Y)
    return shiftImpl(x, y, assumeSorted = assumeSorted)[0]
}

/**
 * Measures how many times larger x is compared to y (Ratio)
 *
 * Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
 * Equivalent to: exp(Shift(log(x), log(y)))
 * For example, ratio = 1.2 means x is typically 20% larger than y.
 * Uses O((m + n) * log(precision)) algorithm.
 *
 * This is the raw native-array API: it accepts plain [List]s and returns a
 * unitless [Double]. The [Sample]-based [ratio] overload is a thin adapter
 * over this function.
 *
 * Assumptions:
 *   - positivity(x) - all values in x must be strictly positive
 *   - positivity(y) - all values in y must be strictly positive
 *
 * @param x First sample (must be strictly positive)
 * @param y Second sample (must be strictly positive)
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the internal sorts are skipped. Passing true
 *   on unsorted input is undefined behavior.
 */
fun ratio(
    x: List<Double>,
    y: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)
    // Check positivity for x (priority 1, subject x)
    checkPositivity(x, Subject.X)
    // Check positivity for y (priority 1, subject y)
    checkPositivity(y, Subject.Y)

    return ratioImpl(x, y, assumeSorted = assumeSorted)[0]
}

/**
 * Measures the typical variability when considering both samples together (AvgSpread)
 *
 * Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
 *
 * Assumptions:
 *   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
 *   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
 *
 * @param x First sample
 * @param y Second sample
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the internal sorts are skipped. Passing true
 *   on unsorted input is undefined behavior.
 */
internal fun avgSpread(
    x: List<Double>,
    y: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)

    val n = x.size
    val m = y.size
    val spreadX = spreadImpl(x, assumeSorted)
    if (spreadX <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    val spreadY = spreadImpl(y, assumeSorted)
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
 * This is the raw native-array API: it accepts plain [List]s and returns a
 * unitless [Double]. The [Sample]-based [disparity] overload is a thin adapter
 * over this function.
 *
 * Assumptions:
 *   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
 *   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
 *
 * @param x First sample
 * @param y Second sample
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the internal sorts are skipped. Passing true
 *   on unsorted input is undefined behavior.
 */
fun disparity(
    x: List<Double>,
    y: List<Double>,
    assumeSorted: Boolean = false,
): Double {
    // Check validity for x (priority 0, subject x)
    checkValidity(x, Subject.X)
    // Check validity for y (priority 0, subject y)
    checkValidity(y, Subject.Y)

    val n = x.size
    val m = y.size

    val spreadX = spreadImpl(x, assumeSorted)
    if (spreadX <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    val spreadY = spreadImpl(y, assumeSorted)
    if (spreadY <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.Y))
    }

    // Calculate shift (we know inputs are valid)
    val shiftVal = shiftImpl(x, y, assumeSorted = assumeSorted)[0]
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
 * This is the raw native-array API: it accepts plain [List]s and returns
 * unitless [Bounds]. The [Sample]-based [shiftBounds] overload is a thin
 * adapter over this function.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate (probability that true shift falls outside bounds)
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the internal sorts are skipped. This is an
 *   order-independent estimator, so the flag only changes the computation path,
 *   not the result. Passing true on unsorted input is undefined behavior.
 * @return A Bounds object containing the lower and upper bounds
 */
fun shiftBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    assumeSorted: Boolean = false,
): Bounds {
    checkValidity(x, Subject.X)
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

    val xs = if (assumeSorted) x else x.sorted()
    val ys = if (assumeSorted) y else y.sorted()

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

    val bounds = shiftImpl(xs, ys, p, assumeSorted = true)

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
 * This is the raw native-array API: it accepts plain [List]s and returns
 * unitless [Bounds]. The [Sample]-based [ratioBounds] overload is a thin
 * adapter over this function.
 *
 * @param x First sample (must be strictly positive)
 * @param y Second sample (must be strictly positive)
 * @param misrate Misclassification rate (probability that true ratio falls outside bounds)
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the internal sorts are skipped. Since log is
 *   monotonic, sorted positive input yields sorted log output. This is an
 *   order-independent estimator, so the flag only changes the computation path,
 *   not the result. Passing true on unsorted input is undefined behavior.
 * @return A Bounds object containing the lower and upper bounds
 */
fun ratioBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    assumeSorted: Boolean = false,
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

    // log is monotonic: sorted positive input -> sorted log output
    val logBounds = shiftBounds(logX, logY, misrate, assumeSorted)

    // Exp-transform back to ratio-space
    return Bounds(
        exp(logBounds.lower),
        exp(logBounds.upper),
    )
}

/**
 * Provides exact distribution-free bounds on the center (Hodges-Lehmann pseudomedian)
 * with specified misclassification rate.
 *
 * Uses Wilcoxon signed-rank distribution for exact coverage.
 * Requires weak symmetry assumption: distribution symmetric around unknown center.
 *
 * This is the raw native-array API: it accepts a plain [List] and returns
 * unitless [Bounds]. The [Sample]-based [centerBounds] overload is a thin
 * adapter over this function.
 *
 * @param x Sample data
 * @param misrate Misclassification rate (probability that true center falls outside bounds)
 * @param assumeSorted When true, the caller guarantees [x] is already sorted
 *   ascending, so the internal sort is skipped. This is an order-independent
 *   estimator, so the flag only changes the computation path, not the result.
 *   Passing true on unsorted input is undefined behavior.
 * @return A Bounds object containing the lower and upper bounds
 * @throws AssumptionException if sample size < 2 or misrate is below minimum achievable
 */
fun centerBounds(
    x: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    assumeSorted: Boolean = false,
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

    val sorted = if (assumeSorted) x else x.sorted()
    val (lo, hi) = CenterQuantilesImpl.bounds(sorted, kLeft, kRight)

    return Bounds(lo, hi)
}

/**
 * Provides distribution-free bounds on the Spread estimator using
 * disjoint pairs with sign-test inversion.
 *
 * This is the raw native-array API: it accepts a plain [List] and returns
 * unitless [Bounds]. The [Sample]-based [spreadBounds] overload is a thin
 * adapter over this function.
 *
 * The disjoint-pair shuffle always runs on the passed array's order, so the
 * shuffle result is independent of [assumeSorted]. [assumeSorted] only skips
 * the internal sort of the order-independent sparity (spread > 0) check, so the
 * flag never changes the result. Passing true on unsorted input is undefined
 * behavior.
 *
 * @param x Sample data
 * @param misrate Misclassification rate (probability that true spread falls outside bounds)
 * @param seed Optional string seed for deterministic randomization
 * @param assumeSorted When true, the caller guarantees [x] is already sorted
 *   ascending, so the sparity check reuses [x] without re-sorting.
 * @return A Bounds object containing the lower and upper bounds
 */
fun spreadBounds(
    x: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
    assumeSorted: Boolean = false,
): Bounds =
    // Map the assumeSorted flag to the optional pre-sorted view: when sorted,
    // x doubles as the sparity-check view. The shuffle always runs on x.
    spreadBoundsImpl(
        x = x,
        misrate = misrate,
        seed = seed,
        sortedX = if (assumeSorted) x else null,
    )

/**
 * Single implementation of spread bounds shared by the raw and [Sample]-based
 * entry points.
 *
 * [x] is always in ORIGINAL order (the disjoint-pair shuffle is order-dependent).
 * [sortedX], when non-null, is a pre-sorted view used only to speed up the
 * order-independent sparity check. NEVER pass [sortedX] as the shuffle array.
 */
internal fun spreadBoundsImpl(
    x: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
    sortedX: List<Double>? = null,
): Bounds {
    checkValidity(x, Subject.X)

    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val n = x.size
    if (n < 2) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    val m = n / 2
    val minMisrate = minAchievableMisrateOneSample(m)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }
    // sortedX (when provided) is a pre-sorted view for the order-independent
    // sparity check; the shuffle in spreadBoundsInner always runs on original x.
    val spreadVal = if (sortedX != null) spreadImpl(sortedX, assumeSorted = true) else spreadImpl(x)
    if (spreadVal <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }

    return spreadBoundsInner(x, misrate, seed)
}

/**
 * Inner computation for spreadBounds: margin calculation, shuffle, diff computation.
 * Caller is responsible for validity and sparity checks.
 */
internal fun spreadBoundsInner(
    values: List<Double>,
    misrate: Double,
    seed: String?,
): Bounds {
    val n = values.size
    val m = n / 2

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
            abs(values[a] - values[b])
        }
    diffs.sort()

    return Bounds(diffs[kLeft - 1], diffs[kRight - 1])
}

/**
 * Computes disparity bounds from shift and avgSpread bound components.
 * Handles edge cases where the avgSpread interval crosses zero.
 */
internal fun disparityBoundsFromComponents(
    ls: Double,
    us: Double,
    la: Double,
    ua: Double,
): Bounds {
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
 * Provides distribution-free bounds for the Disparity estimator (Shift / AvgSpread)
 * using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
 *
 * This is the raw native-array API: it accepts plain [List]s and returns
 * unitless [Bounds]. The [Sample]-based [disparityBounds] overload is a thin
 * adapter over this function.
 *
 * The disjoint-pair shuffle (inside the avg-spread component) always runs on the
 * passed arrays' order, so it is independent of [assumeSorted]. However, the
 * embedded shift-bounds sub-computation consumes the passed arrays AS a sorted
 * view when [assumeSorted] is true: on genuinely sorted input this is inert (the
 * flag only skips the internal sort and does not change the result), but on
 * UNSORTED input the flag is undefined behavior and CAN change the result
 * (shift-bounds would treat the unsorted slice as sorted).
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate
 * @param seed Optional string seed for deterministic randomization
 * @param assumeSorted When true, the caller guarantees both [x] and [y] are
 *   already sorted ascending, so the sub-computations reuse them without
 *   re-sorting. Passing true on unsorted input is undefined behavior and can
 *   change the result.
 * @return A Bounds object containing the lower and upper bounds
 */
fun disparityBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
    assumeSorted: Boolean = false,
): Bounds =
    // Map the assumeSorted flag to the optional pre-sorted views: when sorted,
    // x/y double as the sparity/shift-bounds views. The shuffles always run on x/y.
    disparityBoundsImpl(
        x = x,
        y = y,
        misrate = misrate,
        seed = seed,
        sortedX = if (assumeSorted) x else null,
        sortedY = if (assumeSorted) y else null,
    )

/**
 * Single implementation of disparity bounds shared by the raw and [Sample]-based
 * entry points.
 *
 * [x]/[y] are always in ORIGINAL order; [sortedX]/[sortedY], when non-null, are
 * pre-sorted views used only for the order-independent sparity and shift-bounds
 * sub-computations. NEVER pass the sorted views as the shuffle arrays.
 */
internal fun disparityBoundsImpl(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
    sortedX: List<Double>? = null,
    sortedY: List<Double>? = null,
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

    // The spread > 0 sparity check is performed by avgSpreadBounds below
    // (identical predicate, Subject.X/Y order). shiftBounds runs first but cannot
    // throw for these inputs, so it cannot mask that sparity error.
    // shiftBounds is order-independent given sorted input; use sorted views when present.
    val sb =
        if (sortedX != null && sortedY != null) {
            shiftBounds(sortedX, sortedY, alphaShift, assumeSorted = true)
        } else {
            shiftBounds(x, y, alphaShift)
        }
    val ab = avgSpreadBounds(x, y, alphaAvg, seed, sortedX, sortedY)

    return disparityBoundsFromComponents(sb.lower, sb.upper, ab.lower, ab.upper)
}

/**
 * Provides distribution-free bounds for AvgSpread using Bonferroni combination.
 *
 * @param x First sample
 * @param y Second sample
 * @param misrate Misclassification rate (probability that true avg_spread falls outside bounds)
 * @param seed Optional string seed for deterministic randomization
 * @param sortedX Optional pre-sorted view of [x], used only for the
 *   order-independent sparity check; null means sort internally. The shuffle
 *   always runs on the original [x] order.
 * @param sortedY Optional pre-sorted view of [y], used only for the
 *   order-independent sparity check; null means sort internally. The shuffle
 *   always runs on the original [y] order.
 * @return A Bounds object containing the lower and upper bounds
 */
internal fun avgSpreadBounds(
    x: List<Double>,
    y: List<Double>,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
    sortedX: List<Double>? = null,
    sortedY: List<Double>? = null,
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

    // sortedX/sortedY (when provided) are pre-sorted views for the sparity
    // checks; the shuffles in avgSpreadBoundsInner always run on original x/y.
    val spreadX = if (sortedX != null) spreadImpl(sortedX, assumeSorted = true) else spreadImpl(x)
    if (spreadX <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.X))
    }
    val spreadY = if (sortedY != null) spreadImpl(sortedY, assumeSorted = true) else spreadImpl(y)
    if (spreadY <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, Subject.Y))
    }

    return avgSpreadBoundsInner(x, y, alpha, seed)
}

/**
 * Inner computation for avgSpreadBounds: computes weighted combination of individual spread bounds.
 * Caller is responsible for validity, domain, and sparity checks.
 */
internal fun avgSpreadBoundsInner(
    x: List<Double>,
    y: List<Double>,
    alpha: Double,
    seed: String?,
): Bounds {
    val n = x.size
    val m = y.size

    val boundsX = spreadBoundsInner(x, alpha, seed)
    val boundsY = spreadBoundsInner(y, alpha, seed)

    val weightX = n.toDouble() / (n + m).toDouble()
    val weightY = m.toDouble() / (n + m).toDouble()

    return Bounds(
        weightX * boundsX.lower + weightY * boundsY.lower,
        weightX * boundsX.upper + weightY * boundsY.upper,
    )
}
