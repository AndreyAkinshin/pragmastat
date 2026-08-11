package dev.pragmastat

/**
 * Shared sample preparation for the distribution estimation models.
 *
 * Runs the two assumptions that follow validity in the canonical order:
 * `domain(x)` (at least two observations and a finite representable range) and
 * `distinctness(x)` (no repeated value; -0.0 and +0.0 count as repeated).
 * The caller's list is never retained: the model always owns a private copy.
 */
internal object DistSampleGuard {
    fun prepare(
        values: List<Double>,
        assumeSorted: Boolean,
    ): DoubleArray {
        val copy = values.toDoubleArray()
        if (!assumeSorted) {
            copy.sort()
        }

        val n = copy.size
        if (n < 2) {
            throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
        }

        val range = copy[n - 1] - copy[0]
        if (range.isNaN() || range.isInfinite()) {
            throw AssumptionException(Violation(AssumptionId.DOMAIN, Subject.X))
        }

        for (i in 1 until n) {
            if (copy[i - 1] == copy[i]) {
                throw AssumptionException(Violation(AssumptionId.DISTINCTNESS, Subject.X))
            }
        }

        return copy
    }

    /** Attaches a unit to every element of a query result. */
    fun toMeasurements(
        values: List<Double>,
        unit: MeasurementUnit,
    ): List<Measurement> = values.map { Measurement(it, unit) }

    /**
     * Converts a measured query vector into the model's own unit.
     *
     * A query in a compatible but different unit is converted rather than rejected, matching the
     * two-sample estimators, which reconcile their inputs instead of demanding an exact match.
     * An incompatible unit is a caller error and is reported as one.
     */
    fun toRaw(
        values: List<Measurement>,
        unit: MeasurementUnit,
    ): List<Double> =
        values.map { measurement ->
            val source = measurement.unit
            if (!source.isCompatible(unit)) {
                throw UnitMismatchException(source, unit)
            }
            measurement.value * conversionFactor(source, unit)
        }
}
