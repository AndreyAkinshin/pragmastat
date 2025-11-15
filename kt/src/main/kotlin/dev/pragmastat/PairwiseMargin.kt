package dev.pragmastat

import kotlin.math.*

private const val MAX_EXACT_SIZE = 400
private const val MAX_ACCEPTABLE_BINOM_N = 65

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
 * @throws IllegalArgumentException if n <= 0, m <= 0, or misrate is outside [0, 1]
 */
fun pairwiseMargin(n: Int, m: Int, misrate: Double): Int {
    require(n > 0) { "n must be positive" }
    require(m > 0) { "m must be positive" }
    require(misrate in 0.0..1.0) { "misrate must be in range [0, 1]" }

    return if (n + m <= MAX_EXACT_SIZE) {
        pairwiseMarginExact(n, m, misrate)
    } else {
        pairwiseMarginApprox(n, m, misrate)
    }
}

/**
 * Uses the exact distribution based on Loeffler's recurrence
 */
private fun pairwiseMarginExact(n: Int, m: Int, misrate: Double): Int {
    return pairwiseMarginExactRaw(n, m, misrate / 2.0) * 2
}

/**
 * Uses Edgeworth approximation for large samples
 */
private fun pairwiseMarginApprox(n: Int, m: Int, misrate: Double): Int {
    return pairwiseMarginApproxRaw(n, m, misrate / 2.0) * 2
}

/**
 * Inversed implementation of Andreas Löffler's (1982)
 * "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
 */
private fun pairwiseMarginExactRaw(n: Int, m: Int, p: Double): Int {
    val total = if (n + m < MAX_ACCEPTABLE_BINOM_N) {
        binomialCoefficient(n + m, m)
    } else {
        binomialCoefficientFloat(n + m, m)
    }

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
private fun pairwiseMarginApproxRaw(n: Int, m: Int, misrate: Double): Int {
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
private fun edgeworthCdf(n: Int, m: Int, u: Long): Double {
    val nf = n.toDouble()
    val mf = m.toDouble()
    val uf = u.toDouble()

    val mu = (nf * mf) / 2.0
    val su = sqrt((nf * mf * (nf + mf + 1.0)) / 12.0)
    val z = (uf - mu - 0.5) / su

    // Standard normal PDF and CDF
    val phi = exp((-z * z) / 2.0) / sqrt(2.0 * PI)
    val bigPhi = gauss(z)

    // Pre-compute powers of n and m for efficiency
    val n2 = nf * nf
    val n3 = n2 * nf
    val n4 = n2 * n2
    val m2 = mf * mf
    val m3 = m2 * mf
    val m4 = m2 * m2

    // Compute moments
    val mu2 = (nf * mf * (nf + mf + 1.0)) / 12.0
    val mu4 = (nf * mf * (nf + mf + 1.0) *
            (5.0 * mf * nf * (mf + nf) - 2.0 * (m2 + n2) + 3.0 * mf * nf - 2.0 * (nf + mf))) / 240.0

    val mu6 = (nf * mf * (nf + mf + 1.0) *
            (35.0 * m2 * n2 * (m2 + n2) +
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
                    8.0 * (nf + mf))) / 4032.0

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

/**
 * Computes the standard normal CDF using ACM Algorithm 209
 *
 * Calculates (1/sqrt(2*pi)) * integral from -infinity to x of e^(-u^2/2) du
 * by means of polynomial approximations due to A. M. Murray of Aberdeen University.
 *
 * See: http://dl.acm.org/citation.cfm?id=367664
 *
 * @param x -infinity..+infinity
 * @return Area under the Standard Normal Curve from -infinity to x
 */
private fun gauss(x: Double): Double {
    val z: Double
    if (abs(x) < 1e-9) {
        z = 0.0
    } else {
        var y = abs(x) / 2
        if (y >= 3.0) {
            z = 1.0
        } else if (y < 1.0) {
            val w = y * y
            z = ((((((((0.000124818987 * w - 0.001075204047) * w + 0.005198775019) * w - 0.019198292004) * w +
                    0.059054035642) * w - 0.151968751364) * w + 0.319152932694) * w - 0.531923007300) * w +
                    0.797884560593) * y * 2.0
        } else {
            y = y - 2.0
            z = (((((((((((((-0.000045255659 * y + 0.000152529290) * y - 0.000019538132) * y - 0.000676904986) *
                    y + 0.001390604284) * y - 0.000794620820) * y - 0.002034254874) * y +
                    0.006549791214) * y - 0.010557625006) * y + 0.011630447319) * y - 0.009279453341) * y +
                    0.005353579108) * y - 0.002141268741) * y + 0.000535310849) * y + 0.999936657524
        }
    }

    return if (x > 0.0) (z + 1.0) / 2 else (1.0 - z) / 2
}

/**
 * Computes binomial coefficient C(n, k) using integer arithmetic
 */
private fun binomialCoefficient(n: Int, k: Int): Double {
    var kk = k
    if (kk > n) return 0.0
    if (kk == 0 || kk == n) return 1.0

    kk = minOf(kk, n - kk) // Take advantage of symmetry
    var result = 1.0

    for (i in 0 until kk) {
        result = result * (n - i) / (i + 1)
    }

    return result
}

/**
 * Computes binomial coefficient using floating-point logarithms for large values
 */
private fun binomialCoefficientFloat(n: Int, k: Int): Double {
    var kk = k
    if (kk > n) return 0.0
    if (kk == 0 || kk == n) return 1.0

    kk = minOf(kk, n - kk) // Take advantage of symmetry

    // Use log-factorial function: C(n, k) = exp(log(n!) - log(k!) - log((n-k)!))
    val logResult = logFactorial(n) - logFactorial(kk) - logFactorial(n - kk)
    return exp(logResult)
}

/**
 * Computes the natural logarithm of n!
 */
private fun logFactorial(n: Int): Double {
    if (n == 0 || n == 1) return 0.0

    val x = (n + 1).toDouble() // n! = Gamma(n+1)

    if (x < 1e-5) return 0.0

    // DONT TOUCH: Stirling's approximation is inaccurate for small x.
    // Use Gamma recurrence: Gamma(x) = Gamma(x+k) / (x*(x+1)*...*(x+k-1))
    // These branches appear unreachable in current usage (n+m >= 65), but
    // are retained for correctness if the function is used in other contexts.
    return when {
        x < 1.0 -> stirlingApproxLog(x + 3.0) - ln(x * (x + 1.0) * (x + 2.0))
        x < 2.0 -> stirlingApproxLog(x + 2.0) - ln(x * (x + 1.0))
        x < 3.0 -> stirlingApproxLog(x + 1.0) - ln(x)
        else -> stirlingApproxLog(x)
    }
}

/**
 * Stirling's approximation with Bernoulli correction
 */
private fun stirlingApproxLog(x: Double): Double {
    var result = x * ln(x) - x + ln(2.0 * PI / x) / 2.0

    // Bernoulli correction series
    val B2 = 1.0 / 6.0
    val B4 = -1.0 / 30.0
    val B6 = 1.0 / 42.0
    val B8 = -1.0 / 30.0
    val B10 = 5.0 / 66.0

    val x2 = x * x
    val x3 = x2 * x
    val x5 = x3 * x2
    val x7 = x5 * x2
    val x9 = x7 * x2

    result += B2 / (2.0 * x) + B4 / (12.0 * x3) + B6 / (30.0 * x5) + B8 / (56.0 * x7) + B10 / (90.0 * x9)

    return result
}
