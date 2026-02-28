package dev.pragmastat

/**
 * Represents a unit of measurement with identity, family, and conversion support.
 *
 * Units within the same [family] are compatible and can be converted between each other.
 * The [baseUnits] value determines the conversion factor: a unit with smaller baseUnits
 * represents a finer (higher precision) measurement.
 */
interface MeasurementUnit {
    /** Unique identifier for this unit (e.g., "ns", "us", "ms"). */
    val id: String

    /** Unit family for compatibility checking (e.g., "Time", "Number"). */
    val family: String

    /** Short display label (e.g., "ns", "ms"). Empty for dimensionless units. */
    val abbreviation: String

    /** Human-readable name (e.g., "Nanosecond", "Number"). */
    val fullName: String

    /** Number of base units this unit represents. Used for conversion factor calculation. */
    val baseUnits: Long

    /** Returns true if this unit is compatible (same family) with [other]. */
    fun isCompatible(other: MeasurementUnit): Boolean = family == other.family
}

/**
 * Sealed interface for the standard built-in units (Number, Ratio, Disparity).
 */
sealed interface StandardUnit : MeasurementUnit

/** Dimensionless numeric unit. Default unit for raw numeric samples. */
data object NumberUnit : StandardUnit {
    override val id: String = "number"
    override val family: String = "Number"
    override val abbreviation: String = ""
    override val fullName: String = "Number"
    override val baseUnits: Long = 1L
}

/** Dimensionless ratio unit. Used by the Ratio estimator. */
data object RatioUnit : StandardUnit {
    override val id: String = "ratio"
    override val family: String = "Ratio"
    override val abbreviation: String = ""
    override val fullName: String = "Ratio"
    override val baseUnits: Long = 1L
}

/** Dimensionless disparity (effect size) unit. Used by the Disparity estimator. */
data object DisparityUnit : StandardUnit {
    override val id: String = "disparity"
    override val family: String = "Disparity"
    override val abbreviation: String = ""
    override val fullName: String = "Disparity"
    override val baseUnits: Long = 1L
}

/**
 * A user-defined measurement unit.
 *
 * @property id Unique identifier
 * @property family Unit family for compatibility checking
 * @property abbreviation Short display label
 * @property fullName Human-readable name
 * @property baseUnits Number of base units (for conversion factor calculation)
 */
data class CustomUnit(
    override val id: String,
    override val family: String,
    override val abbreviation: String,
    override val fullName: String,
    override val baseUnits: Long,
) : MeasurementUnit

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
