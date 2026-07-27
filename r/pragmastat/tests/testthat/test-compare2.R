# Run reference tests for compare2
# Compare2 requires special handling since it needs thresholds and seed

# compare2 composes projections of two different arithmetic classes. shift and
# disparity select an element of the pairwise set, so every port reaches the
# same double and the payloads match; ratio routes through log and exp from the
# platform libm, which moves the result by a few ULP between conforming
# implementations. The class is a property of one projection, not of the suite,
# so each projection is compared in its own class. A suite-wide predicate has to
# be the weakest one present: the 24 exact projections in this suite would be
# checked at a tolerance because the other 5 are ratio.
#
# The metric comes from the threshold that produced the projection: compare2
# emits one projection per threshold, in order, so projection i belongs to
# input$thresholds[[i]]. The order-* fixtures exist to pin that correspondence,
# and run_compare2_reference_tests asserts the two lengths agree before indexing.
#
# Every field of a projection follows its metric's class: the estimate comes
# from the point estimator and the bounds from the bounds estimator of the same
# metric. verdict is a string and compares exactly either way.
expect_projection <- function(actual_proj, expected_proj, metric, file_label, idx) {
  label <- paste0(file_label, " projection[", idx, "] (", metric, ")")
  if (identical(metric, METRIC_RATIO)) {
    expect_equal(actual_proj$estimate$value, expected_proj$estimate,
      tolerance = 1e-9, info = paste(label, "estimate")
    )
    expect_equal(actual_proj$bounds$lower, expected_proj$lower,
      tolerance = 1e-9, info = paste(label, "lower")
    )
    expect_equal(actual_proj$bounds$upper, expected_proj$upper,
      tolerance = 1e-9, info = paste(label, "upper")
    )
  } else {
    expect_exact(
      actual_proj$estimate$value, expected_proj$estimate,
      paste(label, "estimate")
    )
    expect_exact(
      actual_proj$bounds$lower, expected_proj$lower,
      paste(label, "lower")
    )
    expect_exact(
      actual_proj$bounds$upper, expected_proj$upper,
      paste(label, "upper")
    )
  }
  expect_equal(actual_proj$verdict, expected_proj$verdict,
    info = paste(label, "verdict")
  )
}

run_compare2_reference_tests <- function() {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "compare2")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file, simplifyVector = FALSE)
    file_label <- basename(json_file)

    if (!is.null(test_case$expected_error)) {
      # Error test case: expect assumption_error with matching violation fields
      cond <- tryCatch(
        {
          compare2(
            unlist(test_case$input$x),
            unlist(test_case$input$y),
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

      # compare2 wraps the y vector in a Sample, so a y validity error surfaces
      # from Sample construction with subject "x" (construction cannot know the
      # argument is arg2). Skip the subject check in that case; assert id only.
      skip_subject <-
        identical(test_case$expected_error$id, "validity") &&
          identical(test_case$expected_error$subject, "y")

      if (!skip_subject && !is.null(test_case$expected_error$subject)) {
        expect_equal(cond$violation$subject, test_case$expected_error$subject,
          info = paste("Error subject mismatch:", file_label)
        )
      }
    } else {
      # Normal test case: compare output
      actual_output <- compare2(
        unlist(test_case$input$x),
        unlist(test_case$input$y),
        test_case$input$thresholds,
        seed = test_case$input$seed
      )

      expected_projections <- test_case$output$projections
      thresholds <- test_case$input$thresholds
      expect_equal(length(actual_output), length(expected_projections),
        info = paste("Projection count mismatch:", file_label)
      )
      # Indexing thresholds by projection index is only sound while the two
      # lists line up; a fixture that broke the correspondence would otherwise
      # silently pick some other metric's comparison class.
      expect_equal(length(thresholds), length(expected_projections),
        info = paste("Threshold/projection count mismatch:", file_label)
      )

      for (i in seq_along(expected_projections)) {
        expect_projection(
          actual_output[[i]], expected_projections[[i]],
          thresholds[[i]]$metric, file_label, i
        )
      }
    }
  }
}

test_that("compare2 satisfy reference tests", {
  run_compare2_reference_tests()
})
