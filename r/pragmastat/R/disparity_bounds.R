# DisparityBounds provides distribution-free bounds for the Disparity estimator
# (Shift / AvgSpread) using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param misrate Misclassification rate.
# @param seed Optional string seed for deterministic randomization.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
disparity_bounds <- function(x, y, misrate = DEFAULT_MISRATE, seed = NULL) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(disparity_bounds_estimator(x, y, misrate, seed = seed))
  }
  # Legacy vector interface
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

  disparity_bounds_from_components(sb$lower, sb$upper, ab$lower, ab$upper)
}

# Internal Sample-based estimator
disparity_bounds_estimator <- function(x, y, misrate, seed = NULL) {
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

  n <- x$size
  m <- y$size
  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, x$subject))
  }
  if (m < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, y$subject))
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

  if (fast_spread(x$values) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }
  if (fast_spread(y$values) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, y$subject))
  }

  # Use vector-based bounds for internal computation
  sb <- shift_bounds(x$values, y$values, alpha_shift)
  ab <- avg_spread_bounds(x$values, y$values, alpha_avg, seed = seed)

  result <- disparity_bounds_from_components(sb$lower, sb$upper, ab$lower, ab$upper)
  Bounds$new(result$lower, result$upper, disparity_unit)
}

# Common logic for computing disparity bounds from shift and avg_spread component bounds.
disparity_bounds_from_components <- function(ls, us, la, ua) {
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
