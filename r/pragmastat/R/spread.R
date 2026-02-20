# Estimates data dispersion (Spread)
#
# Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
# More robust than standard deviation and more efficient than MAD.
# Uses fast O(n log n) algorithm.
#
# Assumptions:
#   - sparity(x) - sample must be non tie-dominant (Spread > 0)
#
# @param x Numeric vector of values
# @return The spread estimate
spread <- function(x) {
  # Check validity (priority 0)
  check_validity(x, SUBJECTS$X)
  spread_val <- fast_spread(x)
  if (spread_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, SUBJECTS$X))
  }
  spread_val
}
