# ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
# The misrate represents the probability that the true shift falls outside the computed bounds.
#
# Public API: accepts either native numeric vectors (returns a plain unitless
# list(lower, upper)) or Samples (returns a Bounds object). shift_bounds is
# order-independent, so the `assume_sorted` flag (vector path only) lets callers
# with already-ascending data skip the internal sort. Passing assume_sorted =
# TRUE on unsorted input is undefined behavior: the caller is responsible for the
# ordering and gets a wrong result on misuse.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true shift falls outside bounds)
# @param assume_sorted If TRUE, assume the vector inputs are already sorted
#   ascending and skip the internal sort (vector input only). Ignored for Sample
#   input, which always reuses its cached sorted views.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
shift_bounds <- function(x, y, misrate = DEFAULT_MISRATE, assume_sorted = FALSE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(shift_bounds_estimator(x, y, misrate))
  }
  # When the caller guarantees sorted input, it doubles as the sorted view.
  sorted_x <- if (assume_sorted) x else NULL
  sorted_y <- if (assume_sorted) y else NULL
  shift_bounds_impl(x, y, misrate, sorted_x = sorted_x, sorted_y = sorted_y)
}

# Single implementation on raw values. `sorted_x`/`sorted_y` (when non-NULL) are
# pre-sorted views used directly for the order-independent quantile selection.
# Returns list(lower, upper).
shift_bounds_impl <- function(x, y, misrate, sorted_x = NULL, sorted_y = NULL) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)
  m <- length(y)

  min_misrate <- min_achievable_misrate_two_sample(n, m)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  xs <- if (!is.null(sorted_x)) sorted_x else sort(x)
  ys <- if (!is.null(sorted_y)) sorted_y else sort(y)

  total <- as.numeric(n) * as.numeric(m)

  if (total == 1) {
    value <- xs[1] - ys[1]
    return(list(lower = value, upper = value))
  }

  margin <- pairwise_margin(n, m, misrate)
  half_margin <- min(floor(margin / 2), floor((total - 1) / 2))
  k_left <- half_margin
  k_right <- (total - 1) - half_margin

  # total >= 2 here (the total == 1 case returned above), so denominator >= 1.
  denominator <- total - 1
  p_left <- k_left / denominator
  p_right <- k_right / denominator

  quantiles <- shift_impl_compute(xs, ys, c(p_left, p_right), assume_sorted = TRUE)
  lower <- min(quantiles[1], quantiles[2])
  upper <- max(quantiles[1], quantiles[2])

  list(lower = lower, upper = upper)
}

# Internal Sample-based estimator: thin adapter over shift_bounds_impl.
shift_bounds_estimator <- function(x, y, misrate) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  res <- shift_bounds_impl(
    x$values, y$values, misrate,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Bounds$new(res$lower, res$upper, x$unit)
}
