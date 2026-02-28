# Measures how many times larger x is compared to y (Ratio)
#
# Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
# Equivalent to: exp(Shift(log(x), log(y)))
#
# Assumptions:
#   - positivity(x) - all values in x must be strictly positive
#   - positivity(y) - all values in y must be strictly positive
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @return Measurement (when Sample input) or numeric (when vector input)
ratio <- function(x, y) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(ratio_estimator(x, y))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)
  log_x <- log_transform(x, SUBJECTS$X)
  log_y <- log_transform(y, SUBJECTS$Y)
  log_result <- fast_shift(log_x, log_y, p = 0.5, assume_sorted = FALSE)
  exp(log_result)
}

# Internal Sample-based estimator
ratio_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  x <- x$with_subject("x")
  y <- y$with_subject("y")
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  if (any(x$values <= 0)) {
    stop(assumption_error(ASSUMPTION_IDS$POSITIVITY, x$subject))
  }
  if (any(y$values <= 0)) {
    stop(assumption_error(ASSUMPTION_IDS$POSITIVITY, y$subject))
  }

  log_x <- log(x$values)
  log_y <- log(y$values)
  log_result <- fast_shift(log_x, log_y, p = 0.5, assume_sorted = FALSE)
  Measurement$new(exp(log_result), ratio_unit)
}
