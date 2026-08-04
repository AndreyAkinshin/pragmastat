# MeasurementUnit$new refuses malformed unit definitions. A non-positive base_units
# poisons every conversion through the unit and makes finer() prefer it over any
# well-formed unit; an empty family makes two unrelated units compatible, because
# is_compatible() compares families. An empty abbreviation stays legal: the three
# dimensionless units have none.

test_that("MeasurementUnit rejects an empty id", {
  expect_error(
    MeasurementUnit$new("", "Time", "ms", "Millisecond", 1e6),
    "id cannot be empty",
    fixed = TRUE
  )
})

test_that("MeasurementUnit rejects an empty family", {
  expect_error(
    MeasurementUnit$new("ms", "", "ms", "Millisecond", 1e6),
    "family cannot be empty",
    fixed = TRUE
  )
})

test_that("MeasurementUnit rejects zero base_units", {
  expect_error(
    MeasurementUnit$new("ms", "Time", "ms", "Millisecond", 0),
    "base_units must be positive",
    fixed = TRUE
  )
})

test_that("MeasurementUnit rejects negative base_units", {
  expect_error(
    MeasurementUnit$new("ms", "Time", "ms", "Millisecond", -1),
    "base_units must be positive",
    fixed = TRUE
  )
})

test_that("MeasurementUnit accepts a well-formed unit", {
  ms <- MeasurementUnit$new("ms", "Time", "ms", "Millisecond", 1e6)
  expect_equal(ms$id, "ms")
  expect_equal(ms$family, "Time")
  expect_equal(ms$abbreviation, "ms")
  expect_equal(ms$full_name, "Millisecond")
  expect_equal(ms$base_units, 1e6)
})

test_that("the dimensionless units keep their empty abbreviation", {
  units <- list(number = number_unit, ratio = ratio_unit, disparity = disparity_unit)
  for (unit_id in names(units)) {
    unit <- units[[unit_id]]
    expect_equal(unit$id, unit_id)
    expect_equal(unit$abbreviation, "")
    expect_equal(unit$base_units, 1)
  }
})

test_that("a fractional base_units is refused", {
  expect_error(
    MeasurementUnit$new("half", "Time", "h", "Half", 0.5),
    "whole number"
  )
})

test_that("a base_units that is not finite is refused", {
  # The positivity test lets Inf through on its own: Inf <= 0 is FALSE, and round(Inf) is Inf,
  # so the whole-number test would pass it too.
  expect_error(MeasurementUnit$new("inf", "Time", "i", "Inf", Inf), "positive")
  expect_error(MeasurementUnit$new("nan", "Time", "n", "NaN", NaN), "positive")
})

test_that("a whole-valued double is accepted", {
  # 1 and 1.0 are the same scale, and the four integer-typed ports cannot tell them apart.
  expect_equal(MeasurementUnit$new("s", "Time", "s", "Second", 1)$base_units, 1)
})
