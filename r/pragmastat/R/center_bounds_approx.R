# CenterBoundsApprox provides bootstrap-based nominal bounds for Center (Hodges-Lehmann pseudomedian).
# No symmetry requirement, but provides only nominal (not exact) coverage.
#
# WARNING: Bootstrap percentile method has known undercoverage for small samples.
# When requesting 95% confidence (misrate = 0.05), actual coverage is typically 85-92% for n < 30.
# Users requiring exact coverage should use center_bounds (if symmetry holds) or median_bounds.
#
# @param x Numeric vector of values
# @param misrate Misclassification rate (probability that true center falls outside bounds)
# @param seed Optional seed string for deterministic results
# @return List with 'lower' and 'upper' components
center_bounds_approx <- function(x, misrate, seed = NULL) {
  center_bounds_approx_internal(x, misrate, seed, .CENTER_BOUNDS_APPROX_ITERATIONS)
}

.CENTER_BOUNDS_APPROX_ITERATIONS <- 10000L
.CENTER_BOUNDS_APPROX_MAX_SUBSAMPLE <- 5000L
.CENTER_BOUNDS_APPROX_DEFAULT_SEED <- "center-bounds-approx"

center_bounds_approx_internal <- function(x, misrate, seed, iterations) {
  check_validity(x, SUBJECTS$X)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)
  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }

  min_misrate <- max(2.0 / iterations, min_achievable_misrate_one_sample(n))
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  # Sort for permutation invariance
  sorted_x <- sort(x)

  # Subsample cap for performance
  m <- min(n, .CENTER_BOUNDS_APPROX_MAX_SUBSAMPLE)

  # Resolve seed
  effective_seed <- if (is.null(seed)) .CENTER_BOUNDS_APPROX_DEFAULT_SEED else seed

  # Bootstrap loop in C (xoshiro256++ RNG + fast_center per iteration)
  centers <- .Call("center_bounds_approx_bootstrap_c",
                   as.double(sorted_x), as.integer(m), as.integer(iterations),
                   effective_seed, PACKAGE = "pragmastat")

  # Extract percentile bounds
  alpha <- misrate / 2.0
  lo_idx <- floor(alpha * iterations) + 1  # +1 for 1-based indexing
  hi_idx <- ceiling((1.0 - alpha) * iterations)  # already 1-based after ceiling
  lo_idx <- min(max(1, lo_idx), hi_idx)

  bootstrap_lo <- centers[lo_idx]
  bootstrap_hi <- centers[min(iterations, hi_idx)]

  # Scale bounds to full n using asymptotic sqrt(n) rate
  if (m < n) {
    center_val <- fast_center(sorted_x)
    scale_factor <- sqrt(m / n)
    lo <- center_val + (bootstrap_lo - center_val) / scale_factor
    hi <- center_val + (bootstrap_hi - center_val) / scale_factor
    return(list(lower = lo, upper = hi))
  }

  return(list(lower = bootstrap_lo, upper = bootstrap_hi))
}
