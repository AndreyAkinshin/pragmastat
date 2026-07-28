# The scaling step of portable_exp is spelled 2^n because R has no ldexp. The other ports
# name one (Go math.Ldexp, C# Math.ScaleB, Kotlin Math.scalb, Python math.ldexp, Rust a
# bit-pattern constructor) and get exactness from its contract; here "^" is a libm pow call,
# so exactness is a property of the platform rather than of the language. That makes it a
# claim to check rather than one to assume: a single inexact 2^n would move the result of
# every exponential that reduces to that k, and the whole point of this function is that the
# seven implementations return the same bits.
test_that("2^n is exact for every n portable_exp can reach", {
  # k = floor(y/ln2 + 1/2) over the whole band the cutoffs admit, then the two exponents
  # the split scaling forms from it. Derived from the constants rather than written down,
  # so widening a cutoff widens the test with it.
  k <- floor(-745.2 * .INV_LN2 + 0.5):floor(709.79 * .INV_LN2 + 0.5)
  half <- trunc(k / 2)
  ns <- sort(unique(c(half, k - half)))
  expect_equal(range(ns), c(-538, 512))

  # Repeated doubling cannot be wrong: multiplying a double by 2 is exact whenever the
  # result is representable, and every value reached here is.
  by_doubling <- vapply(ns, function(n) {
    v <- 1
    for (i in seq_len(abs(n))) v <- if (n > 0) v * 2 else v / 2
    v
  }, numeric(1))
  as_written <- vapply(ns, function(n) 2^n, numeric(1))

  expect_identical(double_bits(as_written), double_bits(by_doubling))
})

# The direct cross-language check: four files, 1032 arguments, compared on binary64 payloads.
#
# Before this suite existed the seven ports met only through the margin fixtures, which reach
# the exponential wherever an Edgeworth expansion happens to look and nowhere else. The
# reduction endpoints, the underflow cutoff and the denormal tail went unwitnessed, and those
# are where implementations of an exponential actually part company.
#
# expect_exact, never a tolerance: the manifest declares this suite exact, and a tolerance here
# passes on precisely the divergence the suite exists to catch.
test_that("portable_exp reproduces the shared portable-exp fixture bit for bit", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "portable-exp")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, "No JSON test files found")

  # A loader that silently finds nothing passes every assertion inside the loop, so the number
  # of arguments actually compared is asserted rather than assumed. Widening the fixture is
  # meant to move this number with it.
  checked <- 0L
  denormals_seen <- 0L

  for (json_file in json_files) {
    label <- basename(json_file)
    test_case <- jsonlite::fromJSON(json_file)
    expect_identical(test_case$input$name, "portable_exp", info = paste(label, "suite name"))

    args <- as.double(test_case$input$arg)
    expected <- as.double(test_case$output)
    expect_identical(length(args), length(expected),
      info = paste(label, "arg/output length")
    )

    # Only the outputs are compared. JSON does not distinguish -0 from 0, so the two signed
    # zeros in boundaries.json both arrive as +0; both exponentiate to 1, so nothing is lost,
    # and comparing the inputs would turn a property of the format into a failure.
    actual <- vapply(args, portable_exp, numeric(1))
    expect_exact(actual, expected, label)

    checked <- checked + length(args)
    denormals_seen <- denormals_seen +
      sum(expected != 0 & abs(expected) < 2^-1022)
  }

  expect_identical(checked, 1032L)

  # The fixture runs down to the smallest denormal, which is the part a JSON reader is most
  # likely to mangle. A reader that flushed those to zero would fail the payload comparison
  # above with a message blaming the port, so the parse is stated on its own here.
  expect_gt(denormals_seen, 0L)
})

# Bit-identity with the other six is checked by the fixture above, which is a closed loop:
# it would pass just as happily on seven copies of the same wrong reduction. This is the
# open end of it. The polynomial and the range reduction are supposed to reproduce the true
# exponential to a couple of ulp, and R's own exp is an independent witness to that.
test_that("portable_exp tracks the platform exponential to about 2 ulp", {
  ys <- seq(-700, 700, by = 0.37)
  actual <- vapply(ys, portable_exp, numeric(1))
  expected <- exp(ys)
  expect_lt(max(abs(actual - expected) / expected), 2^-51)
})

test_that("portable_exp returns the declared values outside the reduction band", {
  expect_identical(portable_exp(NaN), NaN)
  expect_identical(portable_exp(NA_real_), NA_real_)
  expect_identical(portable_exp(710), Inf)
  expect_identical(portable_exp(Inf), Inf)
  expect_identical(portable_exp(-746), 0)
  expect_identical(portable_exp(-Inf), 0)
  expect_identical(portable_exp(0), 1)
})
