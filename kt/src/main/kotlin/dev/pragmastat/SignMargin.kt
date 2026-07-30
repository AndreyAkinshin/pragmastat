package dev.pragmastat

// The randomized sign margin, computed without a single library call.
//
// It used to be evaluated in log space: nine calls to ln and exp, one of them inside a loop that
// runs n times. IEEE 754 fixes nothing about either function, and this value is not merely
// returned to the caller: the margin selects an order statistic, so a difference between two
// conforming implementations becomes a different confidence interval from identical inputs. It
// did. Two ports disagreed on spreadBounds for a sample of 200 consecutive integers.
//
// No logarithm is needed. Binomial(n, 1/2) has an exact rational distribution function, and the
// two quantities the randomization wants are its partial sum and the next term. Both follow from
// the same multiplicative recurrence the binomial coefficient uses: one multiply and one divide
// per step, plus a scaling by a power of two, and IEEE 754 pins all three.
//
// The scaling is what makes the recurrence work at any n. pmf(0) is 2^-n, which underflows to zero
// past n = 1074, so the running term is carried as w * 2^e with the exponent tracked separately:
// w stays in the normal range and e absorbs the magnitude. Rescaling happens by multiplying by a
// power of two, which is exact, so it costs no accuracy and changes no bits.
//
// Measured against exact rational arithmetic over 195 (n, misrate) pairs spanning n = 1 to 5000
// and misrate from 1 down to the smallest positive double: the selected index is right every time,
// and the randomization probability is within 6.1e-13. The log-space version it replaces reached
// 1.9e-11 on the same set, thirty times further out, and did it differently in each port.

/**
 * How far the running term is rescaled when it grows too large. Any power of two works; 512 keeps
 * the rescaling rare without letting w approach the overflow threshold.
 */
private const val SCALE_STEP = 512

internal fun signMarginRandomized(
    n: Int,
    misrate: Double,
    rng: Rng,
): Int {
    if (n <= 0) throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    if (misrate.isNaN() || misrate < 0.0 || misrate > 1.0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }
    val minMisrate = minAchievableMisrateOneSample(n)
    if (misrate < minMisrate) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.MISRATE))
    }

    val target = misrate / 2.0
    if (target <= 0.0) return 0
    if (target >= 1.0) return n * 2

    val (rLow, p) = binomCdfSplit(n, target)
    val u = rng.uniformDouble()
    val r = if (u < p) rLow + 1 else rLow
    return r * 2
}

/**
 * The largest k whose Binomial(n, 0.5) CDF does not exceed target, together with the fraction of
 * the next term that would be needed to reach it. The caller compares that fraction against a
 * uniform draw, which is what makes the margin achieve the requested misrate exactly rather than
 * the next admissible one below it.
 */
private fun binomCdfSplit(
    n: Int,
    target: Double,
): Pair<Int, Double> {
    // Binomial(n, 1/2) is symmetric, so for odd n the distribution function at (n-1)/2 is exactly
    // one half. No approximation reproduces an exact equality, and misrate = 1 lands on it: the
    // summation would decide the comparison by its last accumulated bit.
    if (target == 0.5 && n % 2 == 1) {
        return Pair((n - 1) / 2, 0.0)
    }

    val scaleUp = ldexp(1.0, SCALE_STEP)
    val scaleDown = ldexp(1.0, -SCALE_STEP)

    // The running term pmf(k) is w * 2^e, starting from pmf(0) = 2^-n.
    var w = 1.0
    var e = -n
    var cdf = 1.0

    if (ldexp(cdf, e) > target) return Pair(0, 0.0)

    var rLow = 0
    for (k in 1..n) {
        w = w * (n - k + 1).toDouble() / k.toDouble()
        while (w > scaleUp) {
            w *= scaleDown
            cdf *= scaleDown
            e += SCALE_STEP
        }
        val next = cdf + w
        if (ldexp(next, e) > target) {
            // target and cdf are both in units of 2^e here, so the fraction is a plain quotient.
            val p = (ldexp(target, -e) - cdf) / w
            return Pair(rLow, p.coerceIn(0.0, 1.0))
        }
        rLow = k
        cdf = next
    }

    return Pair(rLow, 0.0)
}

/**
 * v * 2^exp, exactly, including where the result leaves the normal range.
 *
 * Kotlin has no ldexp. Scaling by a power of two is exact wherever the result is representable, so
 * an out-of-range exponent is split into steps that are not.
 */
private fun ldexp(
    v: Double,
    exp: Int,
): Double {
    var result = v
    var remaining = exp
    while (remaining > 1023) {
        result *= Double.fromBits(2046L shl 52)
        remaining -= 1023
    }
    while (remaining < -1022) {
        result *= Double.fromBits(1L shl 52)
        remaining += 1022
    }
    return result * Double.fromBits((1023L + remaining) shl 52)
}
