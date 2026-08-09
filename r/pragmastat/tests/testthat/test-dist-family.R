# The five parametric families against their shared dist-* fixtures.
#
# Each family answers three questions about itself, and each answer is assembled out of the
# toolkit's own exponential, logarithm, normal cumulative and normal quantile. Nothing on these
# paths reaches a platform transcendental, so the seven ports owe each other identical bits and the
# suites are declared exact in tests/manifest.json: the comparison below is by binary64 payload. A
# tolerance would pass on exactly the divergence these suites exist to catch.
#
# The sibling distributions suite stays approximate, because drawing a sample still goes through
# the platform's log, exp, cos and pow. That is a separate change with its own effect on every
# seeded stream.

# Builds the family the suite names, rather than letting the parameter names decide: two families
# could come to share a spelling, and a fixture read into the wrong family would still produce
# numbers and still be compared.
#
# as.double on every parameter, because jsonlite types a JSON whole number as an R integer, and an
# integer reaching arithmetic that expects a double is a defect this package has already paid for
# once.
build_family <- function(suite, parameters) {
  need <- function(name) {
    if (is.null(parameters[[name]])) {
      stop(paste0(suite, ": fixture has no parameter named ", name))
    }
    as.double(parameters[[name]])
  }
  switch(suite,
    "dist-uniform" = dist_uniform(need("min"), need("max")),
    "dist-exp" = dist_exp(need("rate")),
    "dist-additive" = dist_additive(need("mean"), need("stdDev")),
    "dist-multiplic" = dist_multiplic(need("logMean"), need("logStdDev")),
    "dist-power" = dist_power(need("min"), need("shape")),
    stop(paste("Unknown suite", suite))
  )
}

test_that("the five families reproduce their shared fixtures bit for bit", {
  repo_root <- find_repo_root()
  suites <- c("dist-uniform", "dist-exp", "dist-additive", "dist-multiplic", "dist-power")

  # A loader that silently finds nothing passes every assertion inside the loop, so the number of
  # values actually compared is asserted rather than assumed: for each family, a density and a
  # cumulative at every x and a quantile at every p.
  checked <- 0L

  for (suite in suites) {
    test_data_dir <- file.path(repo_root, "tests", suite)
    json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
    expect_true(length(json_files) > 0, info = paste("No JSON test files found for", suite))

    for (json_file in json_files) {
      label <- paste0(suite, "/", basename(json_file))
      test_case <- jsonlite::fromJSON(json_file)

      x <- as.double(test_case$input$x)
      p <- as.double(test_case$input$p)
      expect_true(length(x) > 0, info = paste(label, "no x values"))
      expect_true(length(p) > 0, info = paste(label, "no p values"))

      expected_density <- as.double(test_case$output$density)
      expected_cumulative <- as.double(test_case$output$cumulative)
      expected_quantile <- as.double(test_case$output$quantile)
      expect_identical(length(expected_density), length(x), info = paste(label, "densities"))
      expect_identical(length(expected_cumulative), length(x), info = paste(label, "cumulatives"))
      expect_identical(length(expected_quantile), length(p), info = paste(label, "quantiles"))

      d <- build_family(suite, test_case$input$parameters)
      expect_exact(vapply(x, d$density_at, numeric(1)), expected_density, paste(label, "density_at"))
      expect_exact(vapply(x, d$cumulative_at, numeric(1)), expected_cumulative, paste(label, "cumulative_at"))
      expect_exact(vapply(p, d$quantile_at, numeric(1)), expected_quantile, paste(label, "quantile_at"))

      checked <- checked + 2L * length(x) + length(p)
    }
  }

  expect_identical(checked, 7800L)
})

# The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an infinity.
#
# Without this table the fixtures are the only check the trio has, and they sample the interior of
# each support through the family's own quantile. That left two defects alive: Uniform reported a
# perfectly ordinary density for NaN in six ports, since neither half of `x < min || x >= max`
# holds for a NaN and control fell straight through, and this port raised "missing value where
# TRUE/FALSE needed" from thirteen of the fifteen functions, since `NaN < 0` is NA in R and
# `if (NA)` is an error. Neither is reachable from a stored fixture.
dist_families <- function() {
  list(
    list(name = "dist-uniform", d = dist_uniform(0, 1), lower = 0, upper = 1),
    list(name = "dist-exp", d = dist_exp(1), lower = 0, upper = Inf),
    list(name = "dist-additive", d = dist_additive(0, 1), lower = -Inf, upper = Inf),
    list(name = "dist-multiplic", d = dist_multiplic(0, 1), lower = 0, upper = Inf),
    list(name = "dist-power", d = dist_power(1, 2), lower = 1, upper = Inf)
  )
}

test_that("NaN propagates through all three functions of every family", {
  for (f in dist_families()) {
    expect_true(is.nan(f$d$density_at(NaN)), info = paste(f$name, "density_at"))
    expect_true(is.nan(f$d$cumulative_at(NaN)), info = paste(f$name, "cumulative_at"))
    expect_true(is.nan(f$d$quantile_at(NaN)), info = paste(f$name, "quantile_at"))
  }
})

test_that("the infinities lie outside every support", {
  for (f in dist_families()) {
    expect_identical(f$d$density_at(-Inf), 0.0, info = f$name)
    expect_identical(f$d$density_at(Inf), 0.0, info = f$name)
    expect_identical(f$d$cumulative_at(-Inf), 0.0, info = f$name)
    expect_identical(f$d$cumulative_at(Inf), 1.0, info = f$name)
  }
})

test_that("a probability outside the unit interval returns the endpoints of the support", {
  for (f in dist_families()) {
    for (p in c(0.0, -0.1, -Inf)) {
      expect_identical(f$d$quantile_at(p), f$lower, info = paste(f$name, p))
    }
    for (p in c(1.0, 1.1, Inf)) {
      expect_identical(f$d$quantile_at(p), f$upper, info = paste(f$name, p))
    }
  }
})

# Every family draws by inverting its own quantile.
#
# A fixture says two numbers agreed on the day it was written. This says they cannot disagree: the
# same seed drives a draw and a direct quantile call, and the two must land on the same bits for
# every family. It is what makes "a distribution is three functions" a statement about the code
# rather than about the documentation, and it would fail the moment a family grew a second
# sampling route.
# lifts is TRUE for the two families whose quantile_at(0) is not a sample.
sampling_identity_families <- function() {
  list(
    list(name = "Uniform", d = dist_uniform(-2.5, 7.5), lifts = FALSE),
    list(name = "Exp", d = dist_exp(0.5), lifts = FALSE),
    list(name = "Additive", d = dist_additive(1.0, 2.0), lifts = TRUE),
    list(name = "Multiplic", d = dist_multiplic(-1.0, 0.5), lifts = TRUE),
    list(name = "Power", d = dist_power(2.0, 3.0), lifts = FALSE)
  )
}

test_that("sampling is the quantile of a drawn cumulance", {
  for (f in sampling_identity_families()) {
    drawing <- rng("identity-seed")
    inverting <- rng("identity-seed")
    for (i in seq_len(1000)) {
      drawn <- f$d$sample(drawing)
      # The same cumulance the draw consumed, taken from a generator on the same seed.
      u <- inverting$uniform_float()
      if (f$lifts && u == 0.0) {
        u <- 5e-324
      }
      inverted <- f$d$quantile_at(u)
      expect_identical(drawn, inverted, info = paste(f$name, "draw", i))
    }
  }
})
