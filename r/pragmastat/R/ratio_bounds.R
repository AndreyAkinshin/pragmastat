# RatioBounds provides bounds on the Ratio estimator with specified misclassification rate.
#
# Computes bounds via log-transformation and shift_bounds delegation:
# ratio_bounds(x, y, misrate) = exp(shift_bounds(log(x), log(y), misrate))
#
# Assumptions:
#   - positivity(x) - all values in x must be strictly positive
#   - positivity(y) - all values in y must be strictly positive
#
# @param x Numeric vector or Sample object (must be strictly positive)
# @param y Numeric vector or Sample object (must be strictly positive)
# @param misrate Misclassification rate (probability that true ratio falls outside bounds)
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
ratio_bounds <- function(x, y, misrate = DEFAULT_MISRATE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(ratio_bounds_estimator(x, y, misrate))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  min_misrate <- min_achievable_misrate_two_sample(length(x), length(y))
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  log_x <- log_transform(x, SUBJECTS$X)
  log_y <- log_transform(y, SUBJECTS$Y)

  log_bounds <- shift_bounds(log_x, log_y, misrate)

  return(list(lower = exp(log_bounds$lower), upper = exp(log_bounds$upper)))
}

# Internal Sample-based estimator
ratio_bounds_estimator <- function(x, y, misrate) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  x <- x$with_subject("x")
  y <- y$with_subject("y")
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  min_misrate <- min_achievable_misrate_two_sample(x$size, y$size)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  log_x <- x$log_transform()
  log_y <- y$log_transform()

  # Use vector-based shift_bounds since log-transformed values are in NumberUnit
  log_bounds <- shift_bounds(log_x$values, log_y$values, misrate)

  Bounds$new(exp(log_bounds$lower), exp(log_bounds$upper), ratio_unit)
}
