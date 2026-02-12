package dev.pragmastat

import kotlin.math.PI
import kotlin.math.exp
import kotlin.math.max
import kotlin.math.min
import kotlin.math.sqrt

// Maximum n for exact computation. Limited to 63 because 2^n must fit in a 64-bit integer.
private const val MAX_EXACT_SIZE = 63

/**
 * Computes the signed-rank margin for one-sample bounds.
 *
 * One-sample analog of pairwiseMargin using Wilcoxon signed-rank distribution.
 * Uses exact computation for n <= 63, Edgeworth approximation for larger n.
 *
 * @param n Sample size (must be positive)
 * @param misrate Desired misclassification rate in [0, 1]
 * @return Margin value for one-sample bounds
 * @throws IllegalArgumentException if misrate is below minimum achievable
 */
internal fun signedRankMargin(n: Int, misrate: Double): Int {
    if (n <= 0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }
    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val minMisrate = minAchievableMisrateOneSample(n)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    return if (n <= MAX_EXACT_SIZE) {
        calcExact(n, misrate)
    } else {
        calcApprox(n, misrate)
    }
}

private fun calcExact(n: Int, misrate: Double): Int {
    val raw = calcExactRaw(n, misrate / 2)
    return raw * 2
}

private fun calcApprox(n: Int, misrate: Double): Int {
    val raw = calcApproxRaw(n, misrate / 2)
    val margin = raw * 2
    if (margin > Int.MAX_VALUE) {
        throw ArithmeticException("Signed-rank margin exceeds supported range for n=$n")
    }
    return margin.toInt()
}

/**
 * Compute one-sided margin using exact Wilcoxon signed-rank distribution.
 * Uses dynamic programming to compute the CDF.
 */
@OptIn(ExperimentalUnsignedTypes::class)
private fun calcExactRaw(n: Int, p: Double): Int {
    val total = 1UL shl n
    val maxW = n.toLong() * (n + 1) / 2

    val count = ULongArray(maxW.toInt() + 1)
    count[0] = 1u

    for (i in 1..n) {
        val maxWi = min(maxW, i.toLong() * (i + 1) / 2).toInt()
        for (w in maxWi downTo i) {
            count[w] = count[w] + count[w - i]
        }
    }

    var cumulative = 0UL
    for (w in 0..maxW.toInt()) {
        cumulative = cumulative + count[w]
        val cdf = cumulative.toDouble() / total.toDouble()
        if (cdf >= p) {
            return w
        }
    }

    return maxW.toInt()
}

/**
 * Compute one-sided margin using Edgeworth approximation for large n.
 */
private fun calcApproxRaw(n: Int, misrate: Double): Long {
    val maxW = n.toLong() * (n + 1) / 2
    var a = 0L
    var b = maxW

    while (a < b - 1) {
        val c = (a + b) / 2
        val cdf = edgeworthCdf(n, c)
        if (cdf < misrate) {
            a = c
        } else {
            b = c
        }
    }

    return if (edgeworthCdf(n, b) < misrate) b else a
}

/**
 * Edgeworth expansion for Wilcoxon signed-rank distribution CDF.
 */
private fun edgeworthCdf(n: Int, w: Long): Double {
    val mu = n.toDouble() * (n + 1) / 4.0
    val sigma2 = n * (n + 1.0) * (2 * n + 1) / 24.0
    val sigma = sqrt(sigma2)

    val z = (w - mu + 0.5) / sigma
    val phi = exp(-z * z / 2) / sqrt(2 * PI)
    val bigPhi = gaussCdf(z)

    val nk = n.toDouble()
    val kappa4 = -nk * (nk + 1) * (2 * nk + 1) * (3 * nk * nk + 3 * nk - 1) / 240.0

    val e3 = kappa4 / (24 * sigma2 * sigma2)

    val z2 = z * z
    val z3 = z2 * z
    val f3 = -phi * (z3 - 3 * z)

    val edgeworth = bigPhi + e3 * f3
    return min(max(edgeworth, 0.0), 1.0)
}

