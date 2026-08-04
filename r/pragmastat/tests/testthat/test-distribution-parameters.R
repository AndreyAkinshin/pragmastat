# The five parametric families refuse malformed parameters at construction. Every parameter has
# to be finite, and every scale or shape parameter has to be positive.
#
# Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
# location constructed a family whose every draw is NaN. And the predicate was written negatively
# (std_dev <= 0), which lets NaN through in the other ports because NaN <= 0 is FALSE. In R the
# same comparison is NA, so `if (NA)` raised base R's "missing value where TRUE/FALSE needed",
# which names neither the package nor the parameter. is.finite() now runs first, and it is FALSE
# for NaN rather than NA.

test_that("dist_uniform refuses a non-finite min", {
  for (min_val in c(NaN, -Inf, Inf)) {
    expect_error(dist_uniform(min_val, 1), "min must be finite", fixed = TRUE)
  }
})

test_that("dist_uniform refuses a non-finite max", {
  for (max_val in c(NaN, -Inf, Inf)) {
    expect_error(dist_uniform(0, max_val), "max must be finite", fixed = TRUE)
  }
})

test_that("dist_uniform refuses an empty interval", {
  expect_error(dist_uniform(1, 1), "min must be less than max", fixed = TRUE)
  expect_error(dist_uniform(2, 1), "min must be less than max", fixed = TRUE)
})

test_that("dist_additive refuses a non-finite mean", {
  for (mean_val in c(NaN, Inf, -Inf)) {
    expect_error(dist_additive(mean_val, 1), "mean must be finite", fixed = TRUE)
  }
})

test_that("dist_additive refuses a malformed std_dev", {
  for (std_dev in c(NaN, Inf, 0, -1)) {
    expect_error(dist_additive(0, std_dev), "std_dev must be finite and positive", fixed = TRUE)
  }
})

test_that("dist_multiplic refuses a non-finite log_mean", {
  for (log_mean in c(NaN, Inf, -Inf)) {
    expect_error(dist_multiplic(log_mean, 1), "log_mean must be finite", fixed = TRUE)
  }
})

test_that("dist_multiplic refuses a malformed log_std_dev", {
  for (log_std_dev in c(NaN, Inf, 0, -1)) {
    expect_error(
      dist_multiplic(0, log_std_dev),
      "log_std_dev must be finite and positive",
      fixed = TRUE
    )
  }
})

test_that("dist_exp refuses a malformed rate", {
  for (rate in c(NaN, Inf, 0, -1)) {
    expect_error(dist_exp(rate), "rate must be finite and positive", fixed = TRUE)
  }
})

test_that("dist_power refuses a malformed min", {
  for (min_val in c(NaN, Inf, 0, -1)) {
    expect_error(dist_power(min_val, 3), "min must be finite and positive", fixed = TRUE)
  }
})

test_that("dist_power refuses a malformed shape", {
  for (shape in c(NaN, Inf, 0, -1)) {
    expect_error(dist_power(1, shape), "shape must be finite and positive", fixed = TRUE)
  }
})

test_that("well-formed families still construct and draw", {
  r <- rng("distribution-parameters")
  families <- list(
    dist_uniform(-2.5, 7.5),
    dist_additive(10, 2),
    dist_multiplic(1, 0.5),
    dist_exp(3),
    dist_power(1, 2)
  )
  for (family in families) {
    expect_true(is.finite(family$sample(r)))
  }
})
