# Exact binomial coefficient C(n, k) via native 64-bit integer arithmetic.
# R's built-in choose() is floating point and inexact for the exact
# distribution branch (n + m < 62), so it disagreed with the go/rust/... ports.
# Delegating to src/binomial.c keeps all seven implementations bit-for-bit
# identical. See min_achievable_misrate_two_sample and pairwise_margin.
exact_binomial <- function(n, k) {
  .Call("binomial_coefficient_c", as.integer(n), as.integer(k), PACKAGE = "pragmastat")
}
