# The decomposition in additive_quantile scales by 2^k for ten values of k, and R has no ldexp. The
# other ports build the same factors out of a bit pattern or get them from a named scaling
# operation; here "^" is a libm pow call, so exactness is a property of the platform rather than
# of the language. That makes it a claim to check rather than one to assume: a single inexact
# factor would move e by one and m by a factor of two for every argument reaching that step,
# and the whole point of this function is that the seven implementations return the same bits.

test_that("additive_quantile returns the values the definition fixes", {
  expect_identical(additive_quantile(0.5), 0.0)
  expect_identical(additive_quantile(0.0), -Inf)
  expect_identical(additive_quantile(1.0), Inf)
  expect_identical(additive_quantile(-0.1), -Inf)
  expect_identical(additive_quantile(1.1), Inf)
  expect_true(is.na(additive_quantile(NaN)))
})

test_that("additive_quantile matches the textbook quantiles", {
  cases <- list(
    c(0.75, 0.674489750196082), c(0.9, 1.2815515655446004), c(0.95, 1.6448536269514722),
    c(0.975, 1.959963984540054), c(0.99, 2.3263478740408408), c(0.995, 2.5758293035489004),
    c(0.999, 3.090232306167813)
  )
  for (case in cases) {
    got <- additive_quantile(case[1])
    expect_lt(abs(got - case[2]), 8 * 2.220446049250313e-16 * abs(case[2]))
  }
})

# The central branch consumes q = p - 1/2 directly, so it is exactly odd. The tails are not, and
# asserting it would assert something false: they work from min(p, 1-p), and 1.0 - 0.95 is
# 0.050000000000000044 rather than 0.05. The band edge is excluded for a related reason, since
# 0.5 + 0.425 - 0.5 returns 0.42500000000000004 and lands its point in the other branch.
test_that("the central branch of additive_quantile is odd about the center", {
  for (q in c(0.25, 0.125, 0.0625, 0.4)) {
    expect_identical(0.5 + q - 0.5, q)
    expect_identical(additive_quantile(0.5 + q), -additive_quantile(0.5 - q))
  }
})

test_that("additive_quantile is monotone into the far tail", {
  previous <- -Inf
  p <- 5e-324
  while (p < 0.5) {
    got <- additive_quantile(p)
    expect_gt(got, previous)
    expect_true(is.finite(got))
    previous <- got
    p <- p * 8
  }
  expect_lt(abs(additive_quantile(5e-324) - (-38.467405617144)), 1e-9)
})

test_that("additive_quantile reproduces the shared additive-quantile fixture bit for bit", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "additive-quantile")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, "No JSON test files found")

  # A loader that silently finds nothing passes every assertion inside the loop, so the number
  # of arguments actually compared is asserted rather than assumed. Widening the fixture is
  # meant to move this number with it.
  checked <- 0L
  subnormals_seen <- 0L

  for (json_file in json_files) {
    label <- basename(json_file)
    test_case <- jsonlite::fromJSON(json_file)
    expect_identical(test_case$input$name, "additive_quantile", info = paste(label, "suite name"))

    args <- as.double(test_case$input$arg)
    expected <- as.double(test_case$output)
    expect_identical(length(args), length(expected),
      info = paste(label, "arg/output length")
    )

    actual <- vapply(args, additive_quantile, numeric(1))
    expect_exact(actual, expected, label)

    checked <- checked + length(args)
    subnormals_seen <- subnormals_seen + sum(args < .Machine$double.xmin)
  }

  expect_identical(checked, 547L)

  # Here it is the arguments that run down to the smallest subnormal, which is the part a JSON
  # reader is most likely to mangle. A reader that flushed those to zero would send the function
  # down its zero branch and fail the payload comparison above with a message blaming the port,
  # so the parse is stated on its own here.
  expect_gt(subnormals_seen, 0L)
})
