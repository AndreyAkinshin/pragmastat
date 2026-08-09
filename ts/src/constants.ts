/**
 * Mathematical and numerical constants used across the library.
 */

/**
 * Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
 *
 * Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
 *
 * This is the smallest positive value representable in IEEE 754 binary64 format.
 * Unlike machine epsilon (which is the smallest ε where 1+ε ≠ 1), this is the
 * absolute smallest positive number before underflow to zero.
 *
 * Used by samplingCumulance to lift a drawn cumulance off zero, where the unbounded families
 * answer with an infinity. All language implementations use this same value to ensure
 * cross-language determinism in distribution sampling.
 */
export const SMALLEST_POSITIVE_SUBNORMAL = 5e-324;
