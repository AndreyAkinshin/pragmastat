# Asserts that the RAW (native-vector, double misrate) bounds API
# rejects misrate = 2.0, misrate = -0.1, and misrate = NaN with the
# domain/misrate assumption_error (id = "domain", subject = "misrate"), for a
# one-sample (center_bounds) and a two-sample (shift_bounds) estimator.
#
# Uses the raw/double path, NOT a typed Probability path (R has no Probability
# wrapper; the vector API takes a plain double misrate).

expect_misrate_domain_error <- function(expr) {
  err <- tryCatch(
    {
      force(expr)
      NULL
    },
    error = function(e) e
  )
  expect_true(!is.null(err), info = "expected a domain error, got a value")
  expect_true(inherits(err, "assumption_error"))
  expect_equal(err$violation$id, "domain")
  expect_equal(err$violation$subject, "misrate")
}

test_that("raw center_bounds rejects out-of-[0,1] and NaN misrate", {
  x <- c(1.0, 2.0, 3.0, 4.0, 5.0)
  for (bad in list(2.0, -0.1, NaN)) {
    expect_misrate_domain_error(center_bounds(x, misrate = bad))
  }
})

test_that("raw shift_bounds rejects out-of-[0,1] and NaN misrate", {
  x <- c(1.0, 2.0, 3.0, 4.0, 5.0)
  y <- c(2.0, 3.0, 4.0, 5.0, 6.0)
  for (bad in list(2.0, -0.1, NaN)) {
    expect_misrate_domain_error(shift_bounds(x, y, misrate = bad))
  }
})
