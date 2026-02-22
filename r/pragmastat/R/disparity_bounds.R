# DisparityBounds provides distribution-free bounds for the Disparity estimator
# (Shift / AvgSpread) using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
#
# @param x Numeric vector of values (at least 2 elements).
# @param y Numeric vector of values (at least 2 elements).
# @param misrate Misclassification rate.
# @param seed Optional string seed for deterministic randomization.
# @return List with 'lower' and 'upper' components.
disparity_bounds <- function(x, y, misrate = DEFAULT_MISRATE, seed = NULL) {
  # Check validity (priority 0)
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

  min_shift <- min_achievable_misrate_two_sample(n, m)
  min_x <- min_achievable_misrate_one_sample(n %/% 2)
  min_y <- min_achievable_misrate_one_sample(m %/% 2)
  min_avg <- 2.0 * max(min_x, min_y)

  if (misrate < min_shift + min_avg) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  extra <- misrate - (min_shift + min_avg)
  alpha_shift <- min_shift + extra / 2.0
  alpha_avg <- min_avg + extra / 2.0

  if (fast_spread(x) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  if (fast_spread(y) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$Y))
  }

  sb <- shift_bounds(x, y, alpha_shift)
  ab <- avg_spread_bounds(x, y, alpha_avg, seed = seed)

  la <- ab$lower
  ua <- ab$upper
  ls <- sb$lower
  us <- sb$upper

  if (la > 0.0) {
    r1 <- ls / la
    r2 <- ls / ua
    r3 <- us / la
    r4 <- us / ua
    lower <- min(r1, r2, r3, r4)
    upper <- max(r1, r2, r3, r4)
    return(list(lower = lower, upper = upper))
  }

  if (ua <= 0.0) {
    if (ls == 0.0 && us == 0.0) {
      return(list(lower = 0.0, upper = 0.0))
    }
    if (ls >= 0.0) {
      return(list(lower = 0.0, upper = Inf))
    }
    if (us <= 0.0) {
      return(list(lower = -Inf, upper = 0.0))
    }
    return(list(lower = -Inf, upper = Inf))
  }

  # Default: ua > 0 && la <= 0
  if (ls > 0.0) {
    return(list(lower = ls / ua, upper = Inf))
  }
  if (us < 0.0) {
    return(list(lower = -Inf, upper = us / ua))
  }
  if (ls == 0.0 && us == 0.0) {
    return(list(lower = 0.0, upper = 0.0))
  }
  if (ls == 0.0 && us > 0.0) {
    return(list(lower = 0.0, upper = Inf))
  }
  if (ls < 0.0 && us == 0.0) {
    return(list(lower = -Inf, upper = 0.0))
  }

  return(list(lower = -Inf, upper = Inf))
}
