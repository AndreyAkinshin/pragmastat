package dev.pragmastat

/**
 * Assumption identifiers in canonical priority order: validity, domain, positivity,
 * then the shared level of sparity and distinctness, then cumulance.
 */
enum class AssumptionId(
    val id: String,
) {
    VALIDITY("validity"),
    DOMAIN("domain"),
    POSITIVITY("positivity"),
    SPARITY("sparity"),
    DISTINCTNESS("distinctness"),
    CUMULANCE("cumulance"),
}

enum class Subject(
    val id: String,
) {
    X("x"),
    Y("y"),
    MISRATE("misrate"),
}

data class Violation(
    val id: AssumptionId,
    val subject: Subject,
) {
    override fun toString(): String = "${id.id}(${subject.id})"
}

class AssumptionException : IllegalArgumentException {
    val violation: Violation?

    constructor(violation: Violation) : super(violation.toString()) {
        this.violation = violation
    }

    constructor(message: String) : super(message) {
        this.violation = null
    }
}

internal fun checkValidity(
    values: List<Double>,
    subject: Subject,
) {
    if (values.isEmpty()) {
        throw AssumptionException(Violation(AssumptionId.VALIDITY, subject))
    }
    for (v in values) {
        if (v.isNaN() || v.isInfinite()) {
            throw AssumptionException(Violation(AssumptionId.VALIDITY, subject))
        }
    }
}

/**
 * Checks the validity assumption for a Distribution Estimation query vector.
 *
 * A query vector is a list of questions asked of a model, not a sample, so an empty
 * vector satisfies the assumption vacuously. Every element that is present must be finite.
 */
internal fun checkQueryValidity(values: List<Double>) {
    for (v in values) {
        if (v.isNaN() || v.isInfinite()) {
            throw AssumptionException(Violation(AssumptionId.VALIDITY, Subject.Y))
        }
    }
}

/**
 * Checks that every queried cumulance lies in the closed unit interval.
 */
internal fun checkCumulance(values: List<Double>) {
    for (v in values) {
        if (!(v >= 0.0 && v <= 1.0)) {
            throw AssumptionException(Violation(AssumptionId.CUMULANCE, Subject.Y))
        }
    }
}

internal fun checkPositivity(
    values: List<Double>,
    subject: Subject,
) {
    for (v in values) {
        if (v <= 0.0) {
            throw AssumptionException(Violation(AssumptionId.POSITIVITY, subject))
        }
    }
}

/**
 * Log-transforms a list. Throws AssumptionException if any value is non-positive.
 */
internal fun log(
    values: List<Double>,
    subject: Subject,
): List<Double> =
    values.map { v ->
        if (v <= 0.0) {
            throw AssumptionException(Violation(AssumptionId.POSITIVITY, subject))
        }
        logFunction(v)
    }
