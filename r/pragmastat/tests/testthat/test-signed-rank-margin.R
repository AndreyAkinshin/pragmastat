test_that("signed_rank_margin satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "signed-rank-margin")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    n <- test_case$input$n
    misrate <- test_case$input$misrate

    # Handle error test cases
    if (!is.null(test_case$expected_error)) {
      err <- expect_error(
        signed_rank_margin(n, misrate),
        class = "assumption_error"
      )
      expect_equal(err$violation$id, test_case$expected_error$id,
        info = paste("Failed for test file:", basename(json_file), "- violation id")
      )
      next
    }

    expected_output <- test_case$output

    actual_output <- signed_rank_margin(n, misrate)

    # A count, exact by construction in every port: compare payloads, not a
    # tolerance. `tolerance = 0` used to stand in for that and is not the same
    # predicate.
    expect_exact(actual_output, expected_output, basename(json_file))
  }
})

test_that("the exact branch accumulates in integers, not doubles", {
  # The signed-rank distribution is symmetric, so where max_w is odd the cumulative count at the
  # midpoint is exactly half the total, and `cdf >= p` at p = 1/2 is an exact equality. Accumulated
  # in a double vector it came out a hair below, and this returned 1654 against 1652 everywhere
  # else, in a suite the manifest declares exact. n = 57 is inside the window the shared fixtures
  # skip: they cover 50 and then 64, and above 63 there is no exact branch at all.
  expect_equal(signed_rank_margin(57L, 1), 1652)
  expect_equal(signed_rank_margin(55L, 1), 1540)
  expect_equal(signed_rank_margin(61L, 1), 1890)

  # The counts fit a double; it is the cumulative sum that leaves the exact integer range.
  expect_equal(signed_rank_margin(63L, 0.05), 1444)
  expect_equal(signed_rank_margin(60L, 0.01), 1136)
})
