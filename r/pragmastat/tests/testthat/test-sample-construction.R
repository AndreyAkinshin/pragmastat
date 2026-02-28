test_that("sample construction satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "sample-construction")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file, simplifyVector = FALSE)
    file_label <- basename(json_file)

    input <- test_case$input

    # Parse values, handling special floats
    values <- vapply(input$values, function(v) {
      if (is.character(v)) {
        switch(v,
          "NaN" = NaN,
          "Infinity" = Inf,
          "-Infinity" = -Inf,
          as.numeric(v)
        )
      } else {
        as.numeric(v)
      }
    }, numeric(1))

    weights <- if (!is.null(input$weights)) as.numeric(input$weights) else NULL

    if (!is.null(test_case$expected_error)) {
      # Error test case
      expect_error(
        {
          if (!is.null(weights)) {
            Sample$new(values, weights = weights)
          } else {
            Sample$new(values)
          }
        },
        info = paste("Expected error but none for:", file_label)
      )
    } else {
      # Valid test case
      output <- test_case$output

      s <- if (!is.null(weights)) {
        Sample$new(values, weights = weights)
      } else {
        Sample$new(values)
      }

      expect_equal(s$size, output$size,
        info = paste("Size mismatch:", file_label)
      )
      expect_equal(s$is_weighted, output$is_weighted,
        info = paste("IsWeighted mismatch:", file_label)
      )
    }
  }
})
