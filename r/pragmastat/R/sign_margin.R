# SignMargin for one-sample bounds based on Binomial(n, 0.5), computed without a single library
# call.
#
# It used to be evaluated in log space: nine calls to log and exp, one of them inside a loop that
# runs n times. IEEE 754 fixes nothing about either function, and this value is not merely returned
# to the caller: the margin selects an order statistic, so a difference between two conforming libm
# implementations becomes a different confidence interval from identical inputs. It did. Two ports
# disagreed on spread_bounds for a sample of 200 consecutive integers.
#
# No logarithm is needed. Binomial(n, 1/2) has an exact rational distribution function, and the two
# quantities the randomization wants are its partial sum and the next term. Both follow from the
# same multiplicative recurrence the binomial coefficient uses: one multiply and one divide per
# step, plus a scaling by a power of two, and IEEE 754 pins all three.
#
# The scaling is what makes the recurrence work at any n. pmf(0) is 2^-n, which underflows to zero
# past n = 1074, so the running term is carried as w * 2^e with the exponent tracked separately: w
# stays in the normal range and e absorbs the magnitude. Rescaling happens by multiplying by a
# power of two, which is exact, so it costs no accuracy and changes no bits.
#
# Measured against exact rational arithmetic over 195 (n, misrate) pairs spanning n = 1 to 5000 and
# misrate from 1 down to the smallest positive double: the selected index is right every time, and
# the randomization probability is within 6.1e-13. The log-space version it replaces reached
# 1.9e-11 on the same set, thirty times further out, and did it differently in each port.

# How far the running term is rescaled when it grows too large. Any power of two works; 512 keeps
# the rescaling rare without letting w approach the overflow threshold.
.SIGN_MARGIN_SCALE_STEP <- 512L

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

  u <- rng$uniform_float()
  r <- if (u < split$p) split$r_low + 1L else split$r_low
  return(as.integer(r * 2))
}

# The largest k whose Binomial(n, 0.5) CDF does not exceed target, together with the fraction of
# the next term that would be needed to reach it. The caller compares that fraction against a
# uniform draw, which is what makes the margin achieve the requested misrate exactly rather than
# the next admissible one below it.
binom_cdf_split <- function(n, target) {
  # Binomial(n, 1/2) is symmetric, so for odd n the distribution function at (n-1)/2 is exactly one
  # half. No approximation reproduces an exact equality, and misrate = 1 lands on it: the summation
  # would decide the comparison by its last accumulated bit.
  if (target == 0.5 && n %% 2L == 1L) {
    return(list(r_low = as.integer((n - 1L) %/% 2L), p = 0))
  }

  scale_up <- ldexp_exact(1, .SIGN_MARGIN_SCALE_STEP)
  scale_down <- ldexp_exact(1, -.SIGN_MARGIN_SCALE_STEP)

  # The running term pmf(k) is w * 2^e, starting from pmf(0) = 2^-n.
  w <- 1
  e <- -n
  cdf <- 1

  if (ldexp_exact(cdf, e) > target) {
    return(list(r_low = 0L, p = 0))
  }

  r_low <- 0L
  for (k in 1:n) {
    w <- w * (n - k + 1) / k
    while (w > scale_up) {
      w <- w * scale_down
      cdf <- cdf * scale_down
      e <- e + .SIGN_MARGIN_SCALE_STEP
    }
    nxt <- cdf + w
    if (ldexp_exact(nxt, e) > target) {
      # target and cdf are both in units of 2^e here, so the fraction is a plain quotient.
      p <- (ldexp_exact(target, -e) - cdf) / w
      return(list(r_low = r_low, p = max(0, min(1, p))))
    }
    r_low <- as.integer(k)
    cdf <- nxt
  }

  return(list(r_low = r_low, p = 0))
}

# v * 2^exp, exactly, including where the result leaves the normal range.
#
# R's 2^exp is a general power for a general exponent and is not required to be exactly rounded.
# Scaling by a power of two is exact wherever the result is representable, so an out-of-range
# exponent is split into steps that are not, and each step multiplies by a value built from its
# bits rather than computed.
ldexp_exact <- function(v, exp) {
  result <- v
  remaining <- exp
  while (remaining > 1023) {
    result <- result * 8.98846567431158e307 # 2^1023
    remaining <- remaining - 1023
  }
  while (remaining < -1022) {
    result <- result * 2.2250738585072014e-308 # 2^-1022
    remaining <- remaining + 1022
  }
  result * 2^remaining
}
