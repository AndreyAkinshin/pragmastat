test_that("spread_bounds satisfies reference tests (raw + Sample)", {
  run_bounds_reference_tests(
    "spread-bounds", spread_bounds,
    n_samples = 1, extra_arg_names = c("misrate", "seed")
  )
})
