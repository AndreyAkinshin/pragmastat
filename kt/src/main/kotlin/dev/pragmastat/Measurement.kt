package dev.pragmastat

/**
 * A numeric value paired with a [MeasurementUnit].
 *
 * Returned by point estimators (center, spread, shift, ratio, disparity).
 *
 * @property value The numeric value
 * @property unit The unit of measurement
 */
data class Measurement(
    val value: Double,
    val unit: MeasurementUnit = NumberUnit,
) {
    /** Returns the numeric value (alias for [value]). */
    fun toDouble(): Double = value

    override fun toString(): String =
        if (unit.abbreviation.isNotEmpty()) {
            "${formatValue(value)} ${unit.abbreviation}"
        } else {
            formatValue(value)
        }
}

/**
 * States a double the way the other ports state it: 86.0 prints as 86, so a carrier built here
 * reads the same as the one Go and Rust print. Shared by both carriers so neither can drift.
 */
internal fun formatValue(v: Double): String = v.toBigDecimal().stripTrailingZeros().toPlainString()
