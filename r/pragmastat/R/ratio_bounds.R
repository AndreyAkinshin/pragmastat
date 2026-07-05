# RatioBounds provides bounds on the Ratio estimator with specified misclassification rate.
#
# Computes bounds via log-transformation and shift_bounds delegation:
# ratio_bounds(x, y, misrate) = exp(shift_bounds(log(x), log(y), misrate))
#
# Assumptions:
#   - positivity(x) - all values in x must be strictly positive
#   - positivity(y) - all values in y must be strictly positive
#
# Public API: accepts either native numeric vectors (returns a plain unitless
# list(lower, upper)) or Samples (returns a Bounds object). ratio_bounds is
# order-independent, so the `assume_sorted` flag (vector path only) lets callers
# with already-ascending data skip the internal sort. Passing assume_sorted =
# TRUE on unsorted input is undefined behavior: the caller is responsible for the
# ordering and gets a wrong result on misuse.
#
# @param x Numeric vector or Sample object (must be strictly positive)
# @param y Numeric vector or Sample object (must be strictly positive)
# @param misrate Misclassification rate (probability that true ratio falls outside bounds)
# @param assume_sorted If TRUE, assume the vector inputs are already sorted
#   ascending and skip the internal sort (vector input only). Ignored for Sample
#   input, which always reuses its cached sorted views.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
ratio_bounds <- function(x, y, misrate = DEFAULT_MISRATE, assume_sorted = FALSE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(ratio_bounds_estimator(x, y, misrate))
  }
  ratio_bounds_impl(x, y, misrate, assume_sorted = assume_sorted)
}

# Single implementation on raw values. Returns list(lower, upper). The misrate
# domain check runs before log_transform to keep domain priority over positivity.
# log is monotonic, so sorted positive input yields sorted log output; therefore
# `assume_sorted` (or a pre-sorted view) propagates straight to the inner
# shift_bounds over the log-transformed values.
ratio_bounds_impl <- function(x, y, misrate, assume_sorted = FALSE,
                              sorted_x = NULL, sorted_y = NULL) {
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

  # Reuse a pre-sorted view when available; else honor assume_sorted on the input.
  if (!is.null(sorted_x)) {
    log_sorted_x <- log(sorted_x)
    log_sorted_y <- log(sorted_y)
  } else if (assume_sorted) {
    log_sorted_x <- log_x
    log_sorted_y <- log_y
  } else {
    log_sorted_x <- NULL
    log_sorted_y <- NULL
  }

  log_bounds <- shift_bounds_impl(
    log_x, log_y, misrate,
    sorted_x = log_sorted_x, sorted_y = log_sorted_y
  )

  list(lower = exp(log_bounds$lower), upper = exp(log_bounds$upper))
}

# Internal Sample-based estimator: thin adapter over ratio_bounds_impl.
ratio_bounds_estimator <- function(x, y, misrate) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  res <- ratio_bounds_impl(
    x$values, y$values, misrate,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Bounds$new(res$lower, res$upper, ratio_unit)
}
