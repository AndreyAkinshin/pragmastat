test_that("ratio satisfy reference tests", {
  # The only point estimator that keeps a tolerance. ratio is
  # exp(median(log x - log y)): perturbing log and exp by one ULP (the widest
  # two conforming libm implementations may legitimately differ) moves the
  # result on 94% of these fixtures, by up to 16 ULP. Genuinely approximate,
  # unlike the estimators that merely select an element of the pairwise set.
  run_reference_tests("ratio", ratio, is_two_sample = TRUE, tolerance = 1e-9)
})
