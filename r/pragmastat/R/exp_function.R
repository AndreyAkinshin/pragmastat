# The exponential every port evaluates, in place of the platform's.
#
# IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
# nothing about the exponential. Conforming libraries disagree in the last bit and do:
# measured on one Edgeworth crossover, Go's software exp and glibc's return neighboring
# values, which moved the reported margin by two. Since a margin selects an order statistic,
# that is a different confidence interval from the same inputs.
#
# So the exponential cannot be the platform's. This one is built from operations the standard
# does fix:
#
#     k = floor(y/ln2 + 1/2),  r = (y - k*ln2_hi) - k*ln2_lo,  exp(y) = 2^k * exp(r)
#
# with exp(r) on |r| <= ln2/2 from the polynomial in tests/oracles/fit_exp.py. Fitting
# (exp(r) - 1 - r)/r^2 rather than exp(r) keeps the two leading terms exact and leaves the
# polynomial supplying a correction below 0.07, so the assembled result stays within a couple
# of ulp of the true exponential while being reproducible everywhere.
#
# floor(x + 1/2) rather than round(): R rounds halves to even and Go rounds them away from
# zero, so naming a rounding is naming a disagreement.
#
# No rounding needs pinning here, unlike Go: R evaluates each arithmetic operator on its own
# and never fuses a multiply into an add.

# Constants of the range reduction, emitted by tests/oracles/fit_exp.py.
#
# ln 2 is split so that k*.LN2_HI is exact: .LN2_HI carries 32 significant bits and |k| needs
# at most 11, which leaves the product inside the 53 available. Without the split the
# reduction would lose the low bits of r, and r is where the accuracy lives.
.INV_LN2 <- 1.4426950408889634e+00
.LN2_HI <- 6.9314718036912382e-01
.LN2_LO <- 1.9082149292705877e-10

exp_function <- function(y) {
  if (is.na(y)) {
    return(y)
  }
  # Past these the answer is not in doubt, and stating the cutoffs keeps the reduction
  # from having to produce a k it cannot scale by.
  if (y > 709.79) {
    return(Inf)
  }
  if (y < -745.2) {
    return(0)
  }

  # The two halves of the reduction are kept apart until the assembly below, which is where
  # the accuracy is: see the comment there.
  k <- floor(y * .INV_LN2 + 0.5)
  hi <- y - k * .LN2_HI
  lo <- k * .LN2_LO
  r <- hi - lo

  q <- 1.6086622436215554e-10
  q <- q * r + 2.0918129454967065e-09
  q <- q * r + 2.5052071109214447e-08
  q <- q * r + 2.7557263301474753e-07
  q <- q * r + 2.7557319247204789e-06
  q <- q * r + 2.4801587336421322e-05
  q <- q * r + 1.9841269841263304e-04
  q <- q * r + 1.3888888888879082e-03
  q <- q * r + 8.3333333333333332e-03
  q <- q * r + 4.1666666666666678e-02
  q <- q * r + 1.6666666666666666e-01
  q <- q * r + 5.0000000000000000e-01
  # Assembled from hi and lo rather than from r. Writing 1 + r + r*r*q discards the low bits
  # of r: r reaches 0.35, adding it to 1 shifts the mantissa two places, and the tail falls
  # off the end. Reconstructing the same sum from the two halves of the reduction keeps them,
  # and costs nothing, being the same operations in a different order. Measured against a
  # 60-digit reference over the band these estimators reach, it halves the worst relative
  # error, from 2.19e-16 to 1.20e-16, and raises the share of correctly rounded results from
  # 72.6% to 90.0%. That is where fdlibm sits, which reaches it with a division this does
  # not need.
  p <- 1.0 - ((lo - r * r * q) - hi)

  # Two scalings rather than one. Splitting k in half keeps the first factor inside the
  # normal range whatever k is, so only the second can denormalize or overflow, and it
  # does so in a single rounding.
  #
  # R has no ldexp, so the scalings are spelled 2^n. That is exact for every n reachable
  # here, which test-exp-function.R checks against repeated doubling rather than assuming.
  half <- trunc(k / 2)
  (p * 2^half) * 2^(k - half)
}
