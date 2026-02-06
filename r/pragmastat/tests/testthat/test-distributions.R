run_distribution_tests <- function(dist_name, dist_factory) {
  repo_root <- find_repo_root()
  dist_dir <- file.path(repo_root, "tests", "distributions", dist_name)
  json_files <- list.files(dist_dir, pattern = "\\.json$", full.names = TRUE)

  expect_true(length(json_files) > 0, paste("No", dist_name, "distribution test files found"))

  for (json_file in json_files) {
    test_case <- jsonlite::fromJSON(json_file)
    input <- test_case$input
    expected <- test_case$output

    r <- rng(input$seed)
    dist <- dist_factory(input)
    actual <- sapply(seq_len(input$count), function(i) dist$sample(r))

    for (i in seq_along(actual)) {
      expect_equal(actual[i], expected[i], tolerance = 1e-12,
        info = paste("Failed for", basename(json_file), "at index", i))
    }
  }
}

test_that("uniform distribution matches reference tests", {
  run_distribution_tests("uniform", function(input) dist_uniform(input$min, input$max))
})

test_that("additive distribution matches reference tests", {
  run_distribution_tests("additive", function(input) dist_additive(input$mean, input$stdDev))
})

test_that("multiplic distribution matches reference tests", {
  run_distribution_tests("multiplic", function(input) dist_multiplic(input$logMean, input$logStdDev))
})

test_that("exp distribution matches reference tests", {
  run_distribution_tests("exp", function(input) dist_exp(input$rate))
})

test_that("power distribution matches reference tests", {
  run_distribution_tests("power", function(input) dist_power(input$min, input$shape))
})
