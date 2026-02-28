# Estimates data dispersion (Spread)
#
# Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
# More robust than standard deviation and more efficient than MAD.
# Uses fast O(n log n) algorithm.
#
# Assumptions:
#   - sparity(x) - sample must be non tie-dominant (Spread > 0)
#
# @param x Numeric vector or Sample object
# @return Measurement (when Sample input) or numeric (when vector input)
spread <- function(x) {
  if (inherits(x, "Sample")) {
    return(spread_estimator(x))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
  spread_val <- fast_spread(x)
  if (spread_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_val
}

# Internal Sample-based estimator
spread_estimator <- function(x) {
  check_non_weighted("x", x)
  spread_val <- fast_spread(x$values)
  if (spread_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, x$subject))
  }
  Measurement$new(spread_val, x$unit)
}
