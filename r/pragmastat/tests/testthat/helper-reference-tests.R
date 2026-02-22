# Find repository root by walking up from working directory until CITATION.cff is found.
find_repo_root <- function() {
  current_dir <- getwd()
  repo_root <- current_dir
  while (!file.exists(file.path(repo_root, "CITATION.cff"))) {
    parent_dir <- dirname(repo_root)
    if (parent_dir == repo_root) {
      stop(paste0("Could not find repository root (CITATION.cff not found); current dir is ", getwd()))
    }
    repo_root <- parent_dir
  }
  repo_root
}

# Helper function for running reference tests against JSON data
run_reference_tests <- function(estimator_name, estimator_func, is_two_sample = FALSE) {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", estimator_name)

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    file_label <- basename(json_file)

    if (!is.null(test_case$expected_error)) {
      # Error test case: expect assumption_error with matching violation fields
      cond <- tryCatch(
        {
          if (is_two_sample) {
            estimator_func(test_case$input$x, test_case$input$y)
          } else {
            estimator_func(test_case$input$x)
          }
          NULL
        },
        assumption_error = function(e) e
      )

      expect_false(is.null(cond),
        info = paste("Expected assumption_error but none was signaled:", file_label)
      )

      expect_equal(cond$violation$id, test_case$expected_error$id,
        info = paste("Error id mismatch:", file_label)
      )

      if (!is.null(test_case$expected_error$subject)) {
        expect_equal(cond$violation$subject, test_case$expected_error$subject,
          info = paste("Error subject mismatch:", file_label)
        )
      }
    } else {
      # Normal test case: compare output
      if (is_two_sample) {
        actual_output <- estimator_func(test_case$input$x, test_case$input$y)
      } else {
        actual_output <- estimator_func(test_case$input$x)
      }

      expect_equal(actual_output, test_case$output,
        tolerance = 1e-9,
        info = paste("Failed for test file:", file_label)
      )
    }
  }
}
