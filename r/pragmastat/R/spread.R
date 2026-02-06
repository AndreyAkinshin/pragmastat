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
  # Check sparity (priority 2)
  check_sparity(x, SUBJECTS$X)
  # Use fast O(n log n) algorithm
  fast_spread(x)
}
