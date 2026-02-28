# CenterBounds provides exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
# Requires weak symmetry assumption: distribution symmetric around unknown center.
#
# @param x Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true center falls outside bounds)
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
center_bounds <- function(x, misrate = DEFAULT_MISRATE) {
  if (inherits(x, "Sample")) {
    return(center_bounds_estimator(x, misrate))
  }
  # Legacy vector interface
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

  sorted_x <- sort(x)

  result <- fast_center_quantile_bounds(sorted_x, k_left, k_right)
  return(list(lower = result$lower, upper = result$upper))
}

# Internal Sample-based estimator
center_bounds_estimator <- function(x, misrate) {
  check_non_weighted("x", x)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- x$size

  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, x$subject))
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

  result <- fast_center_quantile_bounds(x$sorted_values, k_left, k_right)
  Bounds$new(result$lower, result$upper, x$unit)
}
