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
  # Native-array (raw) interface: plain numeric result.
  avg_spread_impl(x, y)
}

# Single implementation on raw values. spread is order-independent given sorted
# input. `sorted_x`/`sorted_y` (when non-NULL) are pre-sorted views; otherwise
# the impl routines sort internally. Both vector and Sample paths route through
# here.
avg_spread_impl <- function(x, y, sorted_x = NULL, sorted_y = NULL) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  n <- length(x)
  m <- length(y)

  xs <- if (!is.null(sorted_x)) sorted_x else x
  ys <- if (!is.null(sorted_y)) sorted_y else y
  # A pre-sorted view is already sorted; without one the raw vectors are unsorted.
  sorted_x_flag <- !is.null(sorted_x)
  sorted_y_flag <- !is.null(sorted_y)

  spread_x <- spread_impl_compute(xs, assume_sorted = sorted_x_flag)
  if (spread_x <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_y <- spread_impl_compute(ys, assume_sorted = sorted_y_flag)
  if (spread_y <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$Y))
  }

  (n * spread_x + m * spread_y) / (n + m)
}

# Internal Sample-based estimator: thin adapter over avg_spread_impl.
avg_spread_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  result <- avg_spread_impl(
    x$values, y$values,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Measurement$new(result, x$unit)
}
