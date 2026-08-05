# R is the only one of the seven ports where the argument's type is not fixed by the signature,
# and log_function reads the exponent out of the binary64 payload. Those two facts meet badly:
# writeBin serializes an R integer as an integer, so writeBin(4L, raw(), size = 8) is the byte
# pattern of 4, not of 4.0. The exponent field then reads as zero and the function returns
# -1023*ln2 for every integer, whatever its value.
#
# This is not a hypothetical input. jsonlite types a JSON "4" as integer, so every reference
# fixture that writes a whole number reaches the estimators as integer, and ratio_bounds returned
# 0 and 8.99e307 for bounds whose true values are 0.25 and 4. Nothing warned: writeBin accepted
# the integer and produced eight perfectly valid bytes of the wrong number.
#
# The test asserts the property rather than the coercion, so it still holds if the reduction is
# rewritten to get the exponent some other way.
test_that("log_function reads a whole number the same whether it is typed integer or double", {
  ints <- 1:1000
  expect_identical(typeof(ints), "integer")

  from_integer <- vapply(ints, log_function, numeric(1))
  from_double <- vapply(as.double(ints), log_function, numeric(1))

  expect_exact(from_integer, from_double, "log_function over 1:1000")
})

# The same input reaching the public API the way a fixture delivers it. The test above pins the
# function; this one pins that nothing between the entry point and the reduction restores the
# type by accident, which is what would happen if a caller were coercing on its behalf.
test_that("ratio_bounds agrees with itself on integer and double fixtures", {
  x_int <- 1:5
  y_int <- 2:6
  expect_identical(typeof(x_int), "integer")

  from_integer <- ratio_bounds(x_int, y_int, misrate = 0.05)
  from_double <- ratio_bounds(as.double(x_int), as.double(y_int), misrate = 0.05)

  expect_exact(from_integer$lower, from_double$lower, "ratio_bounds lower")
  expect_exact(from_integer$upper, from_double$upper, "ratio_bounds upper")
})

# The direct cross-language check: five files, 1745 arguments, compared on binary64 payloads.
#
# Every ratio fixture reaches log_function, and every one of them compares with a tolerance,
# because until this branch the platform logarithm decided the last bits. That is not cover: ratio
# is exp(median(log x - log y)), so a last-bit difference in log changes which pairwise element the
# median selects, and a tolerance is exactly what forgives it.
#
# expect_exact, never a tolerance: the manifest declares this suite exact.
test_that("log_function reproduces the shared log-function fixture bit for bit", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "log-function")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, "No JSON test files found")

  # A loader that silently finds nothing passes every assertion inside the loop, so the number of
  # arguments actually compared is asserted rather than assumed.
  checked <- 0L
  denormals_seen <- 0L

  for (json_file in json_files) {
    label <- basename(json_file)
    test_case <- jsonlite::fromJSON(json_file)
    expect_identical(test_case$input$name, "log_function", info = paste(label, "suite name"))

    # as.double is not decoration. jsonlite types a JSON whole number as integer, and the
    # reduction reads the binary64 payload of its argument, so an integer would be read as the
    # wrong number entirely. The library coerces for this reason; the fixture is where the
    # integers come from.
    args <- as.double(test_case$input$arg)
    expected <- as.double(test_case$output)
    expect_identical(length(args), length(expected),
      info = paste(label, "arg/output length")
    )

    actual <- vapply(args, log_function, numeric(1))
    expect_exact(actual, expected, label)

    checked <- checked + length(args)
    denormals_seen <- denormals_seen + sum(args != 0 & abs(args) < 2^-1022)
  }

  expect_identical(checked, 1745L)

  # Unlike the exponential suite, it is the arguments that reach the denormal range here. A reader
  # that flushed them to zero would fail the payload comparison above with a message blaming the
  # port, so the parse is stated on its own.
  expect_gt(denormals_seen, 0L)
})

test_that("log_function returns the declared values at the edges of its domain", {
  expect_identical(log_function(NaN), NaN)
  expect_identical(log_function(NA_real_), NA_real_)
  expect_identical(log_function(-1), NaN)
  expect_exact(log_function(0), -Inf, "log_function(0)")
  expect_exact(log_function(-0), -Inf, "log_function(-0)")
  expect_exact(log_function(Inf), Inf, "log_function(Inf)")
  # Exactly +0, not -0: ratio exponentiates this result, and the two zeros are distinct doubles.
  # expect_exact compares payloads, so it tells them apart where a numeric comparison would not.
  expect_exact(log_function(1), 0, "log_function(1)")
})

# The open end of the loop, in the sense test-exp-function.R uses: the cross-port fixtures would
# pass on seven copies of the same wrong reduction, and R's own log is an independent witness that
# the reduction and the polynomial reproduce the true logarithm.
#
# The two must not be required to agree exactly. IEEE 754 leaves the logarithm free, so a
# conforming platform is allowed to differ in the last bit, and this implementation exists
# precisely because they do. What is asserted is that any disagreement is that small.
#
# The tolerance is mixed rather than relative because log crosses zero: at x near 1 the result is
# near 0, a relative bound there divides by almost nothing and would fail on a correctly rounded
# answer. pmax(abs(expected), 1) reads as relative away from the crossing and absolute across it.
#
# Measured over this grid: 97.664% of the 13443 points are bit-identical to the platform, and the
# largest disagreement is 2.2019e-16, just inside one step at that magnitude. The floor below is
# set well under both so that a real regression trips it and normal libm variation does not.
test_that("log_function tracks the platform logarithm to the last bit", {
  set.seed(42)
  xs <- c(
    as.double(1:2000), 1.3^(-400:400), exp(seq(-700, 700, by = 0.37)),
    10^seq(-300, 300, by = 0.7), runif(3000, 0, 5), runif(3000, 0, 1e6)
  )
  xs <- xs[is.finite(xs) & xs > 0]

  actual <- vapply(xs, log_function, numeric(1))
  expected <- log(xs)

  expect_lt(max(abs(actual - expected) / pmax(abs(expected), 1)), 2^-51)

  identical_bits <- sum(double_bits(actual) == double_bits(expected))
  expect_gt(identical_bits / length(xs), 0.95)
})

# exp and log are each other's inverse and both are ours, so the round trip is a statement about
# this toolkit rather than about the platform.
#
# The bound is per-point, not a constant: exp amplifies a relative error in its argument by
# |log x|, so at the ends of this range a logarithm accurate to half a step comes back through
# exp off by hundreds of times more than at the middle, with nothing wrong anywhere. A flat bound
# would have to be loose enough for the worst point and would then assert nothing at the best one.
# Measured worst case over this grid is 0.216 of the allowance.
test_that("log_function round trips through the toolkit exponential", {
  xs <- 1.3^(-100:100)
  logged <- vapply(xs, log_function, numeric(1))
  back <- vapply(logged, exp_function, numeric(1))

  allowed <- (abs(logged) + 1) * 2 * .Machine$double.eps
  expect_lt(max(abs(back - xs) / xs / allowed), 1)
})
