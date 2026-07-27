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

      # Bitwise, via expect_exact. Both fields are public values derived by
      # summing the weights, and a sum depends on the order it is taken in:
      # floating-point addition is not associative. A tolerance here would
      # accept an implementation that reduces pairwise or accumulates in
      # extended precision (which is what R's own sum() does on x86-64), and
      # that divergence is exactly what these fields exist to pin.
      #
      # Both are absent from the unweighted fixtures, so each is checked only
      # when the fixture carries it.
      if (!is.null(output$total_weight)) {
        expect_exact(
          s$total_weight, output$total_weight,
          paste(file_label, "total_weight")
        )
      }
      if (!is.null(output$weighted_size)) {
        expect_exact(
          s$weighted_size, output$weighted_size,
          paste(file_label, "weighted_size")
        )
      }
    }
  }
})
