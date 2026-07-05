test_that("avg_spread_bounds satisfies reference tests (raw + Sample)", {
  # avg_spread_bounds is an internal helper with no public assume_sorted API, so
  # supports_assume_sorted = FALSE. Routing through run_bounds_reference_tests
  # exercises BOTH the raw native-array path and the Sample (Measurement/Bounds)
  # adapter path, which the previous bespoke raw-only loop never covered.
  run_bounds_reference_tests(
    "avg-spread-bounds", avg_spread_bounds,
    n_samples = 2, extra_arg_names = c("misrate", "seed"),
    supports_assume_sorted = FALSE
  )
})
