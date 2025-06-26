test_that("med_spread satisfy reference tests", {
  run_reference_tests("med-spread", med_spread, is_two_sample = TRUE)
})
