# Regression: on a sample carrying both +0 and -0, nothing in the data decided
# which of them landed in the position an estimator selects. Comparison cannot
# separate the two, so the sort picked, every port sorts its own way, and the
# ports returned different bits from the same input. Every estimator now
# normalizes the sign away on the way out (see normalize_zero).
#
# -0 == 0 is TRUE, so an equality check proves nothing here. These tests compare
# the 64-bit payload through double_bits() from helper-reference-tests.R.

POSITIVE_ZERO <- "0x0000000000000000"
NEGATIVE_ZERO <- "0x8000000000000000"

test_that("the payload check separates the two zeros", {
  expect_identical(double_bits(0.0), POSITIVE_ZERO)
  expect_identical(double_bits(-0.0), NEGATIVE_ZERO)
})

test_that("center returns positive zero on samples carrying signed zeros", {
  samples <- list(
    c(0.0, -0.0, 0.0, -0.0, 1.0),
    c(-0.0, -0.0),
    c(-0.0, 0.0),
    c(0.0, -0.0),
    c(-0.0, -0.0, -0.0),
    c(-1.0, 1.0),
    c(-2.0, -0.0, 2.0)
  )
  for (x in samples) {
    label <- paste0("c(", paste(sprintf("%.17g", x), collapse = ", "), ")")
    expect_identical(double_bits(center(x)), POSITIVE_ZERO,
      info = paste("center", label)
    )
    expect_identical(double_bits(unwrap_value(center(Sample$new(x)))), POSITIVE_ZERO,
      info = paste("center Sample", label)
    )
  }
})

test_that("center_bounds returns positive zero on both endpoints", {
  x <- c(0.0, -0.0, 0.0, -0.0, 1.0, -1.0)
  raw_bounds <- center_bounds(x, 0.3)
  expect_identical(double_bits(raw_bounds$lower), POSITIVE_ZERO)
  expect_identical(double_bits(raw_bounds$upper), POSITIVE_ZERO)

  sample_bounds <- center_bounds(Sample$new(x), 0.3)
  expect_identical(double_bits(sample_bounds$lower), POSITIVE_ZERO)
  expect_identical(double_bits(sample_bounds$upper), POSITIVE_ZERO)
})

# The samples below each produced a negative zero before the fix, one per
# estimator that can reach a zero at all. spread, avg_spread and their bounds
# are absent because they cannot: spread > 0 is an enforced assumption, and the
# spread bounds are order statistics of absolute differences, which are never
# negative zeros.
test_that("shift and disparity return positive zero", {
  x <- c(1.0, -1.0, -0.0, -0.0, -1.0, 0.0)
  y <- c(-2.0, -0.0, 0.0)
  expect_identical(double_bits(shift(x, y)), POSITIVE_ZERO)
  expect_identical(double_bits(disparity(x, y)), POSITIVE_ZERO)
})

test_that("shift_bounds returns positive zero endpoints", {
  # The single-pair branch: one x against one y, the difference returned as both
  # endpoints without going through the quantile search.
  degenerate <- shift_bounds(c(-0.0), c(0.0), 1.0)
  expect_identical(double_bits(degenerate$lower), POSITIVE_ZERO)
  expect_identical(double_bits(degenerate$upper), POSITIVE_ZERO)

  searched <- shift_bounds(c(-0.0), c(0.0, 2.0, 1.0, -2.0), 0.5)
  expect_identical(double_bits(searched$upper), POSITIVE_ZERO)
})

test_that("center_bounds and disparity_bounds return positive zero endpoints", {
  cb <- center_bounds(c(-0.0, -1.0), 0.5)
  expect_identical(double_bits(cb$upper), POSITIVE_ZERO)

  db <- disparity_bounds(
    c(-0.0, 0.0, 2.0, -2.0, -0.0, 1.0),
    c(0.0, 2.0, 2.0, 2.0, 1.0, 0.0),
    0.9,
    seed = "negative-zero"
  )
  expect_identical(double_bits(db$upper), POSITIVE_ZERO)
})

test_that("negative zeros stay legal in the input", {
  # Only outputs are normalized: a sample may still contain -0, and it still
  # compares equal to +0, so the estimate is unaffected.
  expect_identical(double_bits(center(c(-0.0, 4.0))), double_bits(center(c(0.0, 4.0))))
  expect_identical(double_bits(shift(c(-0.0, 4.0), c(1.0))), double_bits(shift(c(0.0, 4.0), c(1.0))))
})
