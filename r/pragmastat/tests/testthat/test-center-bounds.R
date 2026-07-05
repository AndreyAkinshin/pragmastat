test_that("center_bounds satisfies reference tests (raw + Sample)", {
  run_bounds_reference_tests(
    "center-bounds", center_bounds,
    n_samples = 1, extra_arg_names = c("misrate")
  )
})
