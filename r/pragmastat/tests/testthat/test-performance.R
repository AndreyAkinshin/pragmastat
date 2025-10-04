library(testthat)
library(pragmastat)

# Simple O(n^2) implementations for comparison
center_simple <- function(x) {
  n <- length(x)
  pairwise_averages <- outer(x, x, "+") / 2
  median(pairwise_averages[upper.tri(pairwise_averages, diag = TRUE)])
}

spread_simple <- function(x) {
  n <- length(x)
  if (n == 1) {
    return(0)
  }
  pairwise_diffs <- outer(x, x, "-")
  pairwise_abs_diffs <- abs(pairwise_diffs)
  median(pairwise_abs_diffs[upper.tri(pairwise_abs_diffs, diag = FALSE)])
}

test_that("fast_center matches simple implementation", {
  set.seed(1729)

  for (n in 1:100) {
    for (iter in seq_len(n)) {
      x <- rnorm(n)

      expected <- center_simple(x)
      actual <- center(x)

      expect_equal(actual, expected, tolerance = 1e-9)
    }
  }
})

test_that("fast_spread matches simple implementation", {
  set.seed(1729)

  for (n in 1:100) {
    for (iter in seq_len(n)) {
      x <- rnorm(n)

      expected <- spread_simple(x)
      actual <- spread(x)

      expect_equal(actual, expected, tolerance = 1e-9)
    }
  }
})

test_that("fast_center performance for n=100000", {
  set.seed(1729)
  n <- 100000
  x <- rnorm(n)

  start_time <- Sys.time()
  result <- center(x)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat(sprintf("\nCenter for n=%d: %.6f\n", n, result))
  cat(sprintf("Elapsed time: %.3f seconds\n", elapsed))

  expect_lt(elapsed, 5) # Should complete in less than 5 seconds
})

test_that("fast_spread performance for n=100000", {
  set.seed(1729)
  n <- 100000
  x <- rnorm(n)

  start_time <- Sys.time()
  result <- spread(x)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat(sprintf("\nSpread for n=%d: %.6f\n", n, result))
  cat(sprintf("Elapsed time: %.3f seconds\n", elapsed))

  expect_lt(elapsed, 5) # Should complete in less than 5 seconds
})
