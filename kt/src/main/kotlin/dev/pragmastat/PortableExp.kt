package dev.pragmastat

import kotlin.math.floor

// Constants of the range reduction, emitted by tests/oracles/fit_exp.py.
//
// ln 2 is split so that k*LN2_HI is exact: LN2_HI carries 33 significant bits and |k| needs at
// most 11, which leaves the product inside the 53 available. Without the split the reduction
// would lose the low bits of r, and r is where the accuracy lives.
private const val INV_LN2 = 1.4426950408889634e+00
private const val LN2_HI = 6.9314718036912382e-01
private const val LN2_LO = 1.9082149292705877e-10

/**
 * The exponential every port evaluates, in place of the platform's.
 *
 * IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
 * nothing about the exponential. Conforming libraries disagree in the last bit and do: measured
 * on one Edgeworth crossover, Go's software exp and glibc's return neighbouring values, which
 * moved the reported margin by two. Since a margin selects an order statistic, that is a
 * different confidence interval from the same inputs.
 *
 * So the exponential cannot be the platform's. This one is built from operations the standard
 * does fix:
 *
 *     k = floor(y/ln2 + 1/2),  r = (y - k*ln2Hi) - k*ln2Lo,  exp(y) = 2^k * exp(r)
 *
 * with exp(r) on |r| <= ln2/2 from the polynomial in tests/oracles/fit_exp.py. Fitting
 * (exp(r) - 1 - r)/r^2 rather than exp(r) keeps the two leading terms exact and leaves the
 * polynomial supplying a correction below 0.07, so the assembled result stays within a couple
 * of ulp of the true exponential while being reproducible everywhere.
 *
 * floor(x + 1/2) rather than a rounding function: Go rounds halves away from zero and R rounds
 * them to even, so naming a rounding is naming a disagreement.
 *
 * No rounding pins are needed here: the JVM never contracts a multiply and an add into a fused
 * multiply-add, so every product lands in binary64 on its own, which is what the Go port has to
 * spell out with explicit float64 conversions.
 *
 * @param y Exponent
 * @return e raised to the power of [y]
 */
internal fun portableExp(y: Double): Double {
    if (y.isNaN()) {
        return y
    }
    // Past these the answer is not in doubt, and stating the cutoffs keeps the reduction from
    // having to produce a k it cannot scale by.
    if (y > 709.79) {
        return Double.POSITIVE_INFINITY
    }
    if (y < -745.2) {
        return 0.0
    }

    val k = floor(y * INV_LN2 + 0.5)
    val r = (y - k * LN2_HI) - k * LN2_LO

    var q = 1.6086622436215554e-10
    q = q * r + 2.0918129454967065e-09
    q = q * r + 2.5052071109214447e-08
    q = q * r + 2.7557263301474753e-07
    q = q * r + 2.7557319247204789e-06
    q = q * r + 2.4801587336421322e-05
    q = q * r + 1.9841269841263304e-04
    q = q * r + 1.3888888888879082e-03
    q = q * r + 8.3333333333333332e-03
    q = q * r + 4.1666666666666678e-02
    q = q * r + 1.6666666666666666e-01
    q = q * r + 5.0000000000000000e-01
    val p = 1.0 + r + r * r * q

    // Two scalings rather than one. Splitting k in half keeps the first factor inside the
    // normal range whatever k is, so only the second can denormalise or overflow, and it does
    // so in a single rounding.
    val ki = k.toInt()
    val half = ki / 2
    return (p * Math.scalb(1.0, half)) * Math.scalb(1.0, ki - half)
}
