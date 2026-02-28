# AvgSpreadBounds provides distribution-free bounds for AvgSpread using
# Bonferroni combination of two SpreadBounds calls with equal split.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true avg_spread falls outside bounds).
# @param seed Optional string seed for deterministic randomization.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
avg_spread_bounds <- function(x, y, misrate = DEFAULT_MISRATE, seed = NULL) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(avg_spread_bounds_estimator(x, y, misrate, seed = seed))
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

  alpha <- misrate / 2
  min_x <- min_achievable_misrate_one_sample(n %/% 2)
  min_y <- min_achievable_misrate_one_sample(m %/% 2)
  if (alpha < min_x || alpha < min_y) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  if (fast_spread(x) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  if (fast_spread(y) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$Y))
  }

  bounds_x <- spread_bounds(x, alpha, seed = seed)
  bounds_y <- spread_bounds(y, alpha, seed = seed)

  weight_x <- n / (n + m)
  weight_y <- m / (n + m)

  return(list(
    lower = weight_x * bounds_x$lower + weight_y * bounds_y$lower,
    upper = weight_x * bounds_x$upper + weight_y * bounds_y$upper
  ))
}

# Internal Sample-based estimator
avg_spread_bounds_estimator <- function(x, y, misrate, seed = NULL) {
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

  alpha <- misrate / 2
  min_x <- min_achievable_misrate_one_sample(n %/% 2)
  min_y <- min_achievable_misrate_one_sample(m %/% 2)
  if (alpha < min_x || alpha < min_y) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  if (fast_spread(x$values) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }
  if (fast_spread(y$values) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, y$subject))
  }

  # Use vector-based spread_bounds for internal computation
  bounds_x <- spread_bounds(x$values, alpha, seed = seed)
  bounds_y <- spread_bounds(y$values, alpha, seed = seed)

  weight_x <- n / (n + m)
  weight_y <- m / (n + m)

  Bounds$new(
    weight_x * bounds_x$lower + weight_y * bounds_y$lower,
    weight_x * bounds_x$upper + weight_y * bounds_y$upper,
    x$unit
  )
}
