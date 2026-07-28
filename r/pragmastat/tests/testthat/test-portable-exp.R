# The scaling step of portable_exp is spelled 2^n because R has no ldexp. The other ports
# name one (Go math.Ldexp, C# Math.ScaleB, Kotlin Math.scalb, Python math.ldexp, Rust a
# bit-pattern constructor) and get exactness from its contract; here "^" is a libm pow call,
# so exactness is a property of the platform rather than of the language. That makes it a
# claim to check rather than one to assume: a single inexact 2^n would move the result of
# every exponential that reduces to that k, and the whole point of this function is that the
# seven implementations return the same bits.
test_that("2^n is exact for every n portable_exp can reach", {
  # k = floor(y/ln2 + 1/2) over the whole band the cutoffs admit, then the two exponents
  # the split scaling forms from it. Derived from the constants rather than written down,
  # so widening a cutoff widens the test with it.
  k <- floor(-745.2 * .INV_LN2 + 0.5):floor(709.79 * .INV_LN2 + 0.5)
  half <- trunc(k / 2)
  ns <- sort(unique(c(half, k - half)))
  expect_equal(range(ns), c(-538, 512))

  # Repeated doubling cannot be wrong: multiplying a double by 2 is exact whenever the
  # result is representable, and every value reached here is.
  by_doubling <- vapply(ns, function(n) {
    v <- 1
    for (i in seq_len(abs(n))) v <- if (n > 0) v * 2 else v / 2
    v
  }, numeric(1))
  as_written <- vapply(ns, function(n) 2^n, numeric(1))

  expect_identical(double_bits(as_written), double_bits(by_doubling))
})

# Bit-identity with the other six is checked by the shared margin fixtures, which is a
# closed loop: it would pass just as happily on six copies of the same wrong reduction.
# This is the open end of it. The polynomial and the range reduction are supposed to
# reproduce the true exponential to a couple of ulp, and R's own exp is an independent
# witness to that.
test_that("portable_exp tracks the platform exponential to about 2 ulp", {
  ys <- seq(-700, 700, by = 0.37)
  actual <- vapply(ys, portable_exp, numeric(1))
  expected <- exp(ys)
  expect_lt(max(abs(actual - expected) / expected), 2^-51)
})

test_that("portable_exp returns the declared values outside the reduction band", {
  expect_identical(portable_exp(NaN), NaN)
  expect_identical(portable_exp(NA_real_), NA_real_)
  expect_identical(portable_exp(710), Inf)
  expect_identical(portable_exp(Inf), Inf)
  expect_identical(portable_exp(-746), 0)
  expect_identical(portable_exp(-Inf), 0)
  expect_identical(portable_exp(0), 1)
})
