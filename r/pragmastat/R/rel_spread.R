# Measures the relative dispersion of a sample (RelSpread)
#
# @description \code{rel_spread} is deprecated.
# Use \code{spread(x) / abs(center(x))} instead.
#
# Calculates the ratio of Spread to absolute Center.
# Robust alternative to the coefficient of variation.
#
# Assumptions:
#   - positivity(x) - all values must be strictly positive (ensures Center > 0)
#
# @param x Numeric vector of values
# @return The relative spread estimate
rel_spread <- function(x) {
  .Deprecated("spread(x) / abs(center(x))")
  # Check validity (priority 0)
  check_validity(x, SUBJECTS$X)
  # Check positivity (priority 1)
  check_positivity(x, SUBJECTS$X)

  center_val <- fast_center(x)
  # Calculate spread (using internal implementation since we already validated)
  spread_val <- fast_spread(x)
  # center is guaranteed positive because all values are positive
  spread_val / abs(center_val)
}
