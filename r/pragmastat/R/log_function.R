# The natural logarithm this toolkit evaluates, in place of the platform's.
#
# IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes nothing
# about the logarithm. The reason this one has to be ours is stronger than for most
# transcendentals: ratio is exp(median(log x - log y)), so a last-bit difference in log moves the
# median's input and changes which pairwise element is selected. That is a different answer, not a
# small error in the same one.
#
# The constants and the polynomial come from tests/oracles/fit_log.py, which documents the
# reduction and what is fitted. Which of the two range endpoints a port names depends on how it
# splits x: a library frexp returns a significand in [0.5, 1) and the reduction doubles the low
# end, while reading the exponent field gives one in [1, 2) and it halves the high end. Both
# arrive at the same [sqrt(2)/2, sqrt(2)) and the seven agree bit for bit; only the endpoint each
# has to name differs.

# Constants of the range reduction. ln 2 is split so that k*LOG_LN2_HI is exact: it carries 32
# significant bits and |k| needs at most 11, which leaves the product inside the 53 available.
LOG_LN2_HI <- 0.6931471803691238
LOG_LN2_LO <- 1.9082149292705877e-10
LOG_SQRT2 <- 1.4142135623730951

# Chebyshev fit of (log((1+s)/(1-s)) - 2*s) / s^3 over the reduced range. The leading terms are
# the recognizable 2/3, 2/5, 2/7 of the odd series doubled, which is where the assembly's missing
# factor of two lives; the tail departs from the series because the fit minimizes the worst error
# rather than matching Taylor.
LOG_POLY_P <- c(
  0.666666666666667,
  0.39999999999899505,
  0.28571428625975487,
  0.2222221113479508,
  0.18182889125261723,
  0.15331721600556042,
  0.14616449685043406
)

# Returns the natural logarithm of x.
#
# The special cases follow IEEE 754 and match what every platform returns for them, since they are
# the cases a platform has no freedom about.
log_function <- function(x) {
  # Coerced, not assumed. The reduction below reads the exponent out of the binary64 payload, and
  # writeBin serializes an R integer as an integer: writeBin(4L, raw(), size = 8) is 4, not 4.0,
  # so the exponent field reads as zero and every integer input returns -1023*ln2. It fails
  # silently, and it is reachable from the public API, because the reference fixtures write whole
  # numbers as "4" and jsonlite types those as integer. double_bits() in the test helper coerces
  # for the same reason.
  x <- as.double(x)
  if (is.na(x) && !is.nan(x)) {
    return(NA_real_)
  }
  if (is.nan(x)) {
    return(NaN)
  }
  if (x < 0) {
    return(NaN)
  }
  if (x == 0) {
    return(-Inf)
  }
  if (is.infinite(x)) {
    return(Inf)
  }
  if (x == 1) {
    return(0)
  }

  # frexp is spelled by reading the exponent field, because arithmetic cannot do it here: at the
  # bottom of the range 2^exponent is subnormal, so dividing by it overflows to Inf and a loop
  # that corrects afterwards never terminates. Subnormals are brought into the normal range first,
  # since their exponent field carries no information. Scaling by a power of two is exact.
  value <- x
  k <- 0L
  if (value < 2.2250738585072014e-308) {
    value <- value * 1.8014398509481984e16
    k <- k - 54L
  }

  # endian = "big" on both ends, so raw_bits[1] is the sign-and-exponent byte on every host. The
  # native order plus rev() would agree with this on x86 and disagree on a big-endian one, and
  # nothing here would notice: the bytes would still assemble into a double, just the wrong one.
  raw_bits <- as.integer(writeBin(value, raw(), size = 8, endian = "big"))
  biased <- bitwShiftL(bitwAnd(raw_bits[1], 0x7f), 4) + bitwShiftR(raw_bits[2], 4)
  k <- k + biased - 1023L
  # Replace the exponent field with the one for [1, 2).
  raw_bits[1] <- 0x3f
  raw_bits[2] <- bitwOr(bitwAnd(raw_bits[2], 0x0f), 0xf0)
  m <- readBin(as.raw(raw_bits), "double", size = 8, endian = "big")

  # Recenter on [sqrt(2)/2, sqrt(2)) so |f| stays small at both ends. Halving is exact.
  if (m >= LOG_SQRT2) {
    m <- m * 0.5
    k <- k + 1L
  }

  f <- m - 1
  s <- f / (2 + f)
  z <- s * s

  acc <- LOG_POLY_P[length(LOG_POLY_P)]
  for (i in (length(LOG_POLY_P) - 1):1) {
    acc <- acc * z + LOG_POLY_P[i]
  }
  r <- z * acc
  hfsq <- 0.5 * f * f
  kf <- as.double(k)

  if (k == 0L) {
    f - (hfsq - s * (hfsq + r))
  } else {
    kf * LOG_LN2_HI - ((hfsq - (s * (hfsq + r) + kf * LOG_LN2_LO)) - f)
  }
}
