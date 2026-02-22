# SignMargin for one-sample bounds based on Binomial(n, 0.5).
# Computes randomized cutoffs for sign-test bounds.

sign_margin_randomized <- function(n, misrate, rng) {
  if (n <= 0) stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }
  min_misrate <- min_achievable_misrate_one_sample(n)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  target <- misrate / 2
  if (target <= 0) {
    return(0L)
  }
  if (target >= 1) {
    return(as.integer(n * 2))
  }

  split <- binom_cdf_split(n, target)
  r_low <- split$r_low
  log_cdf_low <- split$log_cdf_low
  log_pmf_high <- split$log_pmf_high

  log_target <- log(target)
  log_num <- if (log_target > log_cdf_low) log_sub_exp(log_target, log_cdf_low) else -Inf

  if (is.finite(log_pmf_high) && is.finite(log_num)) {
    p <- exp(log_num - log_pmf_high)
  } else {
    p <- 0
  }
  p <- max(0, min(1, p))

  u <- rng$uniform_float()
  r <- if (u < p) r_low + 1L else r_low
  return(as.integer(r * 2))
}

binom_cdf_split <- function(n, target) {
  log_target <- log(target)
  log_pmf <- -n * log(2)
  log_cdf <- log_pmf
  r_low <- 0L

  if (log_cdf > log_target) {
    return(list(r_low = 0L, log_cdf_low = log_cdf, log_pmf_high = log_pmf))
  }

  for (k in 1:n) {
    log_pmf_next <- log_pmf + log(n - k + 1) - log(k)
    log_cdf_next <- log_add_exp(log_cdf, log_pmf_next)
    if (log_cdf_next > log_target) {
      return(list(r_low = r_low, log_cdf_low = log_cdf, log_pmf_high = log_pmf_next))
    }
    r_low <- as.integer(k)
    log_pmf <- log_pmf_next
    log_cdf <- log_cdf_next
  }

  return(list(r_low = r_low, log_cdf_low = log_cdf, log_pmf_high = -Inf))
}

log_add_exp <- function(a, b) {
  if (is.infinite(a) && a < 0) {
    return(b)
  }
  if (is.infinite(b) && b < 0) {
    return(a)
  }
  m <- max(a, b)
  m + log(exp(a - m) + exp(b - m))
}

log_sub_exp <- function(a, b) {
  if (is.infinite(b) && b < 0) {
    return(a)
  }
  diff <- exp(b - a)
  if (diff >= 1) {
    return(-Inf)
  }
  a + log(1 - diff)
}
