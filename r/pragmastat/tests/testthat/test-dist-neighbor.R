NEIGHBOR_X <- c(0.0, 1.0, 2.0, 4.0, 8.0)

test_that("support matches the min and max observation", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_identical(dist$support_min, 0.0)
  expect_identical(dist$support_max, 8.0)
})

test_that("cell_count is at most one fewer than the observation count", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_identical(dist$cell_count, length(NEIGHBOR_X))
})

test_that("cumulative_at the support is zero and one exactly", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_identical(dist$cumulative_at(dist$support_min), 0.0)
  expect_identical(dist$cumulative_at(dist$support_max), 1.0)
})

test_that("cell masses sum to exactly one", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  landscape <- dist$landscape()
  expect_identical(sum(landscape$masses), 1.0)
})

test_that("cumulative_at is non-decreasing across the boundaries", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  cumulative <- dist$landscape()$cumulative
  for (i in seq(2, length(cumulative))) {
    expect_true(cumulative[i] > cumulative[i - 1L])
  }
})

test_that("quantile_at inverts cumulative_at at each boundary", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  landscape <- dist$landscape()
  for (i in seq_along(landscape$boundaries)) {
    expect_identical(dist$quantile_at(landscape$cumulative[i]), landscape$boundaries[i])
  }
})

test_that("quantile_at clamps outside the unit interval", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_identical(dist$quantile_at(-0.1), dist$support_min)
  expect_identical(dist$quantile_at(1.1), dist$support_max)
})

test_that("density_at is zero outside the support", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_identical(dist$density_at(dist$support_min - 1.0), 0.0)
  expect_identical(dist$density_at(dist$support_max + 1.0), 0.0)
})

test_that("scalar queries propagate NaN", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_true(is.nan(dist$quantile_at(NaN)))
  expect_true(is.nan(dist$cumulative_at(NaN)))
  expect_true(is.nan(dist$density_at(NaN)))
})

test_that("landscape agrees with the scalar queries bit for bit", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  landscape <- dist$landscape()
  for (i in seq_along(landscape$boundaries)) {
    expect_identical(dist$cumulative_at(landscape$boundaries[i]), landscape$cumulative[i])
  }
  for (j in seq_len(landscape$cell_count)) {
    midpoint <- (landscape$boundaries[j] + landscape$boundaries[j + 1L]) / 2.0
    expect_identical(dist$density_at(midpoint), landscape$densities[j])
  }
})

test_that("landscape exposes scale_index and radius", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  landscape <- dist$landscape()
  # n = 5: dist_ceil_sqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = 3 %/% 2 = 1.
  expect_identical(landscape$scale_index, 3L)
  expect_identical(landscape$radius, 1L)
})

test_that("landscape sources map each cell to its observation", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_identical(dist$landscape()$sources, c(0L, 1L, 2L, 3L, 4L))
})

test_that("a zero-width cell from a collapsed midpoint is dropped", {
  # The center observation sits exactly one ULP from each neighbor on either side. Both adjacent
  # midpoints round to 1.0 itself under round-to-nearest-even (each is exactly halfway between
  # two representable doubles, and 1.0 has the even mantissa), so the center cell's two
  # boundaries collapse onto the same point and the cell is dropped.
  center <- 1.0
  below <- dist_step_bits(center, -1L)
  above <- dist_step_bits(center, 1L)
  values <- c(0.0, below, center, above, 10.0)

  dist <- neighbor_dist_model(values)
  landscape <- dist$landscape()

  expect_identical(landscape$cell_count, length(values) - 1L)
  expect_false(2L %in% landscape$sources)
})

test_that("sample is quantile_at of a raw drawn cumulance", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  drawing <- rng("identity-seed")
  inverting <- rng("identity-seed")
  for (i in seq_len(100)) {
    drawn <- dist$sample(drawing)
    u <- inverting$uniform_float()
    expect_identical(double_bits(drawn), double_bits(dist$quantile_at(u)))
  }
})

test_that("constructor rejects fewer than two observations", {
  err <- tryCatch(neighbor_dist_model(c(1.0)), error = function(e) e)
  expect_true(is_assumption_error(err))
  expect_identical(err$violation$id, ASSUMPTION_IDS$DOMAIN)
})

test_that("constructor rejects a repeated value", {
  err <- tryCatch(neighbor_dist_model(c(1.0, 1.0, 2.0)), error = function(e) e)
  expect_true(is_assumption_error(err))
  expect_identical(err$violation$id, ASSUMPTION_IDS$DISTINCTNESS)
})

test_that("quantile rejects a cumulance outside the unit interval", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  err <- tryCatch(dist$quantile(c(-0.1)), error = function(e) e)
  expect_true(is_assumption_error(err))
  expect_identical(err$violation$id, ASSUMPTION_IDS$CUMULANCE)
})

test_that("an empty query vector is not a violation", {
  dist <- neighbor_dist_model(NEIGHBOR_X)
  expect_length(dist$quantile(numeric(0)), 0)
  expect_length(dist$cumulative(numeric(0)), 0)
  expect_length(dist$density(numeric(0)), 0)
})

test_that("a broken assume_sorted promise terminates", {
  # dist_prepare's distinctness check only looks at ADJACENT elements of the (unsorted, since
  # assume_sorted is a promise it trusts) vector, so a non-adjacent repeat -- x[3] and x[7] both
  # 5.0 here -- slips through it. That repeat then makes one window's span exactly zero while its
  # cell keeps a nonzero width, so the cell's unnormalized mass divides by zero and the total the
  # whole reconstruction divides by is not a usable number.
  scrambled <- c(0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0)
  expect_error(neighbor_dist_model(scrambled, assume_sorted = TRUE))
})

test_that("Sample constructor takes the sample's unit", {
  meter <- MeasurementUnit$new("meter", "Length", "m", "Meter", 1)
  dist <- neighbor_dist_model(Sample$new(NEIGHBOR_X, unit = meter))
  expect_identical(dist$unit, meter)
})

test_that("measured density returns a plain number", {
  meter <- MeasurementUnit$new("meter", "Length", "m", "Meter", 1)
  dist <- neighbor_dist_model(Sample$new(NEIGHBOR_X, unit = meter))
  result <- dist$measured$density(Measurement$new(3.0, meter))
  expect_identical(result, dist$density_at(3.0))
})
