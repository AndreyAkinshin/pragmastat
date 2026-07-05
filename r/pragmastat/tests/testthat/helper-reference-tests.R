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

# Extract a plain numeric value from either a raw numeric result or a Measurement.
unwrap_value <- function(result) {
  if (inherits(result, "Measurement")) {
    return(result$value)
  }
  result
}

# Build the list of entry points exercised for a one-/two-sample point estimator.
#
# Each entry is list(name, fn) where fn(...) accepts the same native-vector
# arguments as the public function and returns a plain numeric. We test BOTH:
#   - "raw":    the public native-array API with assume_sorted = FALSE.
#   - "sample": the Sample API (wrap vectors in Sample objects), unwrapping the
#               returned Measurement to a plain numeric.
# This dual-path coverage is what catches Sample-adapter bugs (a past critical
# bug shipped because fixtures only ran through the raw path, not Sample).
make_point_entry_points <- function(estimator_func, is_two_sample,
                                    supports_assume_sorted = TRUE) {
  if (is_two_sample) {
    raw_fn <- if (supports_assume_sorted) {
      function(x, y) estimator_func(x, y, assume_sorted = FALSE)
    } else {
      function(x, y) estimator_func(x, y)
    }
    sample_fn <- function(x, y) {
      unwrap_value(estimator_func(Sample$new(x), Sample$new(y)))
    }
  } else {
    raw_fn <- if (supports_assume_sorted) {
      function(x) estimator_func(x, assume_sorted = FALSE)
    } else {
      function(x) estimator_func(x)
    }
    sample_fn <- function(x) unwrap_value(estimator_func(Sample$new(x)))
  }
  list(
    list(name = "raw", fn = raw_fn, is_sample = FALSE),
    list(name = "sample", fn = sample_fn, is_sample = TRUE)
  )
}

# Build the list of entry points exercised for a bounds estimator.
#
# `bounds_func` is the public function. Each entry point takes the leading
# sample vectors as a list plus a named list of trailing scalar arguments
# (e.g. misrate, seed). The raw entry point passes assume_sorted = FALSE
# (unless `supports_assume_sorted = FALSE`, for functions like
# avg_spread_bounds that take no assume_sorted argument); the Sample entry
# point wraps the leading vectors in Sample objects. Both raw
# list(lower, upper) and the Bounds object expose $lower/$upper, so a single
# accessor works.
make_bounds_entry_points <- function(bounds_func, supports_assume_sorted = TRUE) {
  raw_fn <- function(samples, extras) {
    tail_args <- if (supports_assume_sorted) {
      c(extras, list(assume_sorted = FALSE))
    } else {
      extras
    }
    args <- c(samples, tail_args)
    do.call(bounds_func, args)
  }
  sample_fn <- function(samples, extras) {
    wrapped <- lapply(samples, function(v) Sample$new(v))
    args <- c(wrapped, extras)
    do.call(bounds_func, args)
  }
  list(
    list(name = "raw", fn = raw_fn, is_sample = FALSE),
    list(name = "sample", fn = sample_fn, is_sample = TRUE)
  )
}

# Run bounds reference tests through BOTH the raw native-array entry point and
# the Sample entry point. `dir_name` is the tests/<dir> directory. `n_samples`
# is 1 or 2. `extra_arg_names` names the trailing scalar fixture inputs (in
# order) forwarded to the function (e.g. c("misrate"), c("misrate", "seed")).
run_bounds_reference_tests <- function(dir_name, bounds_func, n_samples,
                                       extra_arg_names = c("misrate"),
                                       tolerance = 1e-9,
                                       supports_assume_sorted = TRUE) {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", dir_name)

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, "No JSON test files found")

  entry_points <- make_bounds_entry_points(bounds_func, supports_assume_sorted)

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    file_label <- basename(json_file)

    if (n_samples == 1) {
      samples <- list(test_case$input$x)
    } else {
      samples <- list(test_case$input$x, test_case$input$y)
    }
    extras <- lapply(extra_arg_names, function(nm) test_case$input[[nm]])
    names(extras) <- extra_arg_names

    for (ep in entry_points) {
      label <- paste0(file_label, " [", ep$name, "]")

      if (!is.null(test_case$expected_error)) {
        cond <- tryCatch(
          {
            ep$fn(samples, extras)
            NULL
          },
          assumption_error = function(e) e
        )

        expect_false(is.null(cond),
          info = paste("Expected assumption_error but none was signaled:", label)
        )

        expect_equal(cond$violation$id, test_case$expected_error$id,
          info = paste("Error id mismatch:", label)
        )

        skip_subject <- ep$is_sample &&
          identical(test_case$expected_error$id, "validity") &&
          identical(test_case$expected_error$subject, "y")

        if (!skip_subject && !is.null(test_case$expected_error$subject)) {
          expect_equal(cond$violation$subject, test_case$expected_error$subject,
            info = paste("Error subject mismatch:", label)
          )
        }
      } else {
        actual_output <- ep$fn(samples, extras)

        expect_equal(actual_output$lower, test_case$output$lower,
          tolerance = tolerance,
          info = paste("Failed for test file:", label, "- lower bound")
        )
        expect_equal(actual_output$upper, test_case$output$upper,
          tolerance = tolerance,
          info = paste("Failed for test file:", label, "- upper bound")
        )
      }
    }
  }
}

# Helper function for running reference tests against JSON data through BOTH the
# raw native-array entry point and the Sample entry point.
run_reference_tests <- function(estimator_name, estimator_func, is_two_sample = FALSE,
                                supports_assume_sorted = TRUE) {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", estimator_name)

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  entry_points <- make_point_entry_points(
    estimator_func, is_two_sample, supports_assume_sorted
  )

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    file_label <- basename(json_file)

    for (ep in entry_points) {
      label <- paste0(file_label, " [", ep$name, "]")
      invoke <- function() {
        if (is_two_sample) {
          ep$fn(test_case$input$x, test_case$input$y)
        } else {
          ep$fn(test_case$input$x)
        }
      }

      if (!is.null(test_case$expected_error)) {
        # Error test case: expect assumption_error with matching violation fields.
        cond <- tryCatch(
          {
            invoke()
            NULL
          },
          assumption_error = function(e) e
        )

        expect_false(is.null(cond),
          info = paste("Expected assumption_error but none was signaled:", label)
        )

        expect_equal(cond$violation$id, test_case$expected_error$id,
          info = paste("Error id mismatch:", label)
        )

        # Sample construction always uses subject "x", so when the expected
        # subject is "y" and the error originates from Sample creation
        # (validity), the subject check would always fail. Skip it in that case.
        skip_subject <- ep$is_sample &&
          identical(test_case$expected_error$id, "validity") &&
          identical(test_case$expected_error$subject, "y")

        if (!skip_subject && !is.null(test_case$expected_error$subject)) {
          expect_equal(cond$violation$subject, test_case$expected_error$subject,
            info = paste("Error subject mismatch:", label)
          )
        }
      } else {
        # Normal test case: compare output.
        actual_output <- invoke()

        expect_equal(actual_output, test_case$output,
          tolerance = 1e-9,
          info = paste("Failed for test file:", label)
        )
      }
    }
  }
}
