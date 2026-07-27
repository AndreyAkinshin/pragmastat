# Standard normal CDF using ACM Algorithm 209, and the standard normal density.
#
# R has pnorm and dnorm, and they are more accurate than this polynomial. That is exactly
# why this file exists. The Edgeworth branches of the margins feed a threshold comparison
# whose result is an integer index, and an index selects an order statistic: a difference
# in the last bits of the CDF does not perturb the answer by a little, it returns a
# different observation. So the seven implementations have to evaluate the same
# approximation, not each the best one available to it.
#
# Substituting pnorm here was a local improvement that broke a cross-language contract.
# Measured before this file existed: signed_rank_margin(64, 0.011834092546373905) returned
# 1330 in R against 1332 everywhere else, and pairwise_margin(201, 200, 0.012272550191216047)
# returned 34394 against 34396. Both are Edgeworth-branch inputs, and both agree now.
#
# The chains are written one product per line to match the other six exactly. Do not
# reassociate them and do not replace them with a vectorised expression: the point is that
# every implementation performs the identical sequence of binary64 operations.

# gauss_cdf computes P(Z <= x) for a standard normal Z.
gauss_cdf <- function(x) {
  if (abs(x) < 1e-9) {
    z <- 0.0
  } else {
    y <- abs(x) / 2
    if (y >= 3.0) {
      z <- 1.0
    } else if (y < 1.0) {
      w <- y * y
      p <- 0.000124818987
      p <- p * w - 0.001075204047
      p <- p * w + 0.005198775019
      p <- p * w - 0.019198292004
      p <- p * w + 0.059054035642
      p <- p * w - 0.151968751364
      p <- p * w + 0.319152932694
      p <- p * w - 0.531923007300
      p <- p * w + 0.797884560593
      z <- p * y * 2.0
    } else {
      y <- y - 2.0
      p <- -0.000045255659
      p <- p * y + 0.000152529290
      p <- p * y - 0.000019538132
      p <- p * y - 0.000676904986
      p <- p * y + 0.001390604284
      p <- p * y - 0.000794620820
      p <- p * y - 0.002034254874
      p <- p * y + 0.006549791214
      p <- p * y - 0.010557625006
      p <- p * y + 0.011630447319
      p <- p * y - 0.009279453341
      p <- p * y + 0.005353579108
      p <- p * y - 0.002141268741
      p <- p * y + 0.000535310849
      z <- p * y + 0.999936657524
    }
  }

  if (x > 0.0) {
    (z + 1.0) / 2
  } else {
    (1.0 - z) / 2
  }
}

# gauss_pdf computes the standard normal density, spelled as the other six spell it.
gauss_pdf <- function(z) {
  exp(-z * z / 2) / sqrt(2 * pi)
}
