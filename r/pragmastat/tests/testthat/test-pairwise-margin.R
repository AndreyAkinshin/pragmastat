test_that("pairwise_margin satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "pairwise-margin")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    n <- test_case$input$n
    m <- test_case$input$m
    misrate <- test_case$input$misrate

    # Handle error test cases
    if (!is.null(test_case$expected_error)) {
      err <- expect_error(
        pairwise_margin(n, m, misrate),
        class = "assumption_error"
      )
      expect_equal(err$violation$id, test_case$expected_error$id,
        info = paste("Failed for test file:", basename(json_file), "- violation id")
      )
      next
    }

    expected_output <- test_case$output

    actual_output <- pairwise_margin(n, m, misrate)

    expect_equal(actual_output, expected_output,
      tolerance = 0, # Exact integer match
      info = paste("Failed for test file:", basename(json_file))
    )
  }
})
