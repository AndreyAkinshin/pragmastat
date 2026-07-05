# SpreadBounds provides distribution-free bounds for Spread using disjoint pairs
# with sign-test inversion. Randomizes the cutoff between adjacent ranks to match
# the requested misrate.
#
# Public API: accepts either a native numeric vector (returns a plain unitless
# list(lower, upper)) or a Sample (returns a Bounds object). The disjoint-pair
# shuffle ALWAYS runs on the passed array's order, so `assume_sorted` NEVER
# changes the result; it only skips the internal sort of the order-independent
# sparity (spread > 0) check. Passing assume_sorted = TRUE on unsorted input is
# undefined behavior for that sparity check: the caller is responsible.
#
# @param x Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true spread falls outside bounds)
# @param seed Optional string seed for deterministic randomization
# @param assume_sorted If TRUE, skip the internal sort of the sparity check
#   (vector input only); the shuffle still runs on the original order, so the
#   result is unchanged. Ignored for Sample input, which reuses its sorted view.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
spread_bounds <- function(x, misrate = DEFAULT_MISRATE, seed = NULL, assume_sorted = FALSE) {
  if (inherits(x, "Sample")) {
    return(spread_bounds_estimator(x, misrate, seed = seed))
  }
  # The shuffle runs on the original order `x`; when the caller guarantees
  # sorted input, it doubles as the sparity-only sorted view.
  sorted <- if (assume_sorted) x else NULL
  spread_bounds_impl(x, misrate, seed, sorted = sorted)
}

# Single implementation on raw values. `sorted` (when non-NULL) is a pre-sorted
# view for the order-independent sparity check; the disjoint-pair shuffle in
# spread_bounds_inner_impl always runs on the original order. Returns list.
spread_bounds_impl <- function(values, misrate, seed, sorted = NULL) {
  check_validity(values, SUBJECTS$X)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(values)
  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  m <- n %/% 2

  min_misrate <- min_achievable_misrate_one_sample(m)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }
  spread_val <- if (!is.null(sorted)) spread_impl_compute(sorted, assume_sorted = TRUE) else spread_impl_compute(values)
  if (spread_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }

  spread_bounds_inner_impl(values, n, misrate, seed)
}

# Shuffle + disjoint-pair order statistics. Caller is responsible for validity
# and sparity checks. Operates on the original-order `values`. Returns list.
spread_bounds_inner_impl <- function(values, n, misrate, seed) {
  m <- n %/% 2

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
    diffs[i] <- abs(values[a + 1] - values[b + 1])
  }
  diffs <- sort(diffs)

  list(lower = diffs[k_left], upper = diffs[k_right])
}

# Internal Sample-based estimator: thin adapter over spread_bounds_impl.
spread_bounds_estimator <- function(x, misrate, seed = NULL) {
  check_non_weighted("x", x)
  res <- spread_bounds_impl(
    x$values, misrate, seed,
    sorted = x$sorted_values
  )
  Bounds$new(res$lower, res$upper, x$unit)
}
