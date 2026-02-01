# Assumption violation conformance tests
#
# These tests verify that assumption violations are reported correctly and
# consistently across all languages. The test data is loaded from shared
# JSON files in tests/assumptions/.

# Helper function to parse special values from JSON
parse_value <- function(v) {
  if (is.numeric(v)) {
    return(v)
  }
  if (is.character(v)) {
    switch(v,
      "NaN" = NaN,
      "Infinity" = Inf,
      "-Infinity" = -Inf,
      stop(paste("Unknown string value:", v))
    )
  } else {
    stop(paste("Unexpected value type:", typeof(v)))
  }
}

# Helper function to parse an array of values from nested list structure
parse_array <- function(arr) {
  if (is.null(arr)) {
    return(numeric(0))
  }
  if (length(arr) == 0) {
    return(numeric(0))
  }
  # Handle list of values (may contain special strings like "NaN")
  sapply(arr, parse_value)
}

# Function dispatch: maps function names to actual implementations
call_function <- function(func_name, x, y) {
  switch(func_name,
    "Center" = center(x),
    "Ratio" = ratio(x, y),
    "RelSpread" = rel_spread(x),
    "Spread" = spread(x),
    "Shift" = shift(x, y),
    "AvgSpread" = avg_spread(x, y),
    "Disparity" = disparity(x, y),
    stop(paste("Unknown function:", func_name))
  )
}

test_that("assumption violations are correct", {
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

  assumptions_dir <- file.path(repo_root, "tests", "assumptions")

  # Load manifest
  manifest <- jsonlite::fromJSON(file.path(assumptions_dir, "manifest.json"))

  total_tests <- 0
  passed_tests <- 0

  # Run each suite
  for (i in seq_len(nrow(manifest$suites))) {
    suite_entry <- manifest$suites[i, ]
    suite_path <- file.path(assumptions_dir, suite_entry$file)
    # Use simplifyVector = FALSE to preserve structure for special values
    suite <- jsonlite::fromJSON(suite_path, simplifyVector = FALSE)

    for (j in seq_along(suite$cases)) {
      test_case <- suite$cases[[j]]
      test_name <- paste0(suite$suite, "/", test_case$name)
      total_tests <- total_tests + 1

      x <- parse_array(test_case$inputs$x)
      y <- parse_array(test_case$inputs$y)

      expected_id <- test_case$expected_violation$id
      expected_subject <- test_case$expected_violation$subject

      # Get function name
      func_name <- test_case[["function"]]

      # Try to call the function and catch assumption violations
      result <- tryCatch({
        call_function(func_name, x, y)
        list(success = TRUE, error = NULL)
      }, assumption_error = function(e) {
        list(success = FALSE, error = e)
      }, error = function(e) {
        # Other errors
        list(success = FALSE, error = e)
      })

      if (result$success) {
        fail(paste0(test_name, ": Expected violation ", expected_id, "(", expected_subject, ") but got success"))
        next
      }

      err <- result$error
      if (!is_assumption_error(err)) {
        fail(paste0(test_name, ": Expected AssumptionError but got ", class(err)[1], ": ", conditionMessage(err)))
        next
      }

      actual_id <- err$violation$id
      actual_subject <- err$violation$subject

      expect_equal(actual_id, expected_id,
        info = paste0(test_name, ": Expected id=", expected_id, ", got ", actual_id))
      expect_equal(actual_subject, expected_subject,
        info = paste0(test_name, ": Expected subject=", expected_subject, ", got ", actual_subject))

      passed_tests <- passed_tests + 1
    }
  }

  message(paste0("\nAssumption Tests: ", passed_tests, "/", total_tests, " passed"))
})
