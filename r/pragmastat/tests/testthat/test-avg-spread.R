test_that("avg_spread satisfy reference tests", {
  # avg_spread is an internal helper (no public raw assume_sorted API), but we
  # still exercise both the vector and the Sample entry points.
  run_reference_tests(
    "avg-spread", avg_spread,
    is_two_sample = TRUE, supports_assume_sorted = FALSE
  )
})
