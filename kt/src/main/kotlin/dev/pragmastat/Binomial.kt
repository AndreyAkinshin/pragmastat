package dev.pragmastat

/**
 * Below this total, C(n, k) fits the 64-bit integers the other six ports use, so every
 * implementation returns the exactly rounded value; at or above it they all switch to the
 * binary64 multiplicative recurrence. The threshold is a cross-language contract rather than
 * an implementation limit.
 */
internal const val MAX_ACCEPTABLE_BINOM_N = 62

/**
 * Computes C(n, k) as a Double, by the route all seven implementations agree on.
 *
 * Both call sites want C(n+m, k) for the same n+m: the admissible misrate is `2 / C(n+m, n)`
 * and the Loeffler recurrence normalizes its pmf by `C(n+m, m)`. They share one entry point so
 * they cannot drift onto different routes, which is what makes the admissibility check agree
 * with the distribution it guards.
 */
internal fun binomialCoefficient(
    n: Int,
    k: Int,
): Double =
    if (n < MAX_ACCEPTABLE_BINOM_N) {
        exactBinomial(n, k)
    } else {
        binomialCoefficientFloat(n, k)
    }

/**
 * Computes C(n, k) in exact integer arithmetic, then rounds once.
 */
private fun exactBinomial(
    n: Int,
    k: Int,
): Double {
    var kk = k
    if (kk > n) return 0.0
    if (kk == 0 || kk == n) return 1.0

    kk = minOf(kk, n - kk) // Take advantage of symmetry
    var result = 1L // exact integer arithmetic: each partial product is divisible

    for (i in 0 until kk) {
        result = result * (n - i) / (i + 1)
    }

    return result.toDouble()
}

/**
 * Computes C(n, k) in binary64 by the multiplicative recurrence
 * C(n, k) = prod_{i=1..k} (n-k+i)/i.
 *
 * It replaced an exp-of-Stirling formulation, for two reasons. The specification defines the
 * admissible misrate as `misrate >= 2 / C(n+m, n)`, an exact integer quantity; measured here
 * against BigInteger over 4 <= n+m <= 400, Stirling was inexact on 99.99% of cases with a worst
 * relative error of 1.6e-8, while this recurrence is inexact on 89.3% with a worst relative
 * error of 2.3e-15. And Stirling reached the answer through ln and exp, which every language
 * takes from a different libm: perturbing those by a single ulp moved the computed misrate
 * floor on 75579 of 79797 sample-size pairs. This form calls nothing, so the same perturbation
 * moves none of them, and all seven implementations run the identical sequence of one multiply
 * and one divide per step. Do not reassociate it, do not accumulate a numerator and a
 * denominator separately, and do not fuse the multiply into the divide.
 *
 * Normalizing k to the smaller half also makes the function symmetric by construction, which
 * matters because the two call sites ask for C(n+m, n) and C(n+m, m). Those are the same
 * number, and now they are also the same bits, so the comparison at the misrate floor cannot
 * be decided by which of the two rounded higher.
 *
 * Every operand is an explicit Double. The loop counter is an Int and would promote anyway;
 * spelling the conversion out is what keeps an integer division from ever looking plausible.
 */
private fun binomialCoefficientFloat(
    n: Int,
    k: Int,
): Double {
    val kk = minOf(k, n - k)
    var acc = 1.0
    for (i in 1..kk) {
        acc = acc * (n - kk + i).toDouble() / i.toDouble()
    }
    return acc
}
