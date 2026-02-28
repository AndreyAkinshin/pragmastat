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
    val unit: MeasurementUnit,
) {
    /** Returns the numeric value (alias for [value]). */
    fun toDouble(): Double = value

    override fun toString(): String {
        return if (unit.abbreviation.isNotEmpty()) {
            "${formatValue(value)} ${unit.abbreviation}"
        } else {
            formatValue(value)
        }
    }
}

private fun formatValue(v: Double): String {
    // Use G format to match Go's FormatFloat behavior
    val s = v.toBigDecimal().stripTrailingZeros().toPlainString()
    return s
}
