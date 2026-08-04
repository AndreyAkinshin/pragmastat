package dev.pragmastat

/**
 * Represents a unit of measurement with identity, family, and conversion support.
 *
 * Units within the same [family] are compatible and can be converted between each other.
 * The [baseUnits] value determines the conversion factor: a unit with smaller baseUnits
 * represents a finer (higher precision) measurement.
 *
 * An empty [abbreviation] is legal: the three dimensionless units have none.
 *
 * @throws IllegalArgumentException If [id] or [family] is empty, or if [baseUnits] is not positive.
 */
data class MeasurementUnit(
    /** Unique identifier for this unit (e.g., "ns", "us", "ms"). */
    val id: String,
    /** Unit family for compatibility checking (e.g., "Time", "Number"). */
    val family: String,
    /** Short display label (e.g., "ns", "ms"). Empty for dimensionless units. */
    val abbreviation: String,
    /** Human-readable name (e.g., "Nanosecond", "Number"). */
    val fullName: String,
    /** Number of base units this unit represents. Used for conversion factor calculation. */
    val baseUnits: Long,
) {
    init {
        // A non-positive baseUnits poisons every conversion through this unit, and it makes
        // finer pick it over any well-formed one. An empty family makes unrelated units
        // compatible.
        require(id.isNotEmpty()) { "id cannot be empty" }
        require(family.isNotEmpty()) { "family cannot be empty" }
        require(baseUnits > 0) { "baseUnits must be positive" }
    }

    /** Returns true if this unit is compatible (same family) with [other]. */
    fun isCompatible(other: MeasurementUnit): Boolean = family == other.family
}

/** Dimensionless numeric unit. Default unit for raw numeric samples. */
val NumberUnit = MeasurementUnit("number", "Number", "", "Number", 1)

/** Dimensionless ratio unit. Used by the Ratio estimator. */
val RatioUnit = MeasurementUnit("ratio", "Ratio", "", "Ratio", 1)

/** Dimensionless disparity (effect size) unit. Used by the Disparity estimator. */
val DisparityUnit = MeasurementUnit("disparity", "Disparity", "", "Disparity", 1)

/**
 * Returns the finer (higher precision) of two compatible units.
 * The unit with smaller [MeasurementUnit.baseUnits] is considered finer.
 */
fun finer(
    a: MeasurementUnit,
    b: MeasurementUnit,
): MeasurementUnit = if (a.baseUnits <= b.baseUnits) a else b

/**
 * Computes the multiplicative factor to convert values from [from] to [to].
 *
 * For example, if [from] is milliseconds (baseUnits=1_000_000) and [to] is
 * nanoseconds (baseUnits=1), the factor is 1_000_000.0.
 */
fun conversionFactor(
    from: MeasurementUnit,
    to: MeasurementUnit,
): Double = from.baseUnits.toDouble() / to.baseUnits.toDouble()

/**
 * Error thrown when attempting to combine or convert incompatible units.
 */
class UnitMismatchException(
    val unit1: MeasurementUnit,
    val unit2: MeasurementUnit,
) : IllegalArgumentException("can't convert ${unit1.fullName} to ${unit2.fullName}")
