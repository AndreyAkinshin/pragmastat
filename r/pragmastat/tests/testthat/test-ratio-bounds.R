test_that("ratio_bounds satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "ratio-bounds")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    input_x <- test_case$input$x
    input_y <- test_case$input$y
    misrate <- test_case$input$misrate
    expected_lower <- test_case$output$lower
    expected_upper <- test_case$output$upper

    actual_output <- ratio_bounds(input_x, input_y, misrate)

    expect_equal(actual_output$lower, expected_lower,
      tolerance = 1e-10,
      info = paste("Failed for test file:", basename(json_file), "- lower bound")
    )
    expect_equal(actual_output$upper, expected_upper,
      tolerance = 1e-10,
      info = paste("Failed for test file:", basename(json_file), "- upper bound")
    )
  }
})
