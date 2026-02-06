test_that("rng uniform satisfies reference tests", {
  # Find repository root
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-seed-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform seed test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    count <- test_case$input$count
    expected <- test_case$output

    r <- rng(seed)
    for (i in seq_len(count)) {
      actual <- r$uniform()
      expect_equal(actual, expected[i], tolerance = 1e-15,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("rng uniform_int satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-int-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform int test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    min_val <- test_case$input$min
    max_val <- test_case$input$max
    count <- test_case$input$count
    expected <- test_case$output

    r <- rng(seed)
    for (i in seq_len(count)) {
      actual <- r$uniform_int(min_val, max_val)
      expect_equal(actual, expected[i],
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("rng string seed satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-string-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No string seed test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    count <- test_case$input$count
    expected <- test_case$output

    r <- rng(seed)
    for (i in seq_len(count)) {
      actual <- r$uniform()
      expect_equal(actual, expected[i], tolerance = 1e-15,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("rng uniform_range satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-range-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform range test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    min_val <- test_case$input$min
    max_val <- test_case$input$max
    count <- test_case$input$count
    expected <- test_case$output

    r <- rng(seed)
    for (i in seq_len(count)) {
      actual <- r$uniform_range(min_val, max_val)
      expect_equal(actual, expected[i], tolerance = 1e-12,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("rng uniform_bool satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  rng_dir <- file.path(repo_root, "tests", "rng")
  json_files <- list.files(rng_dir, pattern = "^uniform-bool-seed-.*\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No uniform bool test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    count <- test_case$input$count
    expected <- test_case$output

    r <- rng(seed)
    for (i in seq_len(count)) {
      actual <- r$uniform_bool()
      expect_equal(actual, expected[i],
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("shuffle satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  shuffle_dir <- file.path(repo_root, "tests", "shuffle")
  json_files <- list.files(shuffle_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No shuffle test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    x <- test_case$input$x
    expected <- test_case$output

    r <- rng(seed)
    actual <- r$shuffle(x)

    for (i in seq_along(actual)) {
      expect_equal(actual[i], expected[i], tolerance = 1e-15,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("sample satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  sample_dir <- file.path(repo_root, "tests", "sample")
  json_files <- list.files(sample_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No sample test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    x <- test_case$input$x
    k <- test_case$input$k
    expected <- test_case$output

    r <- rng(seed)
    actual <- r$sample(x, k)

    expect_equal(length(actual), length(expected),
      info = paste("Wrong length for", basename(json_file)))

    for (i in seq_along(actual)) {
      expect_equal(actual[i], expected[i], tolerance = 1e-15,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
})

test_that("sample with negative k throws error", {
  r <- rng("test-sample-validation")
  expect_error(r$sample(1:10, -1), "k must be non-negative")
})

test_that("resample satisfies reference tests", {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "mise.toml"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root; current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  resample_dir <- file.path(repo_root, "tests", "resample")
  json_files <- list.files(resample_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No resample test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    seed <- test_case$input$seed
    x <- test_case$input$x
    k <- test_case$input$k
    expected <- test_case$output

    r <- rng(seed)
    actual <- r$resample(x, k)

    expect_equal(length(actual), length(expected),
      info = paste("Wrong length for", basename(json_file)))

    for (i in seq_along(actual)) {
      expect_equal(actual[i], expected[i], tolerance = 1e-15,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
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
