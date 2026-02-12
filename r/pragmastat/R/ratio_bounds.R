# RatioBounds provides bounds on the Ratio estimator with specified misclassification rate.
#
# Computes bounds via log-transformation and shift_bounds delegation:
# ratio_bounds(x, y, misrate) = exp(shift_bounds(log(x), log(y), misrate))
#
# Assumptions:
#   - positivity(x) - all values in x must be strictly positive
#   - positivity(y) - all values in y must be strictly positive
#
# @param x Numeric vector of values (must be strictly positive)
# @param y Numeric vector of values (must be strictly positive)
# @param misrate Misclassification rate (probability that true ratio falls outside bounds)
# @return List with 'lower' and 'upper' components
ratio_bounds <- function(x, y, misrate = DEFAULT_MISRATE) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  min_misrate <- min_achievable_misrate_two_sample(length(x), length(y))
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  # Log-transform samples (includes positivity check)
  log_x <- log_transform(x, SUBJECTS$X)
  log_y <- log_transform(y, SUBJECTS$Y)

  # Delegate to shift_bounds in log-space
  log_bounds <- shift_bounds(log_x, log_y, misrate)

  # Exp-transform back to ratio-space
  return(list(lower = exp(log_bounds$lower), upper = exp(log_bounds$upper)))
}
