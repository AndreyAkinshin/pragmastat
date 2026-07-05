# Guard the n == 2 center midpoint order-symmetry.
#
# center() of a 2-element sample is the midpoint of the two values. The midpoint
# must be ORDER-SYMMETRIC: center([a, b]) must bit-equal center([b, a]).
#
# assume_sorted = TRUE is required so the midpoint kernel sees the RAW order (the
# normalizing sort would otherwise hide any asymmetry). The current
# implementation uses the symmetric 0.5*a + 0.5*b form. The old overflow-safe
# a + (b - a)*0.5 form gives -3.4000000000000004 for the REVERSED order, i.e. a
# 1-ULP divergence; this test pins exact (bit-for-bit) equality, not approximate.

test_that("n==2 center midpoint is order-symmetric (exact bit-equality)", {
  forward <- center(c(-5.0, -1.8), assume_sorted = TRUE)
  reversed <- center(c(-1.8, -5.0), assume_sorted = TRUE)

  # Exact, not approximate: the old a+(b-a)*0.5 form diverged by 1 ULP.
  expect_identical(forward, reversed)
  expect_identical(forward, -3.4)
})
