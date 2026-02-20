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
  (n * spread_x + m * spread_y) / (n + m)
}
