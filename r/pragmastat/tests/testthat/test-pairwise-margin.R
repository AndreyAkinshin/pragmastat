test_that("pairwise_margin satisfies reference tests", {
  # Find repository root by looking for testthat.R
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "testthat.R"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root (testthat.R not found); current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }

  test_data_dir <- file.path(repo_root, "tests", "pairwise-margin")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    n <- test_case$input$n
    m <- test_case$input$m
    misrate <- test_case$input$misrate
    expected_output <- test_case$output

    actual_output <- pairwise_margin(n, m, misrate)

    expect_equal(actual_output, expected_output,
      tolerance = 0, # Exact integer match
      info = paste("Failed for test file:", basename(json_file))
    )
  }
})
