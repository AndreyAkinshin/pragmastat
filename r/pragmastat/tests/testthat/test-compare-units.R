test_that("compare1 supports Measurement thresholds with Sample input", {
  ms <- MeasurementUnit$new("ms", "Time", "ms", "Millisecond", 1000000)
  ns <- MeasurementUnit$new("ns", "Time", "ns", "Nanosecond", 1)

  sx <- Sample$new(1:10, unit = ms)
  thresholds <- list(
    list(metric = "center", value = Measurement$new(3000000, ns), misrate = 0.05)
  )

  result <- compare1(sx, thresholds)

  proj <- result[[1]]
  expect_equal(proj$estimate$value, 5.5)
  expect_equal(proj$estimate$unit$id, "ms")
  expect_equal(proj$bounds$lower, 3.5)
  expect_equal(proj$bounds$upper, 7.5)
  expect_equal(proj$bounds$unit$id, "ms")
  expect_equal(proj$verdict, "greater")
})

test_that("compare2 supports Measurement thresholds with Sample input", {
  ms <- MeasurementUnit$new("ms", "Time", "ms", "Millisecond", 1000000)
  ns <- MeasurementUnit$new("ns", "Time", "ns", "Nanosecond", 1)

  sx <- Sample$new(1:30, unit = ms)
  sy <- Sample$new((21:50) * 1000000, unit = ns, subject = "y")
  thresholds <- list(
    list(metric = "shift", value = Measurement$new(-14, ms), misrate = 0.05)
  )

  result <- compare2(sx, sy, thresholds)

  proj <- result[[1]]
  expect_equal(proj$estimate$value, -20000000)
  expect_equal(proj$estimate$unit$id, "ns")
  expect_equal(proj$bounds$lower, -25000000)
  expect_equal(proj$bounds$upper, -15000000)
  expect_equal(proj$bounds$unit$id, "ns")
  expect_equal(proj$verdict, "less")
})
