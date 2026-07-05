# DisparityBounds provides distribution-free bounds for the Disparity estimator
# (Shift / AvgSpread) using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
#
# Public API: accepts either native numeric vectors (returns a plain unitless
# list(lower, upper)) or Samples (returns a Bounds object). When the input is
# GENUINELY SORTED, `assume_sorted` does not change the result: it only lets the
# order-independent shift and sparity sub-computations skip the internal sort,
# while the disjoint-pair shuffle in the avg-spread component runs on the same
# (sorted) order either way.
#
# On UNSORTED input, `assume_sorted = TRUE` is undefined behavior and CAN change
# the result: the embedded order-independent shift-bounds (and sparity) consume
# the unsorted slice as if it were a sorted view, producing a different value
# than the assume_sorted = FALSE path (which sorts first). The caller is
# responsible for honoring the sorted-input contract.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param misrate Misclassification rate.
# @param seed Optional string seed for deterministic randomization.
# @param assume_sorted If TRUE, skip the internal sort of the order-independent
#   shift and sparity sub-computations (vector input only). On genuinely sorted
#   input the result is unchanged; on UNSORTED input this is undefined behavior
#   and CAN change the result (the shift/sparity sub-computations read the
#   unsorted slice as a sorted view). Ignored for Sample input, which reuses its
#   sorted views.
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
disparity_bounds <- function(x, y, misrate = DEFAULT_MISRATE, seed = NULL, assume_sorted = FALSE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(disparity_bounds_estimator(x, y, misrate, seed = seed))
  }
  # The shuffle runs on the original order; when the caller guarantees sorted
  # input, it doubles as the sorted view for the shift/sparity sub-computations.
  sorted_x <- if (assume_sorted) x else NULL
  sorted_y <- if (assume_sorted) y else NULL
  disparity_bounds_impl(x, y, misrate, seed, sorted_x = sorted_x, sorted_y = sorted_y)
}

# Single implementation on raw values. `sorted_x`/`sorted_y` (when non-NULL) are
# pre-sorted views for the order-independent shift and sparity sub-computations.
# Returns list(lower, upper).
disparity_bounds_impl <- function(x, y, misrate, seed, sorted_x = NULL, sorted_y = NULL) {
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

  # The spread > 0 sparity check is performed by avg_spread_bounds_impl below
  # (identical predicate, X/Y order). shift_bounds_impl runs first but cannot
  # stop() for these inputs, so it cannot mask that sparity error.
  sb <- shift_bounds_impl(x, y, alpha_shift, sorted_x = sorted_x, sorted_y = sorted_y)
  ab <- avg_spread_bounds_impl(x, y, alpha_avg, seed, sorted_x = sorted_x, sorted_y = sorted_y)

  disparity_bounds_from_components(sb$lower, sb$upper, ab$lower, ab$upper)
}

# Internal Sample-based estimator: thin adapter over disparity_bounds_impl.
disparity_bounds_estimator <- function(x, y, misrate, seed = NULL) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  res <- disparity_bounds_impl(
    x$values, y$values, misrate, seed,
    sorted_x = x$sorted_values, sorted_y = y$sorted_values
  )
  Bounds$new(res$lower, res$upper, disparity_unit)
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
