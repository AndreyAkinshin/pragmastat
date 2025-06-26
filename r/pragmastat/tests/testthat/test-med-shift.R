test_that("med_shift satisfy reference tests", {
  run_reference_tests("med-shift", med_shift, is_two_sample = TRUE)
})
