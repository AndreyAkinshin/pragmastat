# Measures the typical variability when considering both samples together (AvgSpread)
#
# Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
#
# Assumptions:
#   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
#   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
#
# @param x Numeric vector of values
# @param y Numeric vector of values
# @return The average spread estimate
avg_spread <- function(x, y) {
  # Check validity for x (priority 0, subject x)
  check_validity(x, SUBJECTS$X, "AvgSpread")
  # Check validity for y (priority 0, subject y)
  check_validity(y, SUBJECTS$Y, "AvgSpread")
  # Check sparity for x (priority 2, subject x)
  check_sparity(x, SUBJECTS$X, "AvgSpread")
  # Check sparity for y (priority 2, subject y)
  check_sparity(y, SUBJECTS$Y, "AvgSpread")

  n <- length(x)
  m <- length(y)
  # Calculate spreads (using internal implementation since we already validated)
  spread_x <- fast_spread(x)
  spread_y <- fast_spread(y)
  (n * spread_x + m * spread_y) / (n + m)
}
