# Measures the typical variability when considering both samples together (AvgSpread)
#
# Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
#
# Assumptions:
#   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
#   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @return Measurement (when Sample input) or numeric (when vector input)
avg_spread <- function(x, y) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(avg_spread_estimator(x, y))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
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

# Internal Sample-based estimator
avg_spread_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  x <- x$with_subject("x")
  y <- y$with_subject("y")
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  n <- as.double(x$size)
  m <- as.double(y$size)

  spread_x <- fast_spread(x$values)
  if (spread_x <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }
  spread_y <- fast_spread(y$values)
  if (spread_y <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, y$subject))
  }

  Measurement$new((n * spread_x + m * spread_y) / (n + m), x$unit)
}
