# Numerical constants for cross-language consistency in distribution sampling.
# This file is named with 'aaa_' prefix to ensure it loads before other R files.

# Machine epsilon for IEEE 754 double-precision (binary64).
# Value: 2^(-52) ≈ 2.220446049250313e-16
#
# This is the smallest eps such that 1.0 + eps != 1.0 in float64 arithmetic.
# Represents the distance between 1.0 and the next representable number.
#
# Used to avoid log(0) or division by zero when uniform_float() returns exactly 1.0.
# All language implementations use this same value to ensure cross-language
# determinism in distribution sampling.
.MACHINE_EPSILON <- 2.220446049250313e-16

# Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
# Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
#
# This is the smallest positive value representable in IEEE 754 binary64 format.
# Unlike machine epsilon (which is the smallest eps where 1+eps != 1), this is the
# absolute smallest positive number before underflow to zero.
#
# Used to avoid log(0) in Box-Muller transform when uniform_float() returns exactly 0.
# All language implementations use this same value to ensure cross-language
# determinism in distribution sampling.
.SMALLEST_POSITIVE_SUBNORMAL <- 5e-324

#' sqrt(2 * pi), the normalizing constant of the normal density.
#'
#' A literal rather than sqrt(2 * pi) computed at load time: the seven ports must divide by the
#' same double, and a runtime square root of a runtime product is one more place for them to
#' differ.
#'
#' @noRd
.SQRT_2PI <- 2.506628274631000241612355239340
