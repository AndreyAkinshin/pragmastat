package dev.pragmastat

enum class AssumptionId(val id: String) {
    VALIDITY("validity"),
    DOMAIN("domain"),
    POSITIVITY("positivity"),
    SPARITY("sparity"),
}

enum class Subject(val id: String) {
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
): List<Double> {
    return values.map { v ->
        if (v <= 0.0) {
            throw AssumptionException(Violation(AssumptionId.POSITIVITY, subject))
        }
        kotlin.math.ln(v)
    }
}
