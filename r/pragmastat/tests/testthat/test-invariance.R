test_that("invariance tests work correctly", {
  seed <- 1729
  sample_sizes <- c(2, 3, 4, 5, 6, 7, 8, 9, 10)

  # Helper function to perform one-sample tests
  perform_test_one <- function(expr1_func, expr2_func) {
    r <- rng(seed)
    for (n in sample_sizes) {
      x <- vapply(seq_len(n), function(i) r$uniform_float(), numeric(1))
      result1 <- expr1_func(x)
      result2 <- expr2_func(x)
      expect_equal(result1, result2, tolerance = 1e-9)
    }
  }

  # Helper function to perform two-sample tests
  perform_test_two <- function(expr1_func, expr2_func) {
    r <- rng(seed)
    for (n in sample_sizes) {
      x <- vapply(seq_len(n), function(i) r$uniform_float(), numeric(1))
      y <- vapply(seq_len(n), function(i) r$uniform_float(), numeric(1))
      result1 <- expr1_func(x, y)
      result2 <- expr2_func(x, y)
      expect_equal(result1, result2, tolerance = 1e-9)
    }
  }

  # Center tests
  perform_test_one(
    function(x) center(x + 2),
    function(x) center(x) + 2
  )

  perform_test_one(
    function(x) center(2 * x),
    function(x) 2 * center(x)
  )

  perform_test_one(
    function(x) center(-1 * x),
    function(x) -1 * center(x)
  )

  # Spread tests
  perform_test_one(
    function(x) spread(x + 2),
    function(x) spread(x)
  )

  perform_test_one(
    function(x) spread(2 * x),
    function(x) 2 * spread(x)
  )

  perform_test_one(
    function(x) spread(-1 * x),
    function(x) spread(x)
  )

  # Shift tests
  perform_test_two(
    function(x, y) shift(x + 3, y + 2),
    function(x, y) shift(x, y) + 1
  )

  perform_test_two(
    function(x, y) shift(2 * x, 2 * y),
    function(x, y) 2 * shift(x, y)
  )

  perform_test_two(
    function(x, y) shift(x, y),
    function(x, y) -1 * shift(y, x)
  )

  # Ratio tests
  perform_test_two(
    function(x, y) ratio(2 * x, 3 * y),
    function(x, y) (2.0 / 3) * ratio(x, y)
  )

  # AvgSpread tests
  perform_test_one(
    function(x) avg_spread(x, x),
    function(x) spread(x)
  )

  perform_test_two(
    function(x, y) avg_spread(x, y),
    function(x, y) avg_spread(y, x)
  )

  perform_test_one(
    function(x) avg_spread(x, 5 * x),
    function(x) 3 * spread(x)
  )

  perform_test_two(
    function(x, y) avg_spread(-2 * x, -2 * y),
    function(x, y) 2 * avg_spread(x, y)
  )

  # Disparity tests
  perform_test_two(
    function(x, y) disparity(x + 2, y + 2),
    function(x, y) disparity(x, y)
  )

  perform_test_two(
    function(x, y) disparity(2 * x, 2 * y),
    function(x, y) disparity(x, y)
  )

  perform_test_two(
    function(x, y) disparity(-2 * x, -2 * y),
    function(x, y) -1 * disparity(x, y)
  )

  perform_test_two(
    function(x, y) disparity(x, y),
    function(x, y) -1 * disparity(y, x)
  )
})

test_that("shuffle preserves multiset", {
  for (n in c(1, 2, 5, 10, 100)) {
    x <- seq_len(n) - 1 # 0-based to match other languages
    r <- rng(42L)
    shuffled <- r$shuffle(x)
    expect_equal(sort(shuffled), x)
  }
})

test_that("sample returns correct size", {
  x <- 0:9
  for (k in c(1, 3, 5, 10, 15)) {
    r <- rng(42L)
    sampled <- r$sample(x, k)
    expect_equal(length(sampled), min(k, length(x)))
  }
})

test_that("sample elements from source", {
  x <- 0:9
  r <- rng(42L)
  sampled <- r$sample(x, 5)
  for (elem in sampled) {
    expect_true(elem %in% x)
  }
})

test_that("sample preserves order", {
  x <- 0:9
  r <- rng(42L)
  sampled <- r$sample(x, 5)
  if (length(sampled) > 1) {
    for (i in 2:length(sampled)) {
      expect_true(sampled[i] > sampled[i - 1])
    }
  }
})

test_that("sample has no duplicates", {
  for (n in c(2, 3, 5, 10, 20)) {
    source <- seq_len(n)
    for (k in c(1, n %/% 2, n)) {
      r <- rng(42L)
      sampled <- r$sample(source, k)
      expect_equal(length(sampled), length(unique(sampled)),
        info = paste0("Duplicate in sample(n=", n, ", k=", k, ")")
      )
    }
  }
})

test_that("resample with negative k throws error", {
  r <- rng(42L)
  expect_error(r$resample(c(1, 2, 3), -1))
})

test_that("resample elements from source", {
  x <- 0:4
  r <- rng(42L)
  resampled <- r$resample(x, 10)
  for (elem in resampled) {
    expect_true(elem %in% x)
  }
})

test_that("resample k=0 throws error", {
  r <- rng(42L)
  expect_error(r$resample(c(1, 2, 3), 0))
})

test_that("shuffle empty throws error", {
  r <- rng(42L)
  expect_error(r$shuffle(numeric(0)))
})

test_that("sample k=0 throws error", {
  r <- rng(42L)
  expect_error(r$sample(c(1, 2, 3), 0))
})

test_that("sample empty throws error", {
  r <- rng(42L)
  expect_error(r$sample(numeric(0), 1))
})
