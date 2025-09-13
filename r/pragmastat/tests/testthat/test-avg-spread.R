test_that("avg_spread satisfy reference tests", {
  run_reference_tests("avg-spread", avg_spread, is_two_sample = TRUE)
})