test_that("pairwise_margin satisfies reference tests", {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", "pairwise-margin")

  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, "No JSON test files found")

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)

    n <- test_case$input$n
    m <- test_case$input$m
    misrate <- test_case$input$misrate

    # Handle error test cases
    if (!is.null(test_case$expected_error)) {
      err <- expect_error(
        pairwise_margin(n, m, misrate),
        class = "assumption_error"
      )
      expect_equal(err$violation$id, test_case$expected_error$id,
        info = paste("Failed for test file:", basename(json_file), "- violation id")
      )
      next
    }

    expected_output <- test_case$output

    actual_output <- pairwise_margin(n, m, misrate)

    # The margin is a count: an exact quantity every port must land on, so the
    # comparison is on payloads (expect_exact), not on a tolerance. `tolerance =
    # 0` used to stand in for that and is not the same predicate.
    expect_exact(actual_output, expected_output, basename(json_file))
  }
})

test_that("pairwise_margin uses exact integer binomial (cross-language parity)", {
  # R's built-in choose() is inexact even below 2^53: choose(56, 27) returns
  # 7384942649010078, but the exact value is 7384942649010080. exact_binomial
  # mirrors the go (int64) and rust (u128) ports bit-for-bit.
  #
  # Compared on payloads. Under expect_equal()'s default tolerance of 1.5e-8 the
  # two values above are 2.7e-16 apart in relative terms, so the assertion passed
  # for choose() too and stated nothing at all.
  expect_exact(exact_binomial(56, 27), 7384942649010080, "exact_binomial(56, 27)")
  # The pairwise margin all seven ports agree on for this input is 782.
  expect_exact(pairwise_margin(29, 27, 1.0), 782, "pairwise_margin(29, 27, 1.0)")
})
