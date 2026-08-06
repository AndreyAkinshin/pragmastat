test_that("ratio satisfy reference tests", {
  # Bitwise, like every other estimator. ratio is exp(shift(log x, log y)) and
  # both of those functions are now the toolkit's own, so the perturbation
  # experiment moves 0 of 180 probed inputs where it used to move all of them.
  # It was the last suite holding a tolerance for a reason other than randomness.
  run_reference_tests("ratio", ratio, is_two_sample = TRUE)
})
