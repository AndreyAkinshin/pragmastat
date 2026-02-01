# Measures how many times larger x is compared to y (Ratio)
#
# Calculates the median of all pairwise ratios (x[i] / y[j]).
# For example, ratio = 1.2 means x is typically 20% larger than y.
#
# Assumptions:
#   - positivity(x) - all values in x must be strictly positive
#   - positivity(y) - all values in y must be strictly positive
#
# @param x Numeric vector of values
# @param y Numeric vector of values
# @return The ratio estimate
ratio <- function(x, y) {
  # Check validity for x (priority 0, subject x)
  check_validity(x, SUBJECTS$X, "Ratio")
  # Check validity for y (priority 0, subject y)
  check_validity(y, SUBJECTS$Y, "Ratio")
  # Check positivity for x (priority 1, subject x)
  check_positivity(x, SUBJECTS$X, "Ratio")
  # Check positivity for y (priority 1, subject y)
  check_positivity(y, SUBJECTS$Y, "Ratio")

  pairwise_ratios <- outer(x, y, "/")
  median(pairwise_ratios)
}
