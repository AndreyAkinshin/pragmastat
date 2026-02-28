# ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
# The misrate represents the probability that the true shift falls outside the computed bounds.
#
# @param x Numeric vector or Sample object
# @param y Numeric vector or Sample object
# @param misrate Misclassification rate (probability that true shift falls outside bounds)
# @return Bounds object (when Sample input) or list with 'lower' and 'upper' (when vector input)
shift_bounds <- function(x, y, misrate = DEFAULT_MISRATE) {
  if (inherits(x, "Sample") && inherits(y, "Sample")) {
    return(shift_bounds_estimator(x, y, misrate))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)
  m <- length(y)

  min_misrate <- min_achievable_misrate_two_sample(n, m)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  xs <- sort(x)
  ys <- sort(y)

  total <- n * m

  if (total == 1) {
    value <- xs[1] - ys[1]
    return(list(lower = value, upper = value))
  }

  margin <- pairwise_margin(n, m, misrate)
  half_margin <- min(floor(margin / 2), floor((total - 1) / 2))
  k_left <- half_margin
  k_right <- (total - 1) - half_margin

  denominator <- total - 1
  if (denominator <= 0) {
    denominator <- 1
  }
  p_left <- k_left / denominator
  p_right <- k_right / denominator

  quantiles <- fast_shift(xs, ys, c(p_left, p_right), assume_sorted = TRUE)
  lower <- min(quantiles[1], quantiles[2])
  upper <- max(quantiles[1], quantiles[2])

  return(list(lower = lower, upper = upper))
}

# Internal Sample-based estimator
shift_bounds_estimator <- function(x, y, misrate) {
  check_non_weighted("x", x)
  check_non_weighted("y", y)
  x <- x$with_subject("x")
  y <- y$with_subject("y")
  check_compatible_units(x, y)
  pair <- convert_to_finer(x, y)
  x <- pair$a
  y <- pair$b

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- x$size
  m <- y$size

  min_misrate <- min_achievable_misrate_two_sample(n, m)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  xs <- x$sorted_values
  ys <- y$sorted_values

  total <- as.numeric(n) * as.numeric(m)

  if (total == 1) {
    value <- xs[1] - ys[1]
    return(Bounds$new(value, value, x$unit))
  }

  margin <- pairwise_margin(n, m, misrate)
  half_margin <- min(floor(margin / 2), floor((total - 1) / 2))
  k_left <- half_margin
  k_right <- (total - 1) - half_margin

  denominator <- total - 1
  if (denominator <= 0) {
    denominator <- 1
  }
  p_left <- k_left / denominator
  p_right <- k_right / denominator

  quantiles <- fast_shift(xs, ys, c(p_left, p_right), assume_sorted = TRUE)
  lower <- min(quantiles[1], quantiles[2])
  upper <- max(quantiles[1], quantiles[2])

  Bounds$new(lower, upper, x$unit)
}
