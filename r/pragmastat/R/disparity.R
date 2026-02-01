# Measures effect size: a normalized difference between x and y (Disparity)
#
# Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
#
# Assumptions:
#   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
#   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
#
# @param x Numeric vector of values
# @param y Numeric vector of values
# @return The disparity estimate
disparity <- function(x, y) {
  # Check validity for x (priority 0, subject x)
  check_validity(x, SUBJECTS$X, "Disparity")
  # Check validity for y (priority 0, subject y)
  check_validity(y, SUBJECTS$Y, "Disparity")
  # Check sparity for x (priority 2, subject x)
  check_sparity(x, SUBJECTS$X, "Disparity")
  # Check sparity for y (priority 2, subject y)
  check_sparity(y, SUBJECTS$Y, "Disparity")

  n <- length(x)
  m <- length(y)

  # Calculate shift (we know inputs are valid)
  shift_val <- fast_shift(x, y)[1]
  # Calculate avg_spread (using internal implementation since we already validated)
  spread_x <- fast_spread(x)
  spread_y <- fast_spread(y)
  avg_spread_val <- (n * spread_x + m * spread_y) / (n + m)

  shift_val / avg_spread_val
}
