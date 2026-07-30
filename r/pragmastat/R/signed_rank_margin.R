# SignedRankMargin function for one-sample bounds.
# One-sample analog of PairwiseMargin using Wilcoxon signed-rank distribution.
#
# @param n Sample size (must be positive)
# @param misrate Misclassification rate (must be in [0, 1])
# @return Integer margin
# @throws Error if inputs are invalid or misrate is below minimum achievable
signed_rank_margin <- function(n, misrate) {
  if (n <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }
  if (is.nan(misrate) || misrate < 0 || misrate > 1) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  min_misrate <- min_achievable_misrate_one_sample(n)
  if (misrate < min_misrate) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
  }

  # Maximum n for exact computation
  if (n <= 63) {
    return(signed_rank_margin_exact(n, misrate))
  }
  return(signed_rank_margin_approx(n, misrate))
}

# Computes one-sided margin using exact Wilcoxon signed-rank distribution.
signed_rank_margin_exact <- function(n, misrate) {
  signed_rank_margin_exact_raw(n, misrate / 2) * 2
}

# The dynamic programming runs in C, on 64-bit integers, because R has none.
#
# The counts themselves fit a double: the largest is 4.5e14 at n = 57, inside the 2^53 a double
# represents exactly. The cumulative sum does not. It climbs to 2^(n-1), and it feeds a comparison
# against p, so the bits it loses decide an integer. They did: the signed-rank distribution is
# symmetric, so where max_w is odd the cumulative at the midpoint is exactly half the total and
# `cdf >= p` at p = 1/2 is an exact equality. Accumulated in doubles it came out a hair below, and
# signed_rank_margin(57, 1) returned 1654 here against 1652 in the other six ports, in a suite the
# manifest declares exact.
#
# Same answer as binomial_coefficient_c, for the same reason: an integer by definition is
# accumulated as one and converted once.
signed_rank_margin_exact_raw <- function(n, p) {
  .Call("signed_rank_margin_exact_raw_c", as.integer(n), as.numeric(p), PACKAGE = "pragmastat")
}

# Computes one-sided margin using Edgeworth approximation for large n.
signed_rank_margin_approx <- function(n, misrate) {
  signed_rank_margin_approx_raw(n, misrate / 2) * 2
}

signed_rank_margin_approx_raw <- function(n, misrate) {
  max_w <- (n * (n + 1)) %/% 2
  a <- 0
  b <- max_w

  while (a < b - 1) {
    c <- (a + b) %/% 2
    cdf <- signed_rank_edgeworth_cdf(n, c)
    if (cdf < misrate) {
      a <- c
    } else {
      b <- c
    }
  }

  if (signed_rank_edgeworth_cdf(n, b) < misrate) {
    return(b)
  }
  return(a)
}

# Edgeworth expansion for Wilcoxon signed-rank distribution CDF.
signed_rank_edgeworth_cdf <- function(n, w) {
  mu <- n * (n + 1) / 4.0
  sigma2 <- n * (n + 1) * (2 * n + 1) / 24.0
  sigma <- sqrt(sigma2)

  # +0.5 continuity correction: computing P(W <= w) for a left-tail discrete CDF
  z <- (w - mu + 0.5) / sigma
  phi <- additive_pdf(z)
  big_phi <- additive_cumulative(z)

  nf <- as.double(n)
  kappa4 <- -nf * (nf + 1) * (2 * nf + 1) * (3 * nf * nf + 3 * nf - 1) / 240.0

  e3 <- kappa4 / (24 * sigma2 * sigma2)

  z2 <- z * z
  z3 <- z2 * z
  f3 <- -phi * (z3 - 3 * z)

  edgeworth <- big_phi + e3 * f3
  return(max(0, min(1, edgeworth)))
}
