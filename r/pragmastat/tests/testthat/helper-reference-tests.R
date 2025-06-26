# Helper function for running reference tests against JSON data
run_reference_tests <- function(estimator_name, estimator_func, is_two_sample = FALSE) {
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

  test_data_dir <- file.path(repo_root, "tests", estimator_name)

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    if (is_two_sample) {
      input_x <- test_case$input$x
      input_y <- test_case$input$y
      expected_output <- test_case$output

      actual_output <- estimator_func(input_x, input_y)
    } else {
      input_x <- test_case$input$x
      expected_output <- test_case$output

      actual_output <- estimator_func(input_x)
    }

    expect_equal(actual_output, expected_output,
                 tolerance = 1e-10,
                 info = paste("Failed for test file:", basename(json_file)))
  }
}
