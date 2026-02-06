# MedianBounds provides distribution-free bounds for the population median.
# Uses binomial distribution to determine which order statistics form the bounds.
#
# @param x Numeric vector of values
# @param misrate Misclassification rate (probability that true median falls outside bounds)
# @return List with 'lower' and 'upper' components
median_bounds <- function(x, misrate) {
  check_validity(x, SUBJECTS$X)

  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  n <- length(x)

  if (n < 2) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }

  sorted_x <- sort(x)

  min_misrate <- min_achievable_misrate_one_sample(n)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  alpha <- misrate / 2.0

  # Find the largest k where P(Bin(n,0.5) <= k) <= alpha
  lo <- 0
  for (k in 0:((n + 1) %/% 2 - 1)) {
    tail_prob <- binomial_tail_probability(n, k)
    if (tail_prob <= alpha) {
      lo <- k
    } else {
      break
    }
  }

  # Symmetric interval: hi = n - 1 - lo (0-based)
  hi <- n - 1 - lo

  if (hi < lo) {
    hi <- lo
  }
  if (hi >= n) {
    hi <- n - 1
  }

  # Convert to 1-based indexing for R
  return(list(lower = sorted_x[lo + 1], upper = sorted_x[hi + 1]))
}

# Computes P(X <= k) for X ~ Binomial(n, 0.5).
# Uses incremental binomial coefficient computation.
# Note: 2^n overflows double for n > 1024.
binomial_tail_probability <- function(n, k) {
  if (k < 0) return(0.0)
  if (k >= n) return(1.0)

  # Normal approximation with continuity correction for large n
  # (2^n overflows double for n > 1024)
  if (n > 1023) {
    mean_val <- n / 2.0
    std_val <- sqrt(n / 4.0)
    z <- (k + 0.5 - mean_val) / std_val
    return(pnorm(z))
  }

  sum_val <- 0.0
  coef <- 1.0  # C(n, 0) = 1
  total <- 2^n

  for (i in 0:k) {
    sum_val <- sum_val + coef
    coef <- coef * (n - i) / (i + 1)
  }

  return(sum_val / total)
}
