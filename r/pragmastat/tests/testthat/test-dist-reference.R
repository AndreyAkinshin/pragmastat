# LinearDistModel and NeighborDistModel against their shared dist-linear/dist-neighbor fixtures.
#
# Unlike the five parametric families in test-dist-family.R, this suite's oracle is not one of
# the seven ports: it is an independent binary64 + exact-Fraction reimplementation
# (tests/oracles/dist_reference.py), written directly from the normative algorithm so a bug
# shared by every language cannot pass itself. Every case is compared by binary64 payload,
# matching the suite's declared exact class in tests/manifest.json.

# A fixture's x/y/output array can carry the JSON strings "NaN"/"Infinity"/"-Infinity" in place of
# a number -- JSON has no literal for either. jsonlite's default array simplification recognizes
# neither reliably: mixing "Infinity" into a numeric array forces the whole array through
# as.character, which silently drops precision (empirically, a fixture array containing
# 1.0000000000000002 next to the string "Infinity" round-trips through fromJSON's default
# simplifier as the string "1"). Every JSON file in this suite is therefore parsed with
# simplifyVector = FALSE, which leaves each array element as its own untouched scalar -- a double
# at full precision, or one of the three strings -- and every numeric array is threaded through
# parse_special_doubles below instead of jsonlite's own coercion.
load_dist_fixture <- function(path) {
  jsonlite::fromJSON(path, simplifyVector = FALSE)
}

parse_special_double <- function(v) {
  if (is.character(v)) {
    return(switch(v,
      "NaN" = NaN,
      "Infinity" = Inf,
      "-Infinity" = -Inf,
      stop(paste0("dist fixture: unrecognized string value ", v))
    ))
  }
  as.double(v)
}

parse_special_doubles <- function(lst) {
  if (length(lst) == 0) {
    return(numeric(0))
  }
  vapply(lst, parse_special_double, numeric(1))
}

list_dist_fixtures <- function(dir_name) {
  repo_root <- find_repo_root()
  test_data_dir <- file.path(repo_root, "tests", dir_name)
  json_files <- list.files(test_data_dir, pattern = "\\.json$", full.names = TRUE)
  expect_true(length(json_files) > 0, info = paste("No JSON test files found for", dir_name))
  json_files
}

# Builds the model the suite names, rather than letting the input decide, mirroring build_family
# in test-dist-family.R.
build_dist_model <- function(model, x, assume_sorted) {
  switch(model,
    "linear" = linear_dist_model(x, assume_sorted = assume_sorted),
    "neighbor" = neighbor_dist_model(x, assume_sorted = assume_sorted),
    stop(paste("Unknown dist model", model))
  )
}

run_dist_op <- function(dist, op, y) {
  switch(op,
    "quantile" = dist$quantile(y),
    "cumulative" = dist$cumulative(y),
    "density" = dist$density(y),
    stop(paste("Unknown dist operation", op))
  )
}

# Asserts a caught assumption_error's id and subject against expected_error. Every dist fixture's
# expected_error carries both fields (the generator's emit_error always writes them together), and
# every sibling language's loader for this suite asserts subject unconditionally -- matching that
# here, rather than skipping the check when absent, is what lets a future assumption-site
# regression that keeps the right id but the wrong subject fail here the same way it fails there.
# Only called once the caller has already confirmed a condition was signaled.
expect_matches_expected_error <- function(cond, expected_error, label) {
  expect_identical(cond$violation$id, expected_error$id, info = paste(label, "id"))
  expect_identical(cond$violation$subject, expected_error$subject, info = paste(label, "subject"))
}

# Runs every fixture under dist-<model>/<op>. A case that declares expected_error is checked
# against the first place the model actually raises it: construction, for a validity/domain/
# distinctness violation on x, or the query call, for a validity/cumulance violation on y.
#
# The presence of expected_error in the fixture is checked before anything is invoked, not
# discovered by catching whatever tryCatch happens to produce and looking for expected_error
# afterward -- a build or query that raises unexpectedly on a case with no expected_error must
# surface as a hard failure, not be silently swallowed as "no expectation, so no problem".
run_dist_query_suite <- function(model, op) {
  json_files <- list_dist_fixtures(paste0("dist-", model, "/", op))

  for (json_file in json_files) {
    label <- paste0("dist-", model, "/", op, "/", basename(json_file))
    case <- load_dist_fixture(json_file)

    x <- parse_special_doubles(case$input$x)
    y <- parse_special_doubles(case$input$y)
    assume_sorted <- isTRUE(case$input$assume_sorted)

    if (!is.null(case$expected_error)) {
      dist <- NULL
      build_err <- tryCatch(
        {
          dist <- build_dist_model(model, x, assume_sorted)
          NULL
        },
        assumption_error = function(e) e
      )

      if (!is.null(build_err)) {
        expect_matches_expected_error(build_err, case$expected_error, paste0(label, " (build)"))
        next
      }

      query_err <- tryCatch(
        {
          run_dist_op(dist, op, y)
          NULL
        },
        assumption_error = function(e) e
      )
      expect_false(is.null(query_err),
        info = paste("Expected assumption_error but none was signaled:", label)
      )
      if (!is.null(query_err)) {
        expect_matches_expected_error(query_err, case$expected_error, paste0(label, " (query)"))
      }
      next
    }

    dist <- build_dist_model(model, x, assume_sorted)
    actual <- run_dist_op(dist, op, y)
    expected <- parse_special_doubles(case$output)
    expect_exact(actual, expected, label)
  }
}

# Runs every fixture under dist-<model>/sample. No case here declares an error: an invalid x
# would already be covered by the query suites above, and drawing from a built model cannot fail
# on its own.
run_dist_sample_suite <- function(model) {
  json_files <- list_dist_fixtures(paste0("dist-", model, "/sample"))

  for (json_file in json_files) {
    label <- paste0("dist-", model, "/sample/", basename(json_file))
    case <- load_dist_fixture(json_file)

    x <- parse_special_doubles(case$input$x)
    dist <- build_dist_model(model, x, FALSE)

    drawing <- rng(case$input$seed)
    actual <- dist$samples(drawing, as.integer(case$input$count))

    expected <- parse_special_doubles(case$output)
    expect_exact(actual, expected, label)
  }
}

# Runs every fixture under dist-neighbor/landscape. Every case here builds successfully by
# construction: the generator only emits a landscape for a sample that already passed the query
# suites' validity checks.
run_dist_neighbor_landscape_suite <- function() {
  json_files <- list_dist_fixtures("dist-neighbor/landscape")

  for (json_file in json_files) {
    label <- paste0("dist-neighbor/landscape/", basename(json_file))
    case <- load_dist_fixture(json_file)

    x <- parse_special_doubles(case$input$x)
    dist <- neighbor_dist_model(x, assume_sorted = FALSE)
    landscape <- dist$landscape()

    expected <- case$output
    expect_identical(landscape$scale_index, as.integer(expected$k0), info = paste(label, "k0"))
    expect_identical(landscape$radius, as.integer(expected$radius), info = paste(label, "radius"))
    expect_exact(landscape$boundaries, parse_special_doubles(expected$boundaries), paste(label, "boundaries"))
    expect_exact(landscape$cumulative, parse_special_doubles(expected$cumulative), paste(label, "cumulative"))
    expect_exact(landscape$masses, parse_special_doubles(expected$masses), paste(label, "masses"))
    expect_exact(landscape$densities, parse_special_doubles(expected$densities), paste(label, "densities"))

    # sources is 0-indexed in the fixture, matching the generator's own indexing. R's
    # neighbor_dist_impl already stores it that way (`sources <- (positions - 1L)[surviving]`,
    # subtracting the language's 1-based offset before it ever reaches the landscape), so the two
    # compare directly with no shift.
    expected_sources <- vapply(expected$sources, function(v) as.integer(v), integer(1))
    expect_identical(landscape$sources, expected_sources, info = paste(label, "sources"))
  }
}

test_that("linear_dist_model reproduces its shared quantile/cumulative/density fixtures bit for bit", {
  for (op in c("quantile", "cumulative", "density")) {
    run_dist_query_suite("linear", op)
  }
})

test_that("linear_dist_model reproduces its shared sample fixtures bit for bit", {
  run_dist_sample_suite("linear")
})

test_that("neighbor_dist_model reproduces its shared quantile/cumulative/density fixtures bit for bit", {
  for (op in c("quantile", "cumulative", "density")) {
    run_dist_query_suite("neighbor", op)
  }
})

test_that("neighbor_dist_model reproduces its shared sample fixtures bit for bit", {
  run_dist_sample_suite("neighbor")
})

test_that("neighbor_dist_model reproduces its shared landscape fixtures bit for bit", {
  run_dist_neighbor_landscape_suite()
})
