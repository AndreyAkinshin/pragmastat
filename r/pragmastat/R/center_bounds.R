# CenterBounds provides exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
# Requires weak symmetry assumption: distribution symmetric around unknown center.
#
# Public API: accepts either a native numeric vector (returns a plain unitless
# list(lower, upper)) or a Sample (returns a Bounds object). center_bounds is
# order-independent, so the `assume_sorted` flag (vector path only) lets callers
# with already-ascending data skip the internal sort. Passing assume_sorted =
# TRUE on unsorted input is undefined behavior: the caller is responsible for the
# ordering and gets a wrong result on misuse.
#
# @param x Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true center falls outside bounds)
# @param assume_sorted If TRUE, assume the vector input is already sorted
#   ascending and skip the internal sort (vector input only). Ignored for Sample
#   input, which always reuses its cached sorted view.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
center_bounds <- function(x, misrate = DEFAULT_MISRATE, assume_sorted = FALSE) {
  if (inherits(x, "Sample")) {
    return(center_bounds_estimator(x, misrate))
  }
  # When the caller guarantees sorted input, it doubles as the sorted view.
  sorted <- if (assume_sorted) x else NULL
  center_bounds_impl(x, misrate, sorted = sorted)
}

# Single implementation on raw values. `sorted` (when non-NULL) is a pre-sorted
# view. Returns list(lower, upper).
center_bounds_impl <- function(x, misrate, sorted = NULL) {
  check_validity(x, SUBJECTS$X)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)

  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }

  min_misrate <- min_achievable_misrate_one_sample(n)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  total_pairs <- as.numeric(n) * as.numeric(n + 1) / 2

  margin <- signed_rank_margin(n, misrate)
  half_margin <- min(margin %/% 2, (total_pairs - 1) %/% 2)

  k_left <- half_margin + 1
  k_right <- total_pairs - half_margin

  sorted_x <- if (!is.null(sorted)) sorted else sort(x)

  result <- center_quantile_bounds_impl(sorted_x, k_left, k_right)
  list(lower = result$lower, upper = result$upper)
}

# Internal Sample-based estimator: thin adapter over center_bounds_impl.
center_bounds_estimator <- function(x, misrate) {
  check_non_weighted("x", x)
  res <- center_bounds_impl(
    x$values, misrate,
    sorted = x$sorted_values
  )
  Bounds$new(res$lower, res$upper, x$unit)
}
