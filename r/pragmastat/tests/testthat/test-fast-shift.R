test_that("fast_shift matches naive on small arrays", {
  set.seed(1729)

  naive_quantiles <- function(x, y, p) {
    diffs <- as.vector(outer(x, y, "-"))
    quantile(diffs, probs = p, type = 7)
  }

  for (m in 1:20) {
    for (n in 1:20) {
      for (iter in 1:5) {
        x <- rnorm(m)
        y <- rnorm(n)
        p <- c(0.0, 0.25, 0.5, 0.75, 1.0)

        actual <- fast_shift(x, y, p)
        expected <- naive_quantiles(x, y, p)

        expect_equal(actual, as.vector(expected),
          tolerance = 1e-9,
          info = paste("m =", m, ", n =", n, ", iter =", iter)
        )
      }
    }
  }
})

test_that("fast_shift matches naive on medium arrays", {
  set.seed(42)

  naive_quantiles <- function(x, y, p) {
    diffs <- as.vector(outer(x, y, "-"))
    quantile(diffs, probs = p, type = 7)
  }

  sizes <- seq(20, 100, by = 10)
  for (size in sizes) {
    for (iter in 1:3) {
      x <- rnorm(size)
      y <- rnorm(size %/% 2)
      p <- c(0.1, 0.5, 0.9)

      actual <- fast_shift(x, y, p)
      expected <- naive_quantiles(x, y, p)

      expect_equal(actual, as.vector(expected),
        tolerance = 1e-9,
        info = paste("size =", size, ", iter =", iter)
      )
    }
  }
})

test_that("fast_shift handles unsorted input correctly", {
  set.seed(999)

  for (trial in 1:50) {
    x_raw <- rnorm(20)
    y_raw <- rnorm(15)
    p <- c(0.25, 0.5, 0.75)

    x_sorted <- sort(x_raw)
    y_sorted <- sort(y_raw)

    x_shuffled <- sample(x_raw)
    y_shuffled <- sample(y_raw)

    result_unsorted <- fast_shift(x_shuffled, y_shuffled, p, assume_sorted = FALSE)
    result_sorted <- fast_shift(x_sorted, y_sorted, p, assume_sorted = TRUE)

    expect_equal(result_unsorted, result_sorted,
      tolerance = 1e-9,
      info = paste("trial =", trial)
    )
  }
})

test_that("fast_shift single element returns constant", {
  set.seed(123)

  for (trial in 1:20) {
    x <- rnorm(1)
    y <- rnorm(1)
    p <- c(0.0, 0.25, 0.5, 0.75, 1.0)

    result <- fast_shift(x, y, p)
    expected <- x[1] - y[1]

    expect_equal(result, rep(expected, length(p)), tolerance = 1e-9)
  }
})

test_that("fast_shift identical arrays median is zero", {
  set.seed(456)

  for (size in 1:30) {
    for (trial in 1:3) {
      x <- rnorm(size)
      p <- 0.5

      result <- fast_shift(x, x, p)

      expect_equal(result, 0.0,
        tolerance = 1e-9,
        info = paste("size =", size, ", trial =", trial)
      )
    }
  }
})

test_that("fast_shift handles asymmetric sizes", {
  set.seed(789)

  naive_quantiles <- function(x, y, p) {
    diffs <- as.vector(outer(x, y, "-"))
    quantile(diffs, probs = p, type = 7)
  }

  configs <- list(
    list(m = 1, n = 100),
    list(m = 100, n = 1),
    list(m = 10, n = 50),
    list(m = 50, n = 10),
    list(m = 5, n = 200)
  )

  for (config in configs) {
    x <- rnorm(config$m)
    y <- rnorm(config$n)
    p <- c(0.0, 0.5, 1.0)

    actual <- fast_shift(x, y, p)
    expected <- naive_quantiles(x, y, p)

    expect_equal(actual, as.vector(expected),
      tolerance = 1e-9,
      info = paste("m =", config$m, ", n =", config$n)
    )
  }
})

test_that("fast_shift extreme quantiles match min/max", {
  set.seed(321)

  for (trial in 1:30) {
    x <- rnorm(10 + trial)
    y <- rnorm(8 + trial %/% 2)
    p <- c(0.0, 1.0)

    result <- fast_shift(x, y, p)

    all_diffs <- as.vector(outer(x, y, "-"))
    min_diff <- min(all_diffs)
    max_diff <- max(all_diffs)

    expect_equal(result[1], min_diff, tolerance = 1e-9)
    expect_equal(result[2], max_diff, tolerance = 1e-9)
  }
})

test_that("fast_shift quantiles are monotonic increasing", {
  set.seed(654)

  for (trial in 1:20) {
    x <- rnorm(25)
    y <- rnorm(20)
    p <- seq(0, 1, by = 0.05)

    result <- fast_shift(x, y, p)

    for (i in 2:length(result)) {
      expect_true(result[i] >= result[i - 1] - 1e-9,
        info = paste("trial =", trial, ", i =", i)
      )
    }
  }
})

test_that("fast_shift handles negative values", {
  set.seed(111)

  naive_quantiles <- function(x, y, p) {
    diffs <- as.vector(outer(x, y, "-"))
    quantile(diffs, probs = p, type = 7)
  }

  for (trial in 1:20) {
    x <- rnorm(15, mean = -50, sd = 10)
    y <- rnorm(12, mean = -50, sd = 10)
    p <- c(0.25, 0.5, 0.75)

    actual <- fast_shift(x, y, p)
    expected <- naive_quantiles(x, y, p)

    expect_equal(actual, as.vector(expected), tolerance = 1e-9)
  }
})

test_that("fast_shift handles duplicates", {
  set.seed(222)

  naive_quantiles <- function(x, y, p) {
    diffs <- as.vector(outer(x, y, "-"))
    quantile(diffs, probs = p, type = 7)
  }

  for (trial in 1:10) {
    x <- round(rnorm(12) * 5) / 5.0
    y <- round(rnorm(10) * 5) / 5.0
    p <- c(0.0, 0.5, 1.0)

    actual <- fast_shift(x, y, p)
    expected <- naive_quantiles(x, y, p)

    expect_equal(actual, as.vector(expected), tolerance = 1e-9)
  }
})

test_that("fast_shift handles very small values", {
  set.seed(333)

  for (trial in 1:10) {
    x <- rnorm(10, mean = 0, sd = 1e-8)
    y <- rnorm(10, mean = 0, sd = 1e-8)
    p <- 0.5

    result <- fast_shift(x, y, p)

    expect_false(is.nan(result))
    expect_false(is.infinite(result))
  }
})

test_that("fast_shift handles large values", {
  set.seed(444)

  for (trial in 1:10) {
    x <- rnorm(10, mean = 1e6, sd = 1e5)
    y <- rnorm(10, mean = 1e6, sd = 1e5)
    p <- 0.5

    result <- fast_shift(x, y, p)

    expect_false(is.nan(result))
    expect_false(is.infinite(result))
  }
})

test_that("fast_shift handles zero spread", {
  x <- rep(5.0, 10)
  y <- rep(2.0, 8)
  p <- c(0.0, 0.25, 0.5, 0.75, 1.0)

  result <- fast_shift(x, y, p)

  expect_equal(result, rep(3.0, length(p)), tolerance = 1e-9)
})

test_that("fast_shift performance test (large arrays)", {
  skip_on_cran()

  set.seed(1729)
  x <- rnorm(500)
  y <- rnorm(500)
  p <- 0.5

  start_time <- Sys.time()
  result <- fast_shift(x, y, p)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat("\n500x500 arrays:", elapsed, "seconds\n")
  expect_true(elapsed < 5)
  expect_length(result, 1)
})

test_that("fast_shift performance test (very large arrays)", {
  skip_on_cran()

  set.seed(9999)
  x <- rnorm(1000)
  y <- rnorm(1000)
  p <- 0.5

  start_time <- Sys.time()
  result <- fast_shift(x, y, p)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat("\n1000x1000 arrays (1M pairs):", elapsed, "seconds\n")
  expect_true(elapsed < 10)
  expect_length(result, 1)
})

test_that("fast_shift performance test (many quantiles)", {
  skip_on_cran()

  set.seed(7777)
  x <- rnorm(200)
  y <- rnorm(200)
  p <- seq(0, 1, by = 0.05)

  start_time <- Sys.time()
  result <- fast_shift(x, y, p)
  elapsed <- as.numeric(difftime(Sys.time(), start_time, units = "secs"))

  cat("\n200x200 arrays, 21 quantiles:", elapsed, "seconds\n")
  expect_true(elapsed < 5)
  expect_equal(length(result), 21)
})

test_that("fast_shift error handling - null inputs", {
  x <- c(1, 2)
  y <- c(3, 4)
  p <- 0.5

  expect_error(fast_shift(NULL, y, p))
  expect_error(fast_shift(x, NULL, p))
  expect_error(fast_shift(x, y, NULL))
})

test_that("fast_shift error handling - empty arrays", {
  valid <- c(1, 2)
  empty <- numeric(0)
  p <- 0.5

  expect_error(fast_shift(empty, valid, p))
  expect_error(fast_shift(valid, empty, p))
})

test_that("fast_shift error handling - invalid probabilities", {
  x <- c(1, 2)
  y <- c(3, 4)

  expect_error(fast_shift(x, y, c(-0.1)))
  expect_error(fast_shift(x, y, c(1.1)))
  expect_error(fast_shift(x, y, c(NaN)))
})

test_that("fast_shift error handling - NaN in data", {
  x_with_nan <- c(1, NaN)
  y_with_nan <- c(3, NaN)
  valid <- c(1, 2)
  p <- 0.5

  expect_error(fast_shift(x_with_nan, valid, p))
  expect_error(fast_shift(valid, y_with_nan, p))
})

test_that("fast_shift empty probabilities returns empty", {
  x <- c(1, 2)
  y <- c(3, 4)
  p <- numeric(0)

  result <- fast_shift(x, y, p)

  expect_equal(length(result), 0)
})

test_that("fast_shift shift invariance - X shift", {
  set.seed(555)

  naive_quantiles <- function(x, y, p) {
    diffs <- as.vector(outer(x, y, "-"))
    quantile(diffs, probs = p, type = 7)
  }

  for (trial in 1:10) {
    x <- rnorm(15)
    y <- rnorm(12)
    p <- c(0.25, 0.5, 0.75)
    shift <- rnorm(1) * 10

    result1 <- fast_shift(x, y, p)
    x_shifted <- x + shift
    result2 <- fast_shift(x_shifted, y, p)

    expect_equal(result2, result1 + shift, tolerance = 1e-9)
  }
})

test_that("fast_shift shift invariance - Y shift", {
  set.seed(666)

  for (trial in 1:10) {
    x <- rnorm(15)
    y <- rnorm(12)
    p <- c(0.25, 0.5, 0.75)
    shift <- rnorm(1) * 10

    result1 <- fast_shift(x, y, p)
    y_shifted <- y + shift
    result2 <- fast_shift(x, y_shifted, p)

    expect_equal(result2, result1 - shift, tolerance = 1e-9)
  }
})

test_that("fast_shift scale invariance", {
  set.seed(777)

  for (trial in 1:10) {
    x <- rnorm(15)
    y <- rnorm(12)
    p <- 0.5
    scale <- 2.0

    result1 <- fast_shift(x, y, p)
    x_scaled <- x * scale
    y_scaled <- y * scale
    result2 <- fast_shift(x_scaled, y_scaled, p)

    expect_equal(result2, result1 * scale, tolerance = 1e-6)
  }
})
