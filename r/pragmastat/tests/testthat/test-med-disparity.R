test_that("med_disparity satisfy reference tests", {
  run_reference_tests("med-disparity", med_disparity, is_two_sample = TRUE)
})
