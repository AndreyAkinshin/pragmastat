package dev.pragmastat

import kotlin.math.pow

/**
 * Computes the minimum achievable misrate for one-sample bounds.
 *
 * For sample size n, this equals 2^(1-n), representing the irreducible
 * error rate when using the entire sample range as bounds.
 *
 * @param n Sample size (must be positive)
 * @return Minimum achievable misrate in [0, 1]
 */
internal fun minAchievableMisrateOneSample(n: Int): Double {
    if (n <= 0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }
    return 2.0.pow(1 - n)
}

/**
 * Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.
 *
 * @param n Size of first sample (must be positive)
 * @param m Size of second sample (must be positive)
 * @return Minimum achievable misrate
 */
internal fun minAchievableMisrateTwoSample(
    n: Int,
    m: Int,
): Double {
    if (n <= 0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
    }
    if (m <= 0) {
        throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.Y))
    }
    // Shares one binomial with the exact pairwise-margin distribution (see Binomial.kt):
    // the floor and the distribution it guards must be computed by the same route.
    return 2.0 / binomialCoefficient(n + m, n)
}
