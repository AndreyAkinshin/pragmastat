"""Mathematical and numerical constants used across the library."""

# Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
#
# Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
#
# This is the smallest positive value representable in IEEE 754 binary64 format.
# Unlike machine epsilon (which is the smallest ε where 1+ε ≠ 1), this is the
# absolute smallest positive number before underflow to zero.
#
# Used by pragmastat.distributions._sampling.sampling_cumulance to lift a drawn cumulance off
# zero, where the unbounded families answer with an infinity. All language implementations use
# this same value to ensure cross-language determinism in distribution sampling.
#
# Note: float.fromhex("0x1p-1074") produces the exact same value as 5e-324.
SMALLEST_POSITIVE_SUBNORMAL = 5e-324
