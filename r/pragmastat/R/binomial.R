# Exact binomial coefficient C(n, k) via native 64-bit integer arithmetic.
# R's built-in choose() is floating point and inexact for the exact
# distribution branch (n + m < 62), so it disagreed with the go/rust/... ports.
# Delegating to src/binomial.c keeps all seven implementations bit-for-bit
# identical. See min_achievable_misrate_two_sample and pairwise_margin.
exact_binomial <- function(n, k) {
  .Call("binomial_coefficient_c", as.integer(n), as.integer(k), PACKAGE = "pragmastat")
}

# Binomial coefficient C(n, k) in binary64 by the multiplicative recurrence
# C(n, k) = prod_{i=1..k} (n-k+i)/i, used above n + m = 62 where the exact
# integer form overflows 64 bits.
#
# It replaced exp(lchoose(n, k)), for two reasons. The specification defines the
# admissible misrate as `misrate >= 2 / C(n+m, n)`, an exact integer quantity;
# measured against exact big-integer binomials over 4 <= n+m <= 400, lchoose was
# inexact on 99.1% of the 79797 sample-size pairs with a worst relative error of
# 8.3e-14, while this recurrence is inexact on 75.6% with a worst of 2.2e-15.
# And lchoose reached the answer through lgamma and exp, which every language
# takes from a different libm: perturbing those by a single ulp moves the
# computed misrate floor. This form calls nothing, so the same perturbation
# moves none of it, and the other six ports run the identical op sequence.
#
# The loop is written out scalar on purpose. Every step is one multiply and one
# divide in binary64, in that order; a vectorised prod() would be free to
# reassociate and would stop matching the other six ports bit-for-bit.
#
# Normalizing k to the smaller half also makes the function symmetric by
# construction, which matters because the two call sites ask for C(n+m, n) and
# C(n+m, m). Those are the same number, and now they are also the same bits, so
# the comparison at the misrate floor cannot be decided by which of the two
# rounded higher.
binomial_coefficient_float <- function(n, k) {
  n <- as.numeric(n)
  k <- as.numeric(k)
  if (k > n - k) {
    k <- n - k
  }
  acc <- 1.0
  for (i in seq_len(k)) {
    acc <- acc * (n - k + i) / i
  }
  acc
}
