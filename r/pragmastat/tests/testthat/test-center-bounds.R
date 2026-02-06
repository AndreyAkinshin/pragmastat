test_that("center_bounds satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "center-bounds")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    input_x <- test_case$input$x
    misrate <- test_case$input$misrate

    # Handle error test cases
    if (!is.null(test_case$expected_error)) {
      err <- expect_error(
        center_bounds(input_x, misrate),
        class = "assumption_error"
      )
      expect_equal(err$violation$id, test_case$expected_error$id,
        info = paste("Failed for test file:", basename(json_file), "- violation id"))
      next
    }

    expected_lower <- test_case$output$lower
    expected_upper <- test_case$output$upper

    actual_output <- center_bounds(input_x, misrate)

    expect_equal(actual_output$lower, expected_lower,
      tolerance = 1e-9,
      info = paste("Failed for test file:", basename(json_file), "- lower bound")
    )
    expect_equal(actual_output$upper, expected_upper,
      tolerance = 1e-9,
      info = paste("Failed for test file:", basename(json_file), "- upper bound")
    )
  }
})
