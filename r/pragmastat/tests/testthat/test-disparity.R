test_that("disparity satisfy reference tests", {
  run_reference_tests("disparity", disparity, is_two_sample = TRUE)
})
