# SpreadBounds provides distribution-free bounds for Spread using disjoint pairs
# with sign-test inversion. Randomizes the cutoff between adjacent ranks to match
# the requested misrate.
#
# @param x Numeric vector of values
# @param misrate Misclassification rate (probability that true spread falls outside bounds)
# @param seed Optional string seed for deterministic randomization
# @return List with 'lower' and 'upper' components
spread_bounds <- function(x, misrate = DEFAULT_MISRATE, seed = NULL) {
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

  check_sparity(x, SUBJECTS$X)

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

  # Create 0-based indices, shuffle
  indices <- seq(0, n - 1)
  shuffled <- rng_obj$shuffle(indices)

  # Compute absolute differences of disjoint pairs
  diffs <- numeric(m)
  for (i in seq_len(m)) {
    a <- shuffled[i * 2 - 1]
    b <- shuffled[i * 2]
    diffs[i] <- abs(x[a + 1] - x[b + 1])
  }
  diffs <- sort(diffs)

  return(list(lower = diffs[k_left], upper = diffs[k_right]))
}
