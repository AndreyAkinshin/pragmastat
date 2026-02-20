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
  check_validity(x, SUBJECTS$X)
  # Check validity for y (priority 0, subject y)
  check_validity(y, SUBJECTS$Y)

  n <- length(x)
  m <- length(y)

  spread_x <- fast_spread(x)
  if (spread_x <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_y <- fast_spread(y)
  if (spread_y <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$Y))
  }

  # Calculate shift (we know inputs are valid)
  shift_val <- fast_shift(x, y)[1]
  avg_spread_val <- (n * spread_x + m * spread_y) / (n + m)

  shift_val / avg_spread_val
}
