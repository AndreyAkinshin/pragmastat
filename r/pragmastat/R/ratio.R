# Measures how many times larger x is compared to y (Ratio)
#
# Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
# Equivalent to: exp(Shift(log(x), log(y)))
# For example, ratio = 1.2 means x is typically 20% larger than y.
# Uses fast O((m + n) * log(precision)) algorithm.
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

  # Log-transform (includes positivity check)
  log_x <- log_transform(x, SUBJECTS$X, "Ratio")
  log_y <- log_transform(y, SUBJECTS$Y, "Ratio")

  # Compute shift, exp-transform back
  log_result <- fast_shift(log_x, log_y, p = 0.5, assume_sorted = FALSE)
  exp(log_result)
}
