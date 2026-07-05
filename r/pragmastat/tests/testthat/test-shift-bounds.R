test_that("shift_bounds satisfies reference tests (raw + Sample)", {
  run_bounds_reference_tests(
    "shift-bounds", shift_bounds,
    n_samples = 2, extra_arg_names = c("misrate"),
    tolerance = 1e-10
  )
})
