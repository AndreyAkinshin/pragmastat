package dev.pragmastat.distributions

/**
 * `sqrt(2 * pi)`, the normalizing constant of the normal density.
 *
 * A literal rather than `sqrt(2 * PI)`: the seven ports must divide by the same double, and a
 * runtime square root of a runtime product is one more place for them to differ.
 */
internal const val SQRT_2PI = 2.506628274631000241612355239340
