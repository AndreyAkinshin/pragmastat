# Numerical constants for cross-language consistency in distribution sampling.
# This file is named with 'aaa_' prefix to ensure it loads before other R files.

# Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
# Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
#
# This is the smallest positive value representable in IEEE 754 binary64 format.
# Unlike machine epsilon (which is the smallest eps where 1+eps != 1), this is the
# absolute smallest positive number before underflow to zero.
#
# Used by .sampling_cumulance() to lift a drawn cumulance off zero. All language implementations
# use this same value to ensure cross-language determinism in distribution sampling.
.SMALLEST_POSITIVE_SUBNORMAL <- 5e-324

# The cumulance dist_additive and dist_multiplic consume, the two families whose quantile_at(0)
# is not a sample: additive returns negative infinity there, multiplic returns the zero its own
# support excludes. The other three families' quantile_at(0) is the finite value their support
# already includes, so they draw rng$uniform_float() directly and never call this.
#
# A drawn 0 is lifted to the smallest positive subnormal. rng$uniform_float() never returns 1, so
# the other endpoint needs no guard.
.sampling_cumulance <- function(rng) {
  u <- rng$uniform_float()
  if (u == 0.0) .SMALLEST_POSITIVE_SUBNORMAL else u
}

#' sqrt(2 * pi), the normalizing constant of the normal density.
#'
#' A literal rather than sqrt(2 * pi) computed at load time: the seven ports must divide by the
#' same double, and a runtime square root of a runtime product is one more place for them to
#' differ.
#'
#' @noRd
.SQRT_2PI <- 2.506628274631000241612355239340
