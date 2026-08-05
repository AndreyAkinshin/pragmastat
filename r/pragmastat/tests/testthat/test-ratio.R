test_that("ratio satisfy reference tests", {
  # The only point estimator that keeps a tolerance. ratio is
  # exp(median(log x - log y)): the logarithm is the toolkit's own, but the
  # exponential still comes from the platform, and perturbing it by one ULP (the
  # widest two conforming libm implementations may legitimately differ) moves the
  # result on all 180 probed inputs by one representable value. Genuinely
  # approximate, unlike the estimators that merely select a pairwise element.
  run_reference_tests("ratio", ratio, is_two_sample = TRUE, tolerance = 1e-9)
})
