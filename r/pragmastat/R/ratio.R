# Measures how many times larger x is compared to y (Ratio)
#
# Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
# Equivalent to: exp(Shift(log(x), log(y)))
#
# Assumptions:
#   - positivity(x) - all values in x must be strictly positive
#   - positivity(y) - all values in y must be strictly positive
#
# Public API: accepts either native numeric vectors (returns a plain unitless
# numeric) or Samples (returns a Measurement). The `assume_sorted` flag (vector
# path only) lets callers with already-ascending data skip the internal sort.
# Passing assume_sorted = TRUE on unsorted input is undefined behavior: the
# caller is responsible for the ordering and gets a wrong result on misuse.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param assume_sorted If TRUE, assume the vector inputs are already sorted
#   ascending and skip the internal sort (vector input only). Ignored for Sample
#   input, which always reuses its cached sorted views.
# @return Measurement (when Sample input) or numeric (when vector input)
ratio <- function(x, y, assume_sorted = FALSE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(ratio_estimator(x, y))
  }
  # Native-array (raw) interface: unitless numeric result.
  ratio_impl(x, y, assume_sorted)
}

# Single implementation on raw values. log is monotonic, so a sorted positive
# input yields a sorted log output; `assume_sorted` therefore propagates straight
# to the shift over the log-transformed values. Both vector and Sample paths
# route through here.
ratio_impl <- function(x, y, assume_sorted) {
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)
  log_x <- log_transform(x, SUBJECTS$X)
  log_y <- log_transform(y, SUBJECTS$Y)
  log_result <- shift_impl_compute(log_x, log_y, p = 0.5, assume_sorted = assume_sorted)
  exp(log_result)
}

# Internal Sample-based estimator: thin adapter over ratio_impl.
ratio_estimator <- function(x, y) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  result <- ratio_impl(
    x$sorted_values, y$sorted_values,
    assume_sorted = TRUE
  )
  Measurement$new(result, ratio_unit)
}
