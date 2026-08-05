package dev.pragmastat

// Constants of the range reduction. ln 2 is split so that k*LN2_HI is exact: it carries 32
// significant bits and |k| needs at most 11, which leaves the product inside the 53 available.
private const val LN2_HI = 0.6931471803691238
private const val LN2_LO = 1.9082149292705877e-10
private const val LOG_SQRT2 = 1.4142135623730951

// Chebyshev fit of (log((1+s)/(1-s)) - 2*s) / s^3 over the reduced range. The leading terms are
// the recognizable 2/3, 2/5, 2/7 of the odd series doubled, which is where the assembly's missing
// factor of two lives; the tail departs from the series because the fit minimizes the worst error
// rather than matching Taylor.
private val LOG_POLY_P =
    doubleArrayOf(
        0.666666666666667,
        0.39999999999899505,
        0.28571428625975487,
        0.2222221113479508,
        0.18182889125261723,
        0.15331721600556042,
        0.14616449685043406,
    )

/**
 * The natural logarithm this toolkit evaluates, in place of the platform's.
 *
 * IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes nothing
 * about the logarithm. The reason this one has to be ours is stronger than for most
 * transcendentals: ratio is exp(median(log x - log y)), so a last-bit difference in log moves the
 * median's input and changes which pairwise element is selected. That is a different answer, not a
 * small error in the same one.
 *
 * The constants and the polynomial come from tests/oracles/fit_log.py, which documents the
 * reduction and what is fitted. Which of the two range endpoints a port names depends on how it
 * splits x: a library frexp returns a significand in [0.5, 1) and the reduction doubles the low
 * end, while reading the exponent field gives one in [1, 2) and it halves the high end. Both
 * arrive at the same [sqrt(2)/2, sqrt(2)) and the seven agree bit for bit; only the endpoint each
 * has to name differs.
 *
 * The special cases follow IEEE 754 and match what every platform returns for them, since they are
 * the cases a platform has no freedom about.
 */
internal fun logFunction(x: Double): Double {
    if (x.isNaN()) return x
    if (x < 0.0) return Double.NaN
    if (x == 0.0) return Double.NEGATIVE_INFINITY
    if (x.isInfinite()) return Double.POSITIVE_INFINITY
    if (x == 1.0) return 0.0

    // getExponent is undefined for subnormals, whose exponent field carries no information, so
    // the leading bit is brought into the normal range first. Scaling by a power of two is exact.
    var value = x
    var k = 0
    if (value < 2.2250738585072014e-308) {
        value *= 1.8014398509481984e16
        k -= 54
    }
    k += Math.getExponent(value)
    var m = Math.scalb(value, -Math.getExponent(value))

    // Recenter on [sqrt(2)/2, sqrt(2)) so |f| stays small at both ends. Halving is exact.
    if (m >= LOG_SQRT2) {
        m *= 0.5
        k += 1
    }

    val f = m - 1.0
    val s = f / (2.0 + f)
    val z = s * s

    var acc = LOG_POLY_P[LOG_POLY_P.size - 1]
    for (i in LOG_POLY_P.size - 2 downTo 0) {
        acc = acc * z + LOG_POLY_P[i]
    }
    val r = z * acc
    val hfsq = 0.5 * f * f
    val kf = k.toDouble()

    return if (k == 0) {
        f - (hfsq - s * (hfsq + r))
    } else {
        kf * LN2_HI - ((hfsq - (s * (hfsq + r) + kf * LN2_LO)) - f)
    }
}
