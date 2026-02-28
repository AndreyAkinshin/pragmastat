test_that("unit propagation satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "unit-propagation")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, "No JSON test files found")

  registry <- standard_registry()

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file, simplifyVector = TRUE)
    file_label <- basename(json_file)
    input <- test_case$input

    # Handle weighted-rejected test
    if (!is.null(test_case$expected_error)) {
      x_weights <- as.numeric(input$x_weights)
      sx <- Sample$new(as.numeric(input$x), weights = x_weights)
      expect_error(
        center(sx),
        info = paste("Expected error for weighted sample:", file_label)
      )
      next
    }

    output <- test_case$output
    estimator_name <- input$estimator

    x_unit <- registry$resolve(input$x_unit)
    sx <- Sample$new(as.numeric(input$x), unit = x_unit)

    if (estimator_name == "center") {
      m <- center(sx)
      expect_true(inherits(m, "Measurement"),
        info = paste("Expected Measurement for center:", file_label)
      )
      expect_equal(m$unit$id, output$unit,
        info = paste("Unit mismatch for center:", file_label)
      )
      if (!is.null(output$value)) {
        expect_equal(m$value, output$value, tolerance = 1e-9,
          info = paste("Value mismatch for center:", file_label)
        )
      }
    } else if (estimator_name == "spread") {
      m <- spread(sx)
      expect_true(inherits(m, "Measurement"),
        info = paste("Expected Measurement for spread:", file_label)
      )
      expect_equal(m$unit$id, output$unit,
        info = paste("Unit mismatch for spread:", file_label)
      )
    } else if (estimator_name == "shift") {
      y_unit <- registry$resolve(input$y_unit)
      sy <- Sample$new(as.numeric(input$y), unit = y_unit, subject = "y")
      m <- shift(sx, sy)
      expect_true(inherits(m, "Measurement"),
        info = paste("Expected Measurement for shift:", file_label)
      )
      expect_equal(m$unit$id, output$unit,
        info = paste("Unit mismatch for shift:", file_label)
      )
    } else if (estimator_name == "ratio") {
      y_unit <- registry$resolve(input$y_unit)
      sy <- Sample$new(as.numeric(input$y), unit = y_unit, subject = "y")
      m <- ratio(sx, sy)
      expect_true(inherits(m, "Measurement"),
        info = paste("Expected Measurement for ratio:", file_label)
      )
      expect_equal(m$unit$id, output$unit,
        info = paste("Unit mismatch for ratio:", file_label)
      )
    } else if (estimator_name == "disparity") {
      y_unit <- registry$resolve(input$y_unit)
      sy <- Sample$new(as.numeric(input$y), unit = y_unit, subject = "y")
      m <- disparity(sx, sy)
      expect_true(inherits(m, "Measurement"),
        info = paste("Expected Measurement for disparity:", file_label)
      )
      expect_equal(m$unit$id, output$unit,
        info = paste("Unit mismatch for disparity:", file_label)
      )
    } else {
      fail(paste("Unknown estimator:", estimator_name))
    }
  }
})
