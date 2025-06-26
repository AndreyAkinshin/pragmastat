test_that("med_ratio satisfy reference tests", {
  run_reference_tests("med-ratio", med_ratio, is_two_sample = TRUE)
})
