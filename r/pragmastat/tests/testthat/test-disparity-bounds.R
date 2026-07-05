test_that("disparity_bounds satisfies reference tests (raw + Sample)", {
  run_bounds_reference_tests(
    "disparity-bounds", disparity_bounds,
    n_samples = 2, extra_arg_names = c("misrate", "seed")
  )
})
