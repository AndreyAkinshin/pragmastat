package dev.pragmastat

/**
 * A probability: a [Double] constrained to the closed interval [0, 1].
 *
 * Used by the typed (Sample-based) public APIs to express misclassification
 * rates and similar [0, 1] parameters with a self-documenting value type that
 * validates at the type boundary. The raw (List-based) APIs keep plain [Double].
 *
 * This is a zero-overhead inline wrapper: at runtime it is represented as a bare
 * [Double] wherever the compiler can prove it. Construction validates the range
 * (also rejecting NaN), throwing [IllegalArgumentException] otherwise.
 *
 * @property value The underlying probability value, guaranteed to be in [0, 1].
 */
@JvmInline
value class Probability(val value: Double) {
    init {
        require(value in 0.0..1.0) { "Probability must be in [0, 1], got $value" }
    }
}
