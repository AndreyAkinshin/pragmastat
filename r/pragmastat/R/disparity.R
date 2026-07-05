# Measures effect size: a normalized difference between x and y (Disparity)
#
# Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
#
# Assumptions:
#   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
#   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
#
# Public API: accepts either native numeric vectors (returns a plain unitless
# numeric) or Samples (returns a Measurement). The `assume_sorted` flag (vector
# path only) lets callers with already-ascending data skip the internal sort.
# Passing assume_sorted = TRUE on unsorted input is undefined behavior: the
# caller is responsible for the ordering and gets a wrong result on misuse.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param assume_sorted If TRUE, assume the vector inputs are already sorted
#   ascending and skip the internal sort (vector input only). Ignored for Sample
#   input, which always reuses its cached sorted views.
# @return Measurement (when Sample input) or numeric (when vector input)
disparity <- function(x, y, assume_sorted = FALSE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(disparity_estimator(x, y))
  }
  # Native-array (raw) interface: unitless numeric result.
  disparity_impl(x, y, assume_sorted)
}

# Single implementation on raw values. spread, shift and avg_spread are all
# order-independent given sorted input. `sorted_x`/`sorted_y` (when non-NULL) are
# pre-sorted views; otherwise `assume_sorted` controls whether the impl routines
# sort internally. Both vector and Sample paths route through here.
disparity_impl <- function(x, y, assume_sorted, sorted_x = NULL, sorted_y = NULL) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  n <- length(x)
  m <- length(y)

  xs <- if (!is.null(sorted_x)) sorted_x else x
  ys <- if (!is.null(sorted_y)) sorted_y else y
  # When a pre-sorted view is supplied it is already sorted; otherwise honor flag.
  sorted_x_flag <- if (!is.null(sorted_x)) TRUE else assume_sorted
  sorted_y_flag <- if (!is.null(sorted_y)) TRUE else assume_sorted

  spread_x <- spread_impl_compute(xs, assume_sorted = sorted_x_flag)
  if (spread_x <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_y <- spread_impl_compute(ys, assume_sorted = sorted_y_flag)
  if (spread_y <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$Y))
  }

  shift_val <- shift_impl_compute(xs, ys, p = 0.5, assume_sorted = (sorted_x_flag && sorted_y_flag))
  avg_spread_val <- (n * spread_x + m * spread_y) / (n + m)

  shift_val / avg_spread_val
}

# Internal Sample-based estimator: thin adapter over disparity_impl.
disparity_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  result <- disparity_impl(
    x$values, y$values,
    assume_sorted = TRUE,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Measurement$new(result, disparity_unit)
}
