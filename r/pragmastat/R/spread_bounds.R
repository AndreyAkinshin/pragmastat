# SpreadBounds provides distribution-free bounds for Spread using disjoint pairs
# with sign-test inversion. Randomizes the cutoff between adjacent ranks to match
# the requested misrate.
#
# @param x Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true spread falls outside bounds)
# @param seed Optional string seed for deterministic randomization
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
spread_bounds <- function(x, misrate = DEFAULT_MISRATE, seed = NULL) {
  if (inherits(x, "Sample")) {
    return(spread_bounds_estimator(x, misrate, seed = seed))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)
  m <- n %/% 2

  min_misrate <- min_achievable_misrate_one_sample(m)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  if (fast_spread(x) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }

  if (!is.null(seed)) {
    rng_obj <- Rng$new(seed)
  } else {
    rng_obj <- Rng$new()
  }

  margin <- sign_margin_randomized(m, misrate, rng_obj)
  half_margin <- margin %/% 2
  max_half_margin <- (m - 1) %/% 2
  if (half_margin > max_half_margin) half_margin <- max_half_margin

  k_left <- half_margin + 1
  k_right <- m - half_margin

  indices <- seq(0, n - 1)
  shuffled <- rng_obj$shuffle(indices)

  diffs <- numeric(m)
  for (i in seq_len(m)) {
    a <- shuffled[i * 2 - 1]
    b <- shuffled[i * 2]
    diffs[i] <- abs(x[a + 1] - x[b + 1])
  }
  diffs <- sort(diffs)

  return(list(lower = diffs[k_left], upper = diffs[k_right]))
}

# Internal Sample-based estimator
spread_bounds_estimator <- function(x, misrate, seed = NULL) {
  check_non_weighted("x", x)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- x$size
  m <- n %/% 2

  min_misrate <- min_achievable_misrate_one_sample(m)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }
  if (fast_spread(x$values) <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }

  if (!is.null(seed)) {
    rng_obj <- Rng$new(seed)
  } else {
    rng_obj <- Rng$new()
  }

  margin <- sign_margin_randomized(m, misrate, rng_obj)
  half_margin <- margin %/% 2
  max_half_margin <- (m - 1) %/% 2
  if (half_margin > max_half_margin) half_margin <- max_half_margin

  k_left <- half_margin + 1
  k_right <- m - half_margin

  indices <- seq(0, n - 1)
  shuffled <- rng_obj$shuffle(indices)

  vals <- x$values
  diffs <- numeric(m)
  for (i in seq_len(m)) {
    a <- shuffled[i * 2 - 1]
    b <- shuffled[i * 2]
    diffs[i] <- abs(vals[a + 1] - vals[b + 1])
  }
  diffs <- sort(diffs)

  Bounds$new(diffs[k_left], diffs[k_right], x$unit)
}
