# ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
# The misrate represents the probability that the true shift falls outside the computed bounds.
# This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
#
# @param x Numeric vector of values
# @param y Numeric vector of values
# @param misrate Misclassification rate (probability that true shift falls outside bounds)
# @return List with 'lower' and 'upper' components
shift_bounds <- function(x, y, misrate) {
  if (!is.numeric(x) || !is.numeric(y)) {
    stop("x and y must be numeric vectors")
  }
  n <- length(x)
  m <- length(y)

  if (n == 0 || m == 0) {
    stop("Input vectors cannot be empty")
  }

  # Sort both arrays
  xs <- sort(x)
  ys <- sort(y)

  total <- n * m

  # Special case: when there's only one pairwise difference, bounds collapse to a single value
  if (total == 1) {
    value <- xs[1] - ys[1]
    return(list(lower = value, upper = value))
  }

  margin <- pairwise_margin(n, m, misrate)
  half_margin <- min(floor(margin / 2), floor((total - 1) / 2))
  k_left <- half_margin
  k_right <- (total - 1) - half_margin

  # Compute quantile positions
  denominator <- total - 1
  if (denominator <= 0) {
    denominator <- 1
  }
  p_left <- k_left / denominator
  p_right <- k_right / denominator

  # Use fast_shift to compute quantiles of pairwise differences
  # fast_shift uses Type-7 quantile interpolation
  quantiles <- fast_shift(xs, ys, c(p_left, p_right), assume_sorted = TRUE)
  lower <- min(quantiles[1], quantiles[2])
  upper <- max(quantiles[1], quantiles[2])

  return(list(lower = lower, upper = upper))
}
