package dev.pragmastat

import kotlin.math.exp
import kotlin.math.ln
import kotlin.math.max

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

    val (rLow, logCdfLow, logPmfHigh) = binomCdfSplit(n, target)
    val logTarget = ln(target)
    val logNum = if (logTarget > logCdfLow) logSubExp(logTarget, logCdfLow) else Double.NEGATIVE_INFINITY
    var p = if (logPmfHigh.isFinite() && logNum.isFinite()) exp(logNum - logPmfHigh) else 0.0
    p = p.coerceIn(0.0, 1.0)
    val u = rng.uniformDouble()
    val r = if (u < p) rLow + 1 else rLow
    return r * 2
}

private data class SplitResult(val rLow: Int, val logCdfLow: Double, val logPmfHigh: Double)

private fun binomCdfSplit(
    n: Int,
    target: Double,
): SplitResult {
    val logTarget = ln(target)
    var logPmf = -n.toDouble() * ln(2.0)
    var logCdf = logPmf
    var rLow = 0
    if (logCdf > logTarget) return SplitResult(0, logCdf, logPmf)
    for (k in 1..n) {
        val logPmfNext = logPmf + ln((n - k + 1).toDouble()) - ln(k.toDouble())
        val logCdfNext = logAddExp(logCdf, logPmfNext)
        if (logCdfNext > logTarget) return SplitResult(rLow, logCdf, logPmfNext)
        rLow = k
        logPmf = logPmfNext
        logCdf = logCdfNext
    }
    return SplitResult(rLow, logCdf, Double.NEGATIVE_INFINITY)
}

private fun logAddExp(
    a: Double,
    b: Double,
): Double {
    if (a == Double.NEGATIVE_INFINITY) return b
    if (b == Double.NEGATIVE_INFINITY) return a
    val m = max(a, b)
    return m + ln(exp(a - m) + exp(b - m))
}

private fun logSubExp(
    a: Double,
    b: Double,
): Double {
    if (b == Double.NEGATIVE_INFINITY) return a
    val diff = exp(b - a)
    return if (diff >= 1.0) Double.NEGATIVE_INFINITY else a + ln(1.0 - diff)
}
