test_that("invariance tests work correctly", {
  seed <- 1729
  sample_sizes <- c(2, 3, 4, 5, 6, 7, 8, 9, 10)

  # Helper function to perform one-sample tests
  perform_test_one <- function(expr1_func, expr2_func) {
    set.seed(seed)
    for (n in sample_sizes) {
      x <- runif(n)
      result1 <- expr1_func(x)
      result2 <- expr2_func(x)
      expect_equal(result1, result2, tolerance = 1e-9)
    }
  }

  # Helper function to perform two-sample tests
  perform_test_two <- function(expr1_func, expr2_func) {
    set.seed(seed)
    for (n in sample_sizes) {
      x <- runif(n)
      y <- runif(n)
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

  # Volatility tests
  perform_test_one(
    function(x) volatility(2 * x),
    function(x) volatility(x)
  )

  # Precision tests
  perform_test_one(
    function(x) precision(x + 2),
    function(x) precision(x)
  )

  perform_test_one(
    function(x) precision(2 * x),
    function(x) 2 * precision(x)
  )

  perform_test_one(
    function(x) precision(-2 * x),
    function(x) 2 * precision(x)
  )

  # Shift tests
  perform_test_two(
    function(x, y) med_shift(x + 3, y + 2),
    function(x, y) med_shift(x, y) + 1
  )

  perform_test_two(
    function(x, y) med_shift(2 * x, 2 * y),
    function(x, y) 2 * med_shift(x, y)
  )

  perform_test_two(
    function(x, y) med_shift(x, y),
    function(x, y) -1 * med_shift(y, x)
  )

  # Ratio tests
  perform_test_two(
    function(x, y) med_ratio(2 * x, 3 * y),
    function(x, y) (2.0 / 3) * med_ratio(x, y)
  )

  # AvgSpread tests
  perform_test_one(
    function(x) med_spread(x, x),
    function(x) spread(x)
  )

  perform_test_two(
    function(x, y) med_spread(x, y),
    function(x, y) med_spread(y, x)
  )

  perform_test_one(
    function(x) med_spread(x, 5 * x),
    function(x) 3 * spread(x)
  )

  perform_test_two(
    function(x, y) med_spread(-2 * x, -2 * y),
    function(x, y) 2 * med_spread(x, y)
  )

  # Disparity tests
  perform_test_two(
    function(x, y) med_disparity(x + 2, y + 2),
    function(x, y) med_disparity(x, y)
  )

  perform_test_two(
    function(x, y) med_disparity(2 * x, 2 * y),
    function(x, y) med_disparity(x, y)
  )

  perform_test_two(
    function(x, y) med_disparity(-2 * x, -2 * y),
    function(x, y) -1 * med_disparity(x, y)
  )

  perform_test_two(
    function(x, y) med_disparity(x, y),
    function(x, y) -1 * med_disparity(y, x)
  )
})
