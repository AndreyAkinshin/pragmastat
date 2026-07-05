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
  avg_spread_bounds_impl(x, y, misrate, seed)
}

# Single implementation on raw values. `sorted_x`/`sorted_y` (when non-NULL) are
# pre-sorted views for the order-independent sparity checks; the shuffles run on
# the original order via spread_bounds_inner_impl. Returns list(lower, upper).
avg_spread_bounds_impl <- function(x, y, misrate, seed, sorted_x = NULL, sorted_y = NULL) {
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

  spread_x_val <- if (!is.null(sorted_x)) spread_impl_compute(sorted_x, assume_sorted = TRUE) else spread_impl_compute(x)
  if (spread_x_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_y_val <- if (!is.null(sorted_y)) spread_impl_compute(sorted_y, assume_sorted = TRUE) else spread_impl_compute(y)
  if (spread_y_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$Y))
  }

  bounds_x <- spread_bounds_inner_impl(x, n, alpha, seed)
  bounds_y <- spread_bounds_inner_impl(y, m, alpha, seed)

  weight_x <- n / (n + m)
  weight_y <- m / (n + m)

  list(
    lower = weight_x * bounds_x$lower + weight_y * bounds_y$lower,
    upper = weight_x * bounds_x$upper + weight_y * bounds_y$upper
  )
}

# Internal Sample-based estimator: thin adapter over avg_spread_bounds_impl.
avg_spread_bounds_estimator <- function(x, y, misrate, seed = NULL) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  res <- avg_spread_bounds_impl(
    x$values, y$values, misrate, seed,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Bounds$new(res$lower, res$upper, x$unit)
}
