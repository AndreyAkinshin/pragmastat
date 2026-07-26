# The randomization contract is bitwise: rng("experiment-1") must emit the
# identical sequence in every language implementation, and the manual says so.
# These suites therefore compare with expect_exact() (bit patterns) instead of
# expect_equal(), whose default tolerance is 1.5e-8. A tolerant comparison here
# reports a broken contract as a pass: when a backend fuses a multiply into an
# add the last bit moves, and neither 1e-12 nor 1e-15 ever notices.
#
# The estimator suites stay tolerant on purpose: their fixtures encode
# mathematical values that each language is free to reach by a different
# summation order, not a bit pattern the languages agreed to reproduce.

# Draw `count` values from a freshly seeded generator, in order.
rng_draw <- function(r, count, fn, mode = numeric(1)) {
  vapply(seq_len(count), function(i) fn(r), mode)
}

test_that("rng uniform_float satisfies reference tests", {
  repo_root <- find_repo_root()
  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-seed-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform seed test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    r <- rng(test_case$input$seed)
    actual <- rng_draw(r, test_case$input$count, function(r) r$uniform_float())
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("rng uniform_int satisfies reference tests", {
  repo_root <- find_repo_root()
  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-int-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform int test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    min_val <- test_case$input$min
    max_val <- test_case$input$max
    r <- rng(test_case$input$seed)
    actual <- rng_draw(r, test_case$input$count, function(r) r$uniform_int(min_val, max_val))
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("rng string seed satisfies reference tests", {
  repo_root <- find_repo_root()
  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-string-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No string seed test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    r <- rng(test_case$input$seed)
    actual <- rng_draw(r, test_case$input$count, function(r) r$uniform_float())
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("rng uniform_float_range satisfies reference tests", {
  repo_root <- find_repo_root()
  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-range-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform range test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    min_val <- test_case$input$min
    max_val <- test_case$input$max
    r <- rng(test_case$input$seed)
    actual <- rng_draw(r, test_case$input$count, function(r) r$uniform_float_range(min_val, max_val))
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("rng uniform_bool satisfies reference tests", {
  repo_root <- find_repo_root()
  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-bool-seed-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform bool test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    r <- rng(test_case$input$seed)
    actual <- rng_draw(r, test_case$input$count, function(r) r$uniform_bool(), logical(1))
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("shuffle satisfies reference tests", {
  repo_root <- find_repo_root()
  shuffle_dir <- file.path(repo_root, "tests", "shuffle")
  json_files <- list.files(shuffle_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No shuffle test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    r <- rng(test_case$input$seed)
    actual <- r$shuffle(test_case$input$x)
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("sample satisfies reference tests", {
  repo_root <- find_repo_root()
  sample_dir <- file.path(repo_root, "tests", "sample")
  json_files <- list.files(sample_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No sample test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    r <- rng(test_case$input$seed)
    actual <- r$sample(test_case$input$x, test_case$input$k)
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("sample with negative k throws error", {
  r <- rng("test-sample-validation")
  expect_error(r$sample(1:10, -1), "k must be positive")
})

test_that("resample satisfies reference tests", {
  repo_root <- find_repo_root()
  resample_dir <- file.path(repo_root, "tests", "resample")
  json_files <- list.files(resample_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No resample test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    r <- rng(test_case$input$seed)
    actual <- r$resample(test_case$input$x, test_case$input$k)
    expect_exact(actual, test_case$output, basename(json_file))
  }
})

test_that("uniform_int with large range uses correct modulo", {
  # This test documents the behavior for ranges approaching 2^32
  # R's double precision may lose precision for very large moduli
  r <- rng("test-uniform-int-large-range")

  # Range of 2^30 (well within precision)
  range_size <- 2^30
  result <- r$uniform_int(0, range_size)
  expect_true(result >= 0 && result < range_size)

  # Range of 2^31 (still within i32 precision)
  r2 <- rng("test-uniform-int-large-range-2")
  range_size2 <- 2^31
  result2 <- r2$uniform_int(0, range_size2)
  expect_true(result2 >= 0 && result2 < range_size2)
})
