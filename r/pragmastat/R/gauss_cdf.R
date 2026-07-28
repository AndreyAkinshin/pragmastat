# The standard normal CDF, and the standard normal density.
#
# R has pnorm, and pnorm is fine. That is not the point of this file. The Edgeworth branches
# of the margins feed a threshold comparison whose result is an integer index, and an index
# selects an order statistic: a difference in the last bits of the CDF does not perturb the
# answer by a little, it returns a different observation. So the seven implementations have to
# evaluate the same approximation rather than each the best one available to it, and this is
# that approximation.
#
# Substituting pnorm here was a local improvement that broke a cross-language contract.
# Measured before this file existed: signed_rank_margin(64, 0.011834092546373905) returned
# 1330 in R against 1332 everywhere else, and pairwise_margin(201, 200, 0.012272550191216047)
# returned 34394 against 34396. Both are Edgeworth-branch inputs, and both agree now.
#
# The shape is two Chebyshev-fitted Horner chains and one exponential. The coefficients are
# produced by tests/oracles/fit_gauss_cdf.py against a reference good to 36 digits, so they
# are reproducible rather than transcribed; the worst relative error over |x| <= 6 is 9.1e-15.
#
# The chains are written one product per line to match the other six exactly. Do not
# reassociate them and do not replace them with a vectorised expression: the point is that
# every implementation performs the identical sequence of binary64 operations. No rounding
# needs pinning here, unlike Go: R evaluates each arithmetic operator on its own and never
# fuses a multiply into an add.

# gauss_cdf computes P(Z <= x) for a standard normal Z.
gauss_cdf <- function(x) {
  t <- abs(x) / sqrt(2)
  if (t < 0.5) {
    s <- t * t
    u <- 8.0 * s - 1.0
    p <- -1.2757552949301143e-19
    p <- p * u + 1.2307154179828511e-17
    p <- p * u - 1.0890239994332592e-15
    p <- p * u + 8.774530700097397e-14
    p <- p * u - 6.3744178527620835e-12
    p <- p * u + 4.1270254211564467e-10
    p <- p * u - 2.347229163519518e-08
    p <- p * u + 1.151603779513705e-06
    p <- p * u - 4.762336934468491e-05
    p <- p * u + 0.0016130716680617086
    p <- p * u - 0.04364205888669792
    p <- p * u + 1.0830752376761712
    erf <- t * p
    if (x >= 0) {
      return(0.5 * (1.0 + erf))
    }
    return(0.5 * (1.0 - erf))
  }

  erfc <- 0.0
  if (t <= 4.3) {
    u <- (2.0 * t - 4.8) / 3.8
    p <- 2.403093649825437e-09
    p <- p * u - 6.533436159455495e-09
    p <- p * u + 1.334437871983186e-09
    p <- p * u - 2.5055474016226743e-09
    p <- p * u + 5.2376178949357336e-08
    p <- p * u - 1.341394638617228e-07
    p <- p * u + 2.5376572107855777e-07
    p <- p * u - 6.147631059669139e-07
    p <- p * u + 1.561533370779237e-06
    p <- p * u - 3.688982809059467e-06
    p <- p * u + 8.492013869441648e-06
    p <- p * u - 1.9344330869926753e-05
    p <- p * u + 4.3285002216779125e-05
    p <- p * u - 9.489727696113043e-05
    p <- p * u + 0.0002037912849869451
    p <- p * u - 0.0004282777524202283
    p <- p * u + 0.00087969639542425
    p <- p * u - 0.001763698443638436
    p <- p * u + 0.0034462452415540026
    p <- p * u - 0.00655166763664565
    p <- p * u + 0.012094345026186722
    p <- p * u - 0.021629099761798037
    p <- p * u + 0.037371670355588804
    p <- p * u - 0.06218492139115531
    p <- p * u + 0.09925390090168178
    p <- p * u - 0.15121195850373031
    p <- p * u + 0.21849873453703333
    erfc <- portable_exp(-(t * t)) * p
  }

  if (x >= 0) {
    return(1.0 - 0.5 * erfc)
  }
  0.5 * erfc
}

# gauss_pdf computes the standard normal density, spelled as the other six spell it. Both
# Edgeworth expansions reach their phi through here, so the two call sites the other ports
# write out separately are this one line.
gauss_pdf <- function(z) {
  portable_exp(-z * z / 2) / sqrt(2 * pi)
}
