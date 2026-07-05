# Estimates data dispersion (Spread)
#
# Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
# More robust than standard deviation and more efficient than MAD.
# Uses O(n log n) algorithm.
#
# Assumptions:
#   - sparity(x) - sample must be non tie-dominant (Spread > 0)
#
# Public API: accepts either a native numeric vector (returns a plain unitless
# numeric) or a Sample (returns a Measurement). The `assume_sorted` flag (vector
# path only) lets callers with already-ascending data skip the internal sort.
# Passing assume_sorted = TRUE on unsorted input is a contract violation
# (undefined behavior): the result is unspecified, but termination is
# guaranteed. The selection loop is bounded and fails with a deterministic
# convergence error on pathological input.
#
# @param x Numeric vector or Sample object
# @param assume_sorted If TRUE, assume the vector input is already sorted
#   ascending and skip the internal sort (vector input only). Ignored for Sample
#   input, which always reuses its cached sorted view.
# @return Measurement (when Sample input) or numeric (when vector input)
spread <- function(x, assume_sorted = FALSE) {
  if (inherits(x, "Sample")) {
    return(spread_estimator(x))
  }
  # Native-array (raw) interface: unitless numeric result.
  spread_impl(x, assume_sorted)
}

# Single implementation on raw values. Both the vector path and the Sample path
# (via spread_estimator, which passes the cached sorted view) route through here.
spread_impl <- function(values, assume_sorted) {
  check_validity(values, SUBJECTS$X)
  spread_val <- spread_impl_compute(values, assume_sorted = assume_sorted)
  if (spread_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_val
}

# Internal Sample-based estimator: thin adapter over spread_impl.
spread_estimator <- function(x) {
  check_non_weighted("x", x)
  spread_val <- spread_impl(x$sorted_values, assume_sorted = TRUE)
  Measurement$new(spread_val, x$unit)
}
