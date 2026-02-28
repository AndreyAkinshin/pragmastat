# Measures effect size: a normalized difference between x and y (Disparity)
#
# Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
#
# Assumptions:
#   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
#   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @return Measurement (when Sample input) or numeric (when vector input)
disparity <- function(x, y) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(disparity_estimator(x, y))
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

  shift_val <- fast_shift(x, y)[1]
  avg_spread_val <- (n * spread_x + m * spread_y) / (n + m)

  shift_val / avg_spread_val
}

# Internal Sample-based estimator
disparity_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  x <- x$with_subject("x")
  y <- y$with_subject("y")
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  n <- x$size
  m <- y$size

  spread_x <- fast_spread(x$values)
  if (spread_x <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }
  spread_y <- fast_spread(y$values)
  if (spread_y <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, y$subject))
  }

  shift_val <- fast_shift(x$values, y$values)[1]
  avg_spread_val <- (n * spread_x + m * spread_y) / (n + m)

  Measurement$new(shift_val / avg_spread_val, disparity_unit)
}
