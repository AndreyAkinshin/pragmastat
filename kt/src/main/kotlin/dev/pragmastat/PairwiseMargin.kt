package dev.pragmastat

import kotlin.math.*

private const val MAX_EXACT_SIZE = 400

/**
 * PairwiseMargin determines how many extreme pairwise differences to exclude
 * when constructing bounds based on the distribution of dominance statistics.
 * Uses exact calculation for small samples (n+m <= 400) and Edgeworth
 * approximation for larger samples.
 *
 * @param n Sample size of first sample (must be positive)
 * @param m Sample size of second sample (must be positive)
 * @param misrate Misclassification rate (must be in [0, 1])
 * @return Integer representing the total margin split between lower and upper tails
 * @throws AssumptionException if n <= 0, m <= 0, or misrate is outside [0, 1]
 */
internal fun pairwiseMargin(
    n: Int,
    m: Int,
    misrate: Double,
): Int {
    if (n <= 0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }
    if (m <= 0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.Y))
    }
    if (misrate < 0.0 || misrate > 1.0 || misrate.isNaN()) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val minMisrate = minAchievableMisrateTwoSample(n, m)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    return if (n + m <= MAX_EXACT_SIZE) {
        pairwiseMarginExact(n, m, misrate)
    } else {
        pairwiseMarginApprox(n, m, misrate)
    }
}

/**
 * Uses the exact distribution based on Loeffler's recurrence
 */
private fun pairwiseMarginExact(
    n: Int,
    m: Int,
    misrate: Double,
): Int = pairwiseMarginExactRaw(n, m, misrate / 2.0) * 2

/**
 * Uses Edgeworth approximation for large samples
 */
private fun pairwiseMarginApprox(
    n: Int,
    m: Int,
    misrate: Double,
): Int = pairwiseMarginApproxRaw(n, m, misrate / 2.0) * 2

/**
 * Inversed implementation of Andreas Löffler's (1982)
 * "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
 */
private fun pairwiseMarginExactRaw(
    n: Int,
    m: Int,
    p: Double,
): Int {
    val total = binomialCoefficient(n + m, m)

    val pmf = mutableListOf(1.0) // pmf[0] = 1
    val sigma = mutableListOf(0.0) // sigma[0] is unused

    var u = 0
    var cdf = 1.0 / total

    if (cdf >= p) {
        return 0
    }

    while (true) {
        u++

        // Ensure sigma has entry for u
        if (sigma.size <= u) {
            var value = 0
            for (d in 1..n) {
                if (u % d == 0 && u >= d) {
                    value += d
                }
            }
            for (d in (m + 1)..(m + n)) {
                if (u % d == 0 && u >= d) {
                    value -= d
                }
            }
            sigma.add(value.toDouble())
        }

        // Compute pmf[u] using Loeffler recurrence
        var sum = 0.0
        for (i in 0 until u) {
            sum += pmf[i] * sigma[u - i]
        }
        sum /= u
        pmf.add(sum)

        cdf += sum / total
        if (cdf >= p) {
            return u
        }
        if (sum == 0.0) {
            break
        }
    }

    return pmf.size - 1
}

/**
 * Inverse Edgeworth Approximation
 */
private fun pairwiseMarginApproxRaw(
    n: Int,
    m: Int,
    misrate: Double,
): Int {
    var a = 0L
    var b = n.toLong() * m.toLong()
    while (a < b - 1) {
        val c = (a + b) / 2
        val p = edgeworthCdf(n, m, c)
        if (p < misrate) {
            a = c
        } else {
            b = c
        }
    }

    val result = if (edgeworthCdf(n, m, b) < misrate) b else a
    require(result <= Int.MAX_VALUE) { "Pairwise margin exceeds supported range for n=$n, m=$m" }
    return result.toInt()
}

/**
 * Computes the CDF using Edgeworth expansion
 */
private fun edgeworthCdf(
    n: Int,
    m: Int,
    u: Long,
): Double {
    val nf = n.toDouble()
    val mf = m.toDouble()
    val uf = u.toDouble()

    val mu = (nf * mf) / 2.0
    val su = sqrt((nf * mf * (nf + mf + 1.0)) / 12.0)
    // -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
    val z = (uf - mu - 0.5) / su

    // Standard normal PDF and CDF
    val phi = portableExp((-z * z) / 2.0) / sqrt(2.0 * PI)
    val bigPhi = gaussCdf(z)

    // Pre-compute powers of n and m for efficiency
    val n2 = nf * nf
    val n3 = n2 * nf
    val n4 = n2 * n2
    val m2 = mf * mf
    val m3 = m2 * mf
    val m4 = m2 * m2

    // Compute moments
    val mu2 = (nf * mf * (nf + mf + 1.0)) / 12.0
    val mu4 =
        (
            nf * mf * (nf + mf + 1.0) *
                (5.0 * mf * nf * (mf + nf) - 2.0 * (m2 + n2) + 3.0 * mf * nf - 2.0 * (nf + mf))
        ) / 240.0

    val mu6 =
        (
            nf * mf * (nf + mf + 1.0) *
                (
                    35.0 * m2 * n2 * (m2 + n2) +
                        70.0 * m3 * n3 -
                        42.0 * mf * nf * (m3 + n3) -
                        14.0 * m2 * n2 * (nf + mf) +
                        16.0 * (n4 + m4) -
                        52.0 * nf * mf * (n2 + m2) -
                        43.0 * n2 * m2 +
                        32.0 * (m3 + n3) +
                        14.0 * mf * nf * (nf + mf) +
                        8.0 * (n2 + m2) +
                        16.0 * nf * mf -
                        8.0 * (nf + mf)
                )
        ) / 4032.0

    // Pre-compute powers of mu2 and related terms
    val mu2_2 = mu2 * mu2
    val mu2_3 = mu2_2 * mu2
    val mu4_mu2_2 = mu4 / mu2_2

    // Factorial constants: 4! = 24, 6! = 720, 8! = 40320
    val e3 = (mu4_mu2_2 - 3.0) / 24.0
    val e5 = (mu6 / mu2_3 - 15.0 * mu4_mu2_2 + 30.0) / 720.0
    val e7 = 35.0 * (mu4_mu2_2 - 3.0) * (mu4_mu2_2 - 3.0) / 40320.0

    // Pre-compute powers of z for Hermite polynomials
    val z2 = z * z
    val z3 = z2 * z
    val z5 = z3 * z2
    val z7 = z5 * z2

    // Hermite polynomial derivatives: f_n = -phi * H_n(z)
    val f3 = -phi * (z3 - 3.0 * z)
    val f5 = -phi * (z5 - 10.0 * z3 + 15.0 * z)
    val f7 = -phi * (z7 - 21.0 * z5 + 105.0 * z3 - 105.0 * z)

    // Edgeworth expansion
    val edgeworth = bigPhi + e3 * f3 + e5 * f5 + e7 * f7

    // Clamp to [0, 1]
    return max(0.0, min(1.0, edgeworth))
}
