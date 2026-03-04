# Run reference tests for compare1
# Compare1 requires special handling since it needs thresholds and seed

expect_projection <- function(actual_proj, expected_proj, file_label, idx) {
  label <- paste0(file_label, " projection[", idx, "]")
  expect_equal(actual_proj$estimate$value, expected_proj$estimate,
    tolerance = 1e-9, info = paste(label, "estimate")
  )
  expect_equal(actual_proj$bounds$lower, expected_proj$lower,
    tolerance = 1e-9, info = paste(label, "lower")
  )
  expect_equal(actual_proj$bounds$upper, expected_proj$upper,
    tolerance = 1e-9, info = paste(label, "upper")
  )
  expect_equal(actual_proj$verdict, expected_proj$verdict,
    info = paste(label, "verdict")
  )
}

run_compare1_reference_tests <- function() {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "compare1")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file, simplifyVector = FALSE)
    file_label <- basename(json_file)

    if (!is.null(test_case$expected_error)) {
      # Error test case: expect assumption_error with matching violation fields
      cond <- tryCatch(
        {
          compare1(
            unlist(test_case$input$x),
            test_case$input$thresholds,
            seed = test_case$input$seed
          )
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
      actual_output <- compare1(
        unlist(test_case$input$x),
        test_case$input$thresholds,
        seed = test_case$input$seed
      )

      expected_projections <- test_case$output$projections
      expect_equal(length(actual_output), length(expected_projections),
        info = paste("Projection count mismatch:", file_label)
      )

      for (i in seq_along(expected_projections)) {
        expect_projection(
          actual_output[[i]], expected_projections[[i]], file_label, i
        )
      }
    }
  }
}

test_that("compare1 satisfy reference tests", {
  run_compare1_reference_tests()
})
