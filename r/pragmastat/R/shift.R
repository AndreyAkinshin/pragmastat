# Measures the typical difference between elements of x and y (Shift).
# Calculates the median of all pairwise differences (x[i] - y[j]).
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
shift <- function(x, y, assume_sorted = FALSE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(shift_estimator(x, y))
  }
  # Native-array (raw) interface: unitless numeric result.
  shift_impl(x, y, assume_sorted)
}

# Single implementation on raw values. `sorted_x`/`sorted_y` (when non-NULL) are
# pre-sorted views used directly for the order-independent quantile selection.
# Both the vector path and the Sample path route through here.
shift_impl <- function(x, y, assume_sorted, sorted_x = NULL, sorted_y = NULL) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)
  xs <- if (!is.null(sorted_x)) sorted_x else x
  ys <- if (!is.null(sorted_y)) sorted_y else y
  shift_impl_compute(xs, ys, p = 0.5, assume_sorted = assume_sorted)
}

# Internal Sample-based estimator: thin adapter over shift_impl.
shift_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b
  result <- shift_impl(
    x$values, y$values,
    assume_sorted = TRUE,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Measurement$new(result, x$unit)
}
