# PairwiseMargin determines how many extreme pairwise differences to exclude
# when constructing bounds based on the distribution of dominance statistics.
# Uses exact calculation for small samples (n+m <= 400) and Edgeworth
# approximation for larger samples.
#
# @param n Sample size of first sample (must be positive)
# @param m Sample size of second sample (must be positive)
# @param misrate Misclassification rate (must be in [0, 1])
# @return Integer representing the total margin split between lower and upper tails
# @throws Error if n <= 0, m <= 0, or misrate is outside [0, 1]
pairwise_margin <- function(n, m, misrate) {
  if (n <= 0) {
    stop("n must be positive")
  }
  if (m <= 0) {
    stop("m must be positive")
  }
  if (misrate < 0 || misrate > 1) {
    stop("misrate must be in range [0, 1]")
  }

  # Use exact method for small to medium samples
  if (n + m <= 400) {
    return(pairwise_margin_exact(n, m, misrate))
  }
  return(pairwise_margin_approx(n, m, misrate))
}

# pairwise_margin_exact uses the exact distribution based on Loeffler's recurrence
pairwise_margin_exact <- function(n, m, misrate) {
  return(pairwise_margin_exact_raw(n, m, misrate / 2) * 2)
}

# pairwise_margin_approx uses Edgeworth approximation for large samples
pairwise_margin_approx <- function(n, m, misrate) {
  return(pairwise_margin_approx_raw(n, m, misrate / 2) * 2)
}

# pairwise_margin_exact_raw implements the inversed Loeffler (1982) algorithm
# Reference: "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
pairwise_margin_exact_raw <- function(n, m, p) {
  # Use R's built-in choose() function for binomial coefficient
  # For large values, use logarithmic calculation
  if (n + m < 65) {
    total <- choose(n + m, m)
  } else {
    total <- exp(lchoose(n + m, m))
  }

  pmf <- c(1) # pmf[1] = 1 (R uses 1-based indexing)
  sigma <- c(0) # sigma[1] is unused

  u <- 0
  cdf <- 1.0 / total

  if (cdf >= p) {
    return(0)
  }

  repeat {
    u <- u + 1

    # Ensure sigma has entry for u+1 (R uses 1-based indexing)
    if (length(sigma) <= u) {
      value <- 0
      for (d in 1:n) {
        if (u %% d == 0 && u >= d) {
          value <- value + d
        }
      }
      for (d in (m + 1):(m + n)) {
        if (u %% d == 0 && u >= d) {
          value <- value - d
        }
      }
      sigma <- c(sigma, value)
    }

    # Compute pmf[u+1] using Loeffler recurrence
    sum_val <- 0.0
    for (i in 0:(u - 1)) {
      sum_val <- sum_val + pmf[i + 1] * sigma[u - i + 1]
    }
    sum_val <- sum_val / u
    pmf <- c(pmf, sum_val)

    cdf <- cdf + sum_val / total

    if (cdf >= p) {
      return(u)
    }
    if (sum_val == 0) {
      break
    }
  }

  return(length(pmf) - 1)
}

# pairwise_margin_approx_raw uses inverse Edgeworth approximation
pairwise_margin_approx_raw <- function(n, m, misrate) {
  a <- 0
  b <- n * m
  while (a < b - 1) {
    c <- floor((a + b) / 2)
    p <- edgeworth_cdf(n, m, c)
    if (p < misrate) {
      a <- c
    } else {
      b <- c
    }
  }

  if (edgeworth_cdf(n, m, b) < misrate) {
    return(b)
  }
  return(a)
}

# edgeworth_cdf computes the CDF using Edgeworth expansion
edgeworth_cdf <- function(n, m, u) {
  mu <- (n * m) / 2.0
  su <- sqrt((n * m * (n + m + 1)) / 12.0)
  z <- (u - mu - 0.5) / su

  # Use R's built-in normal distribution functions
  phi <- dnorm(z) # Standard normal PDF
  Phi <- pnorm(z) # Standard normal CDF

  # Pre-compute powers of n and m for efficiency
  n2 <- n * n
  n3 <- n2 * n
  n4 <- n2 * n2
  m2 <- m * m
  m3 <- m2 * m
  m4 <- m2 * m2

  # Compute moments
  mu2 <- (n * m * (n + m + 1)) / 12.0
  mu4 <- (n * m * (n + m + 1)) *
    (5 * m * n * (m + n) -
      2 * (m2 + n2) +
      3 * m * n -
      2 * (n + m)) / 240.0

  mu6 <- (n * m * (n + m + 1)) *
    (35 * m2 * n2 * (m2 + n2) +
      70 * m3 * n3 -
      42 * m * n * (m3 + n3) -
      14 * m2 * n2 * (n + m) +
      16 * (n4 + m4) -
      52 * n * m * (n2 + m2) -
      43 * n2 * m2 +
      32 * (m3 + n3) +
      14 * m * n * (n + m) +
      8 * (n2 + m2) +
      16 * n * m -
      8 * (n + m)) / 4032.0

  # Pre-compute powers of mu2 and related terms
  mu2_2 <- mu2 * mu2
  mu2_3 <- mu2_2 * mu2
  mu4_mu2_2 <- mu4 / mu2_2

  # Factorial constants: 4! = 24, 6! = 720, 8! = 40320
  e3 <- (mu4_mu2_2 - 3) / 24.0
  e5 <- (mu6 / mu2_3 - 15 * mu4_mu2_2 + 30) / 720.0
  e7 <- 35 * (mu4_mu2_2 - 3) * (mu4_mu2_2 - 3) / 40320.0

  # Pre-compute powers of z for Hermite polynomials
  z2 <- z * z
  z3 <- z2 * z
  z5 <- z3 * z2
  z7 <- z5 * z2

  # Hermite polynomials
  f3 <- -phi * (z3 - 3 * z)
  f5 <- -phi * (z5 - 10 * z3 + 15 * z)
  f7 <- -phi * (z7 - 21 * z5 + 105 * z3 - 105 * z)

  edgeworth <- Phi + e3 * f3 + e5 * f5 + e7 * f7
  return(max(0, min(edgeworth, 1)))
}
