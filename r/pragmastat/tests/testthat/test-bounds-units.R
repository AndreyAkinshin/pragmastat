# Covers the Sample-path bounds unit re-attachment and the contract that the
# RAW (native-vector) bounds path is UNITLESS: it returns a plain
# list(lower, upper) with no unit attached (not a Bounds object).
#
# Sample-path units propagate as:
#   - center_bounds / spread_bounds -> x's unit
#   - shift_bounds                  -> finer(x, y) unit
#   - ratio_bounds                  -> ratio unit
#   - disparity_bounds              -> disparity unit
#
# The center/spread/shift assertions use CUSTOM Time units rather than the
# default number unit: with the default on both sides, a bug that always
# attached number_unit would go unnoticed.

# ms (base_units = 1e6) is finer than sec (base_units = 1e9).
sec_unit <- MeasurementUnit$new("s", "Time", "s", "Second", 1e9)
ms_unit <- MeasurementUnit$new("ms", "Time", "ms", "Millisecond", 1e6)

# Strictly positive so ratio is defined; n = 8 large enough for misrate = 0.3.
xv <- c(5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0)
yv <- c(12.0, 9.0, 15.0, 10.0, 13.0, 11.0, 16.0, 14.0)

test_that("Sample-path center/spread bounds propagate x's custom unit", {
  sx <- Sample$new(xv, unit = sec_unit)
  m <- 0.3

  cb <- center_bounds(sx, misrate = m)
  expect_true(inherits(cb, "Bounds"))
  expect_equal(cb$unit$id, sec_unit$id)

  spb <- spread_bounds(sx, misrate = m, seed = 42)
  expect_true(inherits(spb, "Bounds"))
  expect_equal(spb$unit$id, sec_unit$id)
})

test_that("Sample-path shift bounds attach the finer of the two units", {
  sx <- Sample$new(xv, unit = sec_unit)
  sy <- Sample$new(yv, unit = ms_unit)

  shb <- shift_bounds(sx, sy, misrate = 0.3)
  expect_true(inherits(shb, "Bounds"))
  expect_equal(shb$unit$id, ms_unit$id)
})

test_that("Sample-path ratio/disparity bounds attach their dedicated units", {
  sx <- Sample$new(xv, unit = sec_unit)
  sy <- Sample$new(yv, unit = sec_unit)
  m <- 0.3

  rb <- ratio_bounds(sx, sy, misrate = m)
  expect_true(inherits(rb, "Bounds"))
  expect_equal(rb$unit$id, ratio_unit$id)

  db <- disparity_bounds(sx, sy, misrate = m, seed = 42)
  expect_true(inherits(db, "Bounds"))
  expect_equal(db$unit$id, disparity_unit$id)
})

test_that("raw (vector) bounds are unitless lists", {
  m <- 0.3

  raw_results <- list(
    center_bounds(xv, misrate = m),
    spread_bounds(xv, misrate = m, seed = 42),
    shift_bounds(xv, yv, misrate = m),
    ratio_bounds(xv, yv, misrate = m),
    disparity_bounds(xv, yv, misrate = m, seed = 42)
  )

  for (res in raw_results) {
    # Plain unitless list(lower, upper): no unit, not a Bounds object.
    expect_false(inherits(res, "Bounds"))
    expect_null(res$unit)
    expect_true(is.list(res))
    expect_true(all(c("lower", "upper") %in% names(res)))
  }
})
