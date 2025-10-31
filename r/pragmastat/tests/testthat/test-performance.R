library(testthat)
library(pragmastat)

test_that("center performance for n=100000", {
  n <- 100000
  x <- seq_len(n)

  start_time <- Sys.time()
  result <- center(x)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat(sprintf("\nCenter for n=%d: %.6f\n", n, result))
  cat(sprintf("Elapsed time: %.3f seconds\n", elapsed))

  expected <- 50000.5
  expect_equal(result, expected, tolerance = 1e-9)
  expect_lt(elapsed, 5) # Should complete in less than 5 seconds
})

test_that("spread performance for n=100000", {
  n <- 100000
  x <- seq_len(n)

  start_time <- Sys.time()
  result <- spread(x)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat(sprintf("\nSpread for n=%d: %.6f\n", n, result))
  cat(sprintf("Elapsed time: %.3f seconds\n", elapsed))

  expected <- 29290
  expect_equal(result, expected, tolerance = 1e-9)
  expect_lt(elapsed, 5) # Should complete in less than 5 seconds
})

test_that("shift performance for n=m=100000", {
  n <- 100000
  x <- seq_len(n)
  y <- seq_len(n)

  start_time <- Sys.time()
  result <- shift(x, y)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat(sprintf("\nShift for n=m=%d: %.6f\n", n, result))
  cat(sprintf("Elapsed time: %.3f seconds\n", elapsed))

  expected <- 0
  expect_equal(result, expected, tolerance = 1e-9)
  expect_lt(elapsed, 5) # Should complete in less than 5 seconds
})
