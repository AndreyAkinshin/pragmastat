# Tests for the raw (vector) API's `assume_sorted` flag.
#
# The vector path is otherwise only exercised with assume_sorted = FALSE; the
# assume_sorted = TRUE branch is reached transitively via Sample. These tests hit
# it directly.
#
# For ORDER-INDEPENDENT estimators (center, spread, shift, ratio, disparity, and
# the center/shift/ratio bounds), calling with assume_sorted = TRUE on a sorted
# copy must equal calling with assume_sorted = FALSE on the unsorted input.
#
# For SHUFFLE-based bounds (spread_bounds, disparity_bounds), the flag only
# affects the sparity check and never the shuffle, so for the SAME input and
# SAME seed the result must be identical regardless of the flag. We compare on
# SORTED input: the shuffle runs on the passed order (identical in both calls),
# and sorted input keeps the order-independent sparity check well-defined.
# Passing assume_sorted = TRUE on UNSORTED input is documented undefined
# behavior (the C sparity kernel relies on the sorted invariant), so we do not
# exercise that misuse here.

tol <- 1e-9

x_unsorted <- c(5, 2, 8, 1, 9, 3, 7, 4)
y_unsorted <- c(6, 2, 9, 1, 4, 7, 3, 8)
x_sorted <- sort(x_unsorted)
y_sorted <- sort(y_unsorted)

test_that("center assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    center(x_sorted, assume_sorted = TRUE),
    center(x_unsorted, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("spread assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    spread(x_sorted, assume_sorted = TRUE),
    spread(x_unsorted, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("shift assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    shift(x_sorted, y_sorted, assume_sorted = TRUE),
    shift(x_unsorted, y_unsorted, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("ratio assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    ratio(x_sorted, y_sorted, assume_sorted = TRUE),
    ratio(x_unsorted, y_unsorted, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("disparity assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    disparity(x_sorted, y_sorted, assume_sorted = TRUE),
    disparity(x_unsorted, y_unsorted, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("center_bounds assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    center_bounds(x_sorted, misrate = 0.3, assume_sorted = TRUE),
    center_bounds(x_unsorted, misrate = 0.3, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("shift_bounds assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    shift_bounds(x_sorted, y_sorted, misrate = 0.3, assume_sorted = TRUE),
    shift_bounds(x_unsorted, y_unsorted, misrate = 0.3, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("ratio_bounds assume_sorted=TRUE on sorted input equals unsorted call", {
  expect_equal(
    ratio_bounds(x_sorted, y_sorted, misrate = 0.3, assume_sorted = TRUE),
    ratio_bounds(x_unsorted, y_unsorted, misrate = 0.3, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("spread_bounds is flag-invariant on the same input and seed", {
  # spread_bounds shuffles internally; the assume_sorted flag only affects the
  # sparity check, never the shuffle. Same input + same seed must be identical.
  expect_equal(
    spread_bounds(x_sorted, misrate = 0.3, seed = 42, assume_sorted = TRUE),
    spread_bounds(x_sorted, misrate = 0.3, seed = 42, assume_sorted = FALSE),
    tolerance = tol
  )
})

test_that("disparity_bounds is flag-invariant on the same input and seed", {
  expect_equal(
    disparity_bounds(x_sorted, y_sorted, misrate = 0.3, seed = 42, assume_sorted = TRUE),
    disparity_bounds(x_sorted, y_sorted, misrate = 0.3, seed = 42, assume_sorted = FALSE),
    tolerance = tol
  )
})

# NOTE on the spread_bounds UNSORTED-inertness property
# -----------------------------------------------------
# There is deliberately NO "spread_bounds inert on UNSORTED input" test (i.e.
#   spread_bounds(UNSORTED, ..., assume_sorted = TRUE) ==
#   spread_bounds(UNSORTED, ..., assume_sorted = FALSE) byte-for-byte).
#
# With assume_sorted = TRUE the sparity check feeds the unsorted vector to the
# sorted-only C kernel spread_impl_c(arr, assume_sorted = 1), which is
# UNDEFINED BEHAVIOR: the selection loop cannot make monotone progress and
# bails out via the iteration cap + stall guard with a deterministic
# "Convergence failure (pathological input)" error (see the pathological-input
# regression test below), while the FALSE call sorts a copy and succeeds — on
# UNSORTED input the two flag values do not agree, so the equality would not
# hold. This matches the other language implementations, which also reject
# this assertion. The legitimate path always feeds a truly-sorted view
# (Sample$sorted_values, or a native vector the caller actually sorted), under
# which the "spread_bounds is flag-invariant on the same input and seed" test
# above already provides the coverage.

# Mutation / aliasing guard. The C kernels sort a copy internally (or, with
# assume_sorted = TRUE on legitimately sorted input, read the buffer strictly
# read-only). Every raw estimator must leave the caller's vector byte-for-byte
# unchanged under BOTH flag values, and a Sample's cached $sorted_values must
# survive estimator calls that alias it read-only.
test_that("raw estimators do not mutate the caller's vector (assume_sorted FALSE/TRUE)", {
  m <- 0.3

  for (flag in c(FALSE, TRUE)) {
    # With assume_sorted = TRUE feed the genuinely sorted vectors (the contract);
    # with FALSE feed the unsorted vectors.
    xv <- if (flag) x_sorted else x_unsorted
    yv <- if (flag) y_sorted else y_unsorted

    # Snapshots must be GENUINE COPIES (`+ 0` forces a fresh allocation).
    # A plain `x_before <- xv` binds the same underlying buffer, so a C kernel
    # writing through REAL() would mutate both bindings at once and the
    # expect_identical() below could never fail.

    # One-sample point + bounds estimators.
    one_sample <- list(
      function(v) center(v, assume_sorted = flag),
      function(v) spread(v, assume_sorted = flag),
      function(v) center_bounds(v, misrate = m, assume_sorted = flag),
      function(v) spread_bounds(v, misrate = m, seed = 42, assume_sorted = flag)
    )
    for (est in one_sample) {
      x_before <- xv + 0
      est(xv)
      expect_identical(xv, x_before)
    }

    # Two-sample point + bounds estimators.
    two_sample <- list(
      function(a, b) shift(a, b, assume_sorted = flag),
      function(a, b) ratio(a, b, assume_sorted = flag),
      function(a, b) disparity(a, b, assume_sorted = flag),
      function(a, b) shift_bounds(a, b, misrate = m, assume_sorted = flag),
      function(a, b) ratio_bounds(a, b, misrate = m, assume_sorted = flag),
      function(a, b) disparity_bounds(a, b, misrate = m, seed = 42, assume_sorted = flag)
    )
    for (est in two_sample) {
      x_before <- xv + 0
      y_before <- yv + 0
      est(xv, yv)
      expect_identical(xv, x_before)
      expect_identical(yv, y_before)
    }
  }
})

# center() on UNSORTED input with assume_sorted = TRUE is documented undefined
# behavior, but it must NEVER wedge the process in an unkillable infinite loop
# inside the C extension. The center_impl selection loop has an iteration cap +
# stall guard that raises a deterministic convergence error (a plain error, NOT
# an assumption_error) on pathological input. This test asserts the call fails
# fast rather than hanging.
test_that("center on unsorted input with assume_sorted=TRUE fails fast, never hangs", {
  # A deliberately unsorted vector chosen to violate the partition invariant
  # that Monahan's algorithm relies on; with assume_sorted = TRUE the loop
  # cannot make monotone progress and must bail out quickly.
  pathological <- c(100, 1, 99, 2, 98, 3, 97, 4, 96, 5)

  elapsed <- system.time(
    err <- tryCatch(
      {
        center(pathological, assume_sorted = TRUE)
        NULL
      },
      error = function(e) e
    )
  )["elapsed"]

  expect_true(!is.null(err), info = "expected a convergence error, got a value")
  expect_match(conditionMessage(err), "Convergence failure \\(pathological input\\)")
  # Must be a plain error, not an assumption_error.
  expect_false(inherits(err, "assumption_error"))
  # Fails fast: well under a second on any machine.
  expect_lt(as.numeric(elapsed), 5)
})

# spread() on UNSORTED input with assume_sorted = TRUE is documented undefined
# behavior, but it must NEVER wedge the process in an unkillable infinite loop
# inside the C extension. The spread_impl selection loop mirrors center_impl's
# iteration cap + stall guard and raises a deterministic convergence error (a
# plain error, NOT an assumption_error) on pathological input. This test
# asserts the call fails fast rather than hanging.
test_that("spread on unsorted input with assume_sorted=TRUE fails fast, never hangs", {
  # A deliberately unsorted vector chosen to violate the sorted invariant that
  # the Shamos selection loop relies on; with assume_sorted = TRUE the loop
  # cannot make monotone progress and must bail out quickly.
  pathological <- c(100, 1, 99, 2, 98, 3, 97, 4, 96, 5)

  elapsed <- system.time(
    err <- tryCatch(
      {
        spread(pathological, assume_sorted = TRUE)
        NULL
      },
      error = function(e) e
    )
  )["elapsed"]

  expect_true(!is.null(err), info = "expected a convergence error, got a value")
  expect_match(conditionMessage(err), "Convergence failure \\(pathological input\\)")
  # Must be a plain error, not an assumption_error.
  expect_false(inherits(err, "assumption_error"))
  # Fails fast: well under a second on any machine.
  expect_lt(as.numeric(elapsed), 5)
})

test_that("Sample $sorted_values is unchanged after estimator calls (read-only aliasing)", {
  sx <- Sample$new(x_unsorted)
  sy <- Sample$new(y_unsorted)

  # Genuine copies (`+ 0`): the active binding hands out the cached buffer
  # itself, so a plain assignment would alias it and an in-place mutation by a
  # C kernel could never be detected by expect_identical().
  sorted_x_before <- sx$sorted_values + 0
  sorted_y_before <- sy$sorted_values + 0

  # First round of calls aliases the cached sorted buffers read-only.
  r1_center <- center(sx)$value
  r1_spread <- spread(sx)$value
  r1_shift <- shift(sx, sy)$value
  r1_ratio <- ratio(sx, sy)$value
  r1_disparity <- disparity(sx, sy)$value

  expect_identical(sx$sorted_values, sorted_x_before)
  expect_identical(sy$sorted_values, sorted_y_before)

  # Two successive calls must return identical results: the cached buffer was not
  # disturbed by the first call (pins the read-only aliasing invariant).
  expect_identical(center(sx)$value, r1_center)
  expect_identical(spread(sx)$value, r1_spread)
  expect_identical(shift(sx, sy)$value, r1_shift)
  expect_identical(ratio(sx, sy)$value, r1_ratio)
  expect_identical(disparity(sx, sy)$value, r1_disparity)

  expect_identical(sx$sorted_values, sorted_x_before)
  expect_identical(sy$sorted_values, sorted_y_before)
})
