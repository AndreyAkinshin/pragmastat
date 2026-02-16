# AvgSpreadBounds provides distribution-free bounds for AvgSpread using
# Bonferroni combination of two SpreadBounds calls with equal split.
#
# @param x Numeric vector of values (at least 2 elements).
# @param y Numeric vector of values (at least 2 elements).
# @param misrate Misclassification rate (probability that true avg_spread falls outside bounds).
# @param seed Optional string seed for deterministic randomization.
# @return List with 'lower' and 'upper' components.
avg_spread_bounds <- function(x, y, misrate = DEFAULT_MISRATE, seed = NULL) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)
  m <- length(y)
  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }
  if (m < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$Y))
  }

  alpha <- misrate / 2
  min_x <- min_achievable_misrate_one_sample(n %/% 2)
  min_y <- min_achievable_misrate_one_sample(m %/% 2)
  if (alpha < min_x || alpha < min_y) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  check_sparity(x, SUBJECTS$X)
  check_sparity(y, SUBJECTS$Y)

  bounds_x <- spread_bounds(x, alpha, seed = seed)
  bounds_y <- spread_bounds(y, alpha, seed = seed)

  weight_x <- n / (n + m)
  weight_y <- m / (n + m)

  return(list(
    lower = weight_x * bounds_x$lower + weight_y * bounds_y$lower,
    upper = weight_x * bounds_x$upper + weight_y * bounds_y$upper
  ))
}
