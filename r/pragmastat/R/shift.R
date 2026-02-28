# Measures the typical difference between elements of x and y (Shift).
# Calculates the median of all pairwise differences (x[i] - y[j]).
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @return Measurement (when Sample input) or numeric (when vector input)
shift <- function(x, y) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(shift_estimator(x, y))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)
  fast_shift(x, y, p = 0.5, assume_sorted = FALSE)
}

# Internal Sample-based estimator
shift_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  x <- x$with_subject("x")
  y <- y$with_subject("y")
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b
  result <- fast_shift(x$values, y$values, p = 0.5, assume_sorted = FALSE)
  Measurement$new(result, x$unit)
}
