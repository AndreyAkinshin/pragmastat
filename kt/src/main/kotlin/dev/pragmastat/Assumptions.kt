package dev.pragmastat

enum class AssumptionId(val id: String) {
    VALIDITY("validity"),
    POSITIVITY("positivity"),
    SPARITY("sparity")
}

enum class Subject(val id: String) {
    X("x"),
    Y("y")
}

data class Violation(
    val id: AssumptionId,
    val subject: Subject
) {
    override fun toString(): String = "${id.id}(${subject.id})"
}

class AssumptionException(
    val violation: Violation
) : IllegalArgumentException(violation.toString())

internal fun checkValidity(values: List<Double>, subject: Subject, functionName: String) {
    if (values.isEmpty()) {
        throw AssumptionException(Violation(AssumptionId.VALIDITY, subject))
    }
    for (v in values) {
        if (v.isNaN() || v.isInfinite()) {
            throw AssumptionException(Violation(AssumptionId.VALIDITY, subject))
        }
    }
}

internal fun checkPositivity(values: List<Double>, subject: Subject, functionName: String) {
    for (v in values) {
        if (v <= 0.0) {
            throw AssumptionException(Violation(AssumptionId.POSITIVITY, subject))
        }
    }
}

internal fun checkSparity(values: List<Double>, subject: Subject, functionName: String) {
    if (values.size < 2) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, subject))
    }
    val spreadVal = fastSpread(values)
    if (spreadVal <= 0.0) {
        throw AssumptionException(Violation(AssumptionId.SPARITY, subject))
    }
}
