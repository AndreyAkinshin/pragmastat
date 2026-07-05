# Center estimates the central value of the data (Hodges-Lehmann estimator).
# Calculates the median of all pairwise averages (x[i] + x[j])/2.
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
center <- function(x, assume_sorted = FALSE) {
  if (inherits(x, "Sample")) {
    return(center_estimator(x))
  }
  # Native-array (raw) interface: unitless numeric result.
  center_impl(x, assume_sorted)
}

# Single implementation on raw values. Both the vector path and the Sample path
# (via center_estimator, which passes the cached sorted view) route through here.
# Delegates the O(n log n) Monahan selection to the C kernel.
center_impl <- function(values, assume_sorted = FALSE) {
  check_validity(values, SUBJECTS$X)
  center_impl_compute(values, assume_sorted)
}

# Internal Sample-based estimator: thin adapter over center_impl.
center_estimator <- function(x) {
  check_non_weighted("x", x)
  result <- center_impl(x$sorted_values, assume_sorted = TRUE)
  Measurement$new(result, x$unit)
}
