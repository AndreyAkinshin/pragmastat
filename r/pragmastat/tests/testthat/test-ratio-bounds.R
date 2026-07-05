test_that("ratio_bounds satisfies reference tests (raw + Sample)", {
  run_bounds_reference_tests(
    "ratio-bounds", ratio_bounds,
    n_samples = 2, extra_arg_names = c("misrate"),
    tolerance = 1e-10
  )
})

# Error-priority contract: ratio_bounds checks the misrate domain BEFORE the
# positivity of the inputs (ratio_bounds_impl runs the domain checks before
# log_transform). Exercised on both entry points (raw vectors and Samples).

expect_assumption_error <- function(expr, id, subject) {
  err <- tryCatch(
    {
      force(expr)
      NULL
    },
    error = function(e) e
  )
  expect_true(!is.null(err), info = "expected an assumption_error, got a value")
  expect_true(inherits(err, "assumption_error"))
  expect_equal(err$violation$id, id)
  expect_equal(err$violation$subject, subject)
}

test_that("ratio_bounds reports domain(misrate) before positivity(x)", {
  # misrate = -0.1 is invalid (domain) AND x contains a non-positive value
  # (positivity); domain(misrate) must win.
  x <- c(-1.0)
  y <- c(1.0)
  expect_assumption_error(
    ratio_bounds(x, y, misrate = -0.1),
    id = "domain", subject = "misrate"
  )
  expect_assumption_error(
    ratio_bounds(Sample$new(x), Sample$new(y), misrate = -0.1),
    id = "domain", subject = "misrate"
  )
})

test_that("ratio_bounds reports positivity(x) when misrate is valid", {
  x <- c(-1.0, -2.0, -3.0)
  y <- c(1.0, 2.0, 3.0)
  expect_assumption_error(
    ratio_bounds(x, y, misrate = 0.5),
    id = "positivity", subject = "x"
  )
  expect_assumption_error(
    ratio_bounds(Sample$new(x), Sample$new(y), misrate = 0.5),
    id = "positivity", subject = "x"
  )
})
