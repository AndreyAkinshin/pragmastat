# Every expected value here is computed by hand against the model's own definition: piecewise
# linear interpolation of the sorted sample at anchors (i-1)/(n-1), so the checks double as a
# derivation, not just a pin.

LINEAR_X <- c(1.0, 2.0, 4.0, 8.0)

test_that("quantile_at lands on each observation at its anchor", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$quantile_at(0.0), 1.0)
  expect_identical(dist$quantile_at(1.0 / 3.0), 2.0)
  expect_identical(dist$quantile_at(2.0 / 3.0), 4.0)
  expect_identical(dist$quantile_at(1.0), 8.0)
})

test_that("quantile_at interpolates between two observations", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$quantile_at(0.5), 3.0)
})

test_that("quantile_at clamps outside the unit interval", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$quantile_at(-0.1), 1.0)
  expect_identical(dist$quantile_at(1.1), 8.0)
})

test_that("cumulative_at inverts quantile_at", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$cumulative_at(3.0), 0.5)
  expect_identical(dist$cumulative_at(2.0), 1.0 / 3.0)
  expect_identical(dist$cumulative_at(4.0), 2.0 / 3.0)
})

test_that("cumulative_at is zero below and one above the support", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$cumulative_at(1.0), 0.0)
  expect_identical(dist$cumulative_at(-5.0), 0.0)
  expect_identical(dist$cumulative_at(8.0), 1.0)
  expect_identical(dist$cumulative_at(20.0), 1.0)
})

test_that("density_at is mass over width inside each cell", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$density_at(1.5), 1.0 / 3.0)
  expect_identical(dist$density_at(3.0), 1.0 / 6.0)
  expect_identical(dist$density_at(6.0), 1.0 / 12.0)
  expect_identical(dist$density_at(8.0), 1.0 / 12.0) # The right edge belongs to the last cell.
})

test_that("density_at is zero outside the support", {
  dist <- linear_dist_model(LINEAR_X)
  expect_identical(dist$density_at(0.5), 0.0)
  expect_identical(dist$density_at(9.0), 0.0)
})

test_that("scalar queries propagate NaN", {
  dist <- linear_dist_model(LINEAR_X)
  expect_true(is.nan(dist$quantile_at(NaN)))
  expect_true(is.nan(dist$cumulative_at(NaN)))
  expect_true(is.nan(dist$density_at(NaN)))
})

test_that("landscape agrees with the scalar queries bit for bit", {
  dist <- linear_dist_model(LINEAR_X)
  landscape <- dist$landscape()

  expect_identical(landscape$boundaries, LINEAR_X)
  expect_identical(landscape$cell_count, 3L)
  for (i in seq_along(LINEAR_X)) {
    expect_identical(dist$cumulative_at(LINEAR_X[i]), landscape$cumulative[i])
  }
  for (j in seq_len(landscape$cell_count)) {
    midpoint <- (LINEAR_X[j] + LINEAR_X[j + 1L]) / 2.0
    expect_identical(dist$density_at(midpoint), landscape$densities[j])
  }
})

test_that("sample is quantile_at of a raw drawn cumulance", {
  dist <- linear_dist_model(LINEAR_X)
  drawing <- rng("identity-seed")
  inverting <- rng("identity-seed")
  for (i in seq_len(100)) {
    drawn <- dist$sample(drawing)
    u <- inverting$uniform_float()
    expect_identical(double_bits(drawn), double_bits(dist$quantile_at(u)))
  }
})

test_that("constructor rejects fewer than two observations", {
  err <- tryCatch(linear_dist_model(c(1.0)), error = function(e) e)
  expect_true(is_assumption_error(err))
  expect_identical(err$violation$id, ASSUMPTION_IDS$DOMAIN)
})

test_that("constructor rejects a repeated value", {
  err <- tryCatch(linear_dist_model(c(1.0, 1.0, 2.0)), error = function(e) e)
  expect_true(is_assumption_error(err))
  expect_identical(err$violation$id, ASSUMPTION_IDS$DISTINCTNESS)
})

test_that("quantile rejects a cumulance outside the unit interval", {
  dist <- linear_dist_model(LINEAR_X)
  err <- tryCatch(dist$quantile(c(1.5)), error = function(e) e)
  expect_true(is_assumption_error(err))
  expect_identical(err$violation$id, ASSUMPTION_IDS$CUMULANCE)
})

test_that("an empty query vector is not a violation", {
  dist <- linear_dist_model(LINEAR_X)
  expect_length(dist$quantile(numeric(0)), 0)
  expect_length(dist$cumulative(numeric(0)), 0)
  expect_length(dist$density(numeric(0)), 0)
})

test_that("Sample constructor takes the sample's unit", {
  meter <- MeasurementUnit$new("meter", "Length", "m", "Meter", 1)
  sample <- Sample$new(LINEAR_X, unit = meter)
  dist <- linear_dist_model(sample)
  expect_identical(dist$unit, meter)
})

test_that("Sample constructor refuses a weighted sample", {
  sample <- Sample$new(c(1.0, 2.0), weights = c(1.0, 2.0))
  expect_error(linear_dist_model(sample))
})

test_that("measured quantile returns a Measurement in the model's unit", {
  meter <- MeasurementUnit$new("meter", "Length", "m", "Meter", 1)
  dist <- linear_dist_model(Sample$new(LINEAR_X, unit = meter))
  result <- dist$measured$quantile(0.5)
  expect_identical(result$value, 3.0)
  expect_identical(result$unit, meter)
})

test_that("measured density returns a plain number", {
  meter <- MeasurementUnit$new("meter", "Length", "m", "Meter", 1)
  dist <- linear_dist_model(Sample$new(LINEAR_X, unit = meter))
  result <- dist$measured$density(Measurement$new(3.0, meter))
  expect_identical(result, 1.0 / 6.0)
})
