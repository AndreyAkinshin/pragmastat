test_that("a MeasurementUnit refuses assignment to any of its fields", {
  # A unit is not a value a caller holds. number_unit is what every Sample, every Measurement and
  # every conversion in the session reaches, so an assignment here rescales the whole session
  # rather than corrupting one object, and nothing revalidates a unit after construction.
  unit <- MeasurementUnit$new("millisecond", "Time", "ms", "Millisecond", 1e6)

  for (field in c("id", "family", "abbreviation", "full_name", "base_units")) {
    expect_error(
      eval(parse(text = paste0("unit$", field, " <- 1"))),
      "read-only",
      info = paste("assigning to", field)
    )
  }
})

test_that("reading a MeasurementUnit still answers", {
  unit <- MeasurementUnit$new("millisecond", "Time", "ms", "Millisecond", 1e6)

  expect_equal(unit$id, "millisecond")
  expect_equal(unit$family, "Time")
  expect_equal(unit$abbreviation, "ms")
  expect_equal(unit$full_name, "Millisecond")
  expect_equal(unit$base_units, 1e6)
})

test_that("the three shared units keep their scale", {
  # The failure this guards is not an error but a wrong number computed later: with the fields
  # writable, number_unit$base_units <- 5 moved conversion_factor(ratio_unit, number_unit) from
  # 1 to 0.2 for everything afterwards.
  expect_error(number_unit$base_units <- 5, "read-only")
  expect_equal(conversion_factor(ratio_unit, number_unit), 1)
  expect_equal(finer(number_unit, ratio_unit)$id, "number")
})
