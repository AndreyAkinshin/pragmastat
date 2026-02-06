# CenterBounds provides exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
# Requires weak symmetry assumption: distribution symmetric around unknown center.
#
# @param x Numeric vector of values
# @param misrate Misclassification rate (probability that true center falls outside bounds)
# @return List with 'lower' and 'upper' components
center_bounds <- function(x, misrate) {
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

  # Total number of pairwise averages (including self-pairs)
  total_pairs <- as.numeric(n) * as.numeric(n + 1) / 2

  # Get signed-rank margin
  margin <- signed_rank_margin(n, misrate)
  half_margin <- min(margin %/% 2, (total_pairs - 1) %/% 2)

  # k_left and k_right are 1-based ranks
  k_left <- half_margin + 1
  k_right <- total_pairs - half_margin

  # Sort the input
  sorted_x <- sort(x)

  result <- fast_center_quantile_bounds(sorted_x, k_left, k_right)
  return(list(lower = result$lower, upper = result$upper))
}
