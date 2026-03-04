# Compare1 and Compare2: confirmatory analysis for one-sample and two-sample estimators.
#
# These high-level APIs compare estimates (Center, Spread, Shift, Ratio, Disparity)
# against practical thresholds and return verdicts (Less, Greater, or Inconclusive).

# Metric constants
#' @export
METRIC_CENTER <- "center"
#' @export
METRIC_SPREAD <- "spread"
#' @export
METRIC_SHIFT <- "shift"
#' @export
METRIC_RATIO <- "ratio"
#' @export
METRIC_DISPARITY <- "disparity"

# Verdict constants
#' @export
VERDICT_LESS <- "less"
#' @export
VERDICT_GREATER <- "greater"
#' @export
VERDICT_INCONCLUSIVE <- "inconclusive"

# Compare1 metric specifications (internal)
compare1_specs <- function() {
  list(
    center = list(
      estimate = function(x) center(x),
      bounds = function(x, misrate) center_bounds(x, misrate),
      has_seed = FALSE
    ),
    spread = list(
      estimate = function(x) spread(x),
      bounds = function(x, misrate, seed = NULL) spread_bounds(x, misrate, seed),
      has_seed = TRUE
    )
  )
}

# Compare2 metric specifications (internal)
compare2_specs <- function() {
  list(
    shift = list(
      estimate = function(x, y) shift(x, y),
      bounds = function(x, y, misrate) shift_bounds(x, y, misrate),
      has_seed = FALSE
    ),
    ratio = list(
      estimate = function(x, y) ratio(x, y),
      bounds = function(x, y, misrate) ratio_bounds(x, y, misrate),
      has_seed = FALSE
    ),
    disparity = list(
      estimate = function(x, y) disparity(x, y),
      bounds = function(x, y, misrate, seed = NULL) disparity_bounds(x, y, misrate, seed),
      has_seed = TRUE
    )
  )
}

# Compute verdict by comparing bounds against threshold value
compute_verdict <- function(bounds, threshold_value) {
  if (bounds$lower > threshold_value) {
    return(VERDICT_GREATER)
  } else if (bounds$upper < threshold_value) {
    return(VERDICT_LESS)
  } else {
    return(VERDICT_INCONCLUSIVE)
  }
}

as_compare_sample <- function(x, subject) {
  if (inherits(x, "Sample")) {
    return(x$with_subject(subject))
  }
  Sample$new(as.double(x), subject = subject)
}

threshold_numeric <- function(value) {
  if (inherits(value, "Measurement")) {
    if (!is.finite(value$value)) {
      stop("threshold value must be finite")
    }
    return(value$value)
  }
  if (!is.numeric(value) || length(value) != 1 || !is.finite(value)) {
    stop("threshold value must be finite")
  }
  as.double(value)
}

normalize_linear_threshold <- function(value, target_unit) {
  if (inherits(value, "Measurement")) {
    if (!value$unit$is_compatible(target_unit)) {
      stop(paste0("can't convert ", value$unit$full_name, " to ", target_unit$full_name))
    }
    factor <- conversion_factor(value$unit, target_unit)
    return(value$value * factor)
  }
  threshold_numeric(value)
}

normalize_ratio_threshold <- function(value) {
  if (inherits(value, "Measurement")) {
    unit_id <- value$unit$id
    if (!(unit_id %in% c(ratio_unit$id, number_unit$id))) {
      stop(paste0("can't convert ", value$unit$full_name, " to Ratio"))
    }
  }
  numeric_value <- threshold_numeric(value)
  if (numeric_value <= 0) {
    stop("Ratio threshold value must be positive")
  }
  numeric_value
}

normalize_disparity_threshold <- function(value) {
  if (inherits(value, "Measurement")) {
    unit_id <- value$unit$id
    if (!(unit_id %in% c(disparity_unit$id, number_unit$id))) {
      stop(paste0("can't convert ", value$unit$full_name, " to Disparity"))
    }
  }
  threshold_numeric(value)
}

#' @export
Threshold <- R6::R6Class("Threshold",
  public = list(
    metric = NULL,
    value = NULL,
    misrate = NULL,

    initialize = function(metric, value, misrate = DEFAULT_MISRATE) {
      if (!is.character(metric) || length(metric) != 1) {
        stop("metric must be a single character string")
      }
      if (inherits(value, "Measurement")) {
        if (!is.finite(value$value)) stop("threshold value must be finite")
      } else {
        if (!is.numeric(value) || length(value) != 1 || !is.finite(value)) {
          stop("threshold value must be finite")
        }
        value <- as.double(value)
      }
      if (!is.numeric(misrate) || length(misrate) != 1 ||
          !is.finite(misrate) || misrate <= 0 || misrate > 1) {
        stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$MISRATE))
      }
      self$metric <- metric
      self$value <- value
      self$misrate <- misrate
    }
  )
)

#' @export
Projection <- R6::R6Class("Projection",
  public = list(
    threshold = NULL,
    estimate = NULL,
    bounds = NULL,
    verdict = NULL,

    initialize = function(threshold, estimate, bounds, verdict) {
      self$threshold <- threshold
      self$estimate <- estimate
      self$bounds <- bounds
      self$verdict <- verdict
    }
  )
)

as_threshold <- function(x) {
  if (inherits(x, "Threshold")) return(x)
  Threshold$new(
    metric = x$metric,
    value = x$value,
    misrate = if (is.null(x$misrate)) DEFAULT_MISRATE else x$misrate
  )
}

build_projection <- function(threshold, estimate, bounds, verdict) {
  Projection$new(threshold = threshold, estimate = estimate, bounds = bounds, verdict = verdict)
}

# One-sample confirmatory analysis: compares Center/Spread against practical thresholds.
#
# @param x Numeric vector of values or Sample object
# @param thresholds List of threshold specifications. Each threshold is a list with:
#   - metric: "center" or "spread"
#   - value: numeric threshold shorthand or Measurement threshold
#   - misrate: misclassification rate (default 0.001)
# @param seed Optional seed string for reproducible randomization (used for spread bounds)
# @return List of projection results. Each projection has:
#   - threshold: the original threshold list (metric, value, misrate)
#   - estimate: Measurement object (point estimate)
#   - bounds: Bounds object (lower and upper bounds)
#   - verdict: "less", "greater", or "inconclusive"
#' @export
compare1 <- function(x, thresholds, seed = NULL) {
  sx <- as_compare_sample(x, SUBJECTS$X)
  check_non_weighted("x", sx)

  if (length(thresholds) == 0) {
    stop("thresholds list cannot be empty")
  }

  specs <- compare1_specs()
  normalized_values <- numeric(length(thresholds))
  misrates <- numeric(length(thresholds))
  projections <- vector("list", length(thresholds))

  for (i in seq_along(thresholds)) {
    threshold <- as_threshold(thresholds[[i]])
    thresholds[[i]] <- threshold

    metric <- threshold$metric
    if (!(metric %in% c(METRIC_CENTER, METRIC_SPREAD))) {
      stop(paste0("Metric ", metric, " is not supported by Compare1. Use Compare2 instead."))
    }

    normalized_values[[i]] <- normalize_linear_threshold(threshold$value, sx$unit)
    misrates[[i]] <- threshold$misrate
  }

  by_metric <- list(center = list(), spread = list())
  for (i in seq_along(thresholds)) {
    threshold <- thresholds[[i]]
    metric <- threshold$metric
    by_metric[[metric]] <- append(by_metric[[metric]], list(list(idx = i, threshold = threshold)))
  }

  for (metric in names(by_metric)) {
    entries <- by_metric[[metric]]
    if (length(entries) == 0) next

    spec <- specs[[metric]]
    estimate <- spec$estimate(sx)

    for (entry in entries) {
      idx <- entry$idx
      misrate <- misrates[[idx]]

      bounds <-
        if (spec$has_seed && !is.null(seed)) {
          spec$bounds(sx, misrate, seed)
        } else {
          spec$bounds(sx, misrate)
        }

      verdict <- compute_verdict(bounds, normalized_values[[idx]])
      projections[[idx]] <- build_projection(entry$threshold, estimate, bounds, verdict)
    }
  }

  projections
}

# Two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.
#
# @param x Numeric vector of values or Sample object (first sample)
# @param y Numeric vector of values or Sample object (second sample)
# @param thresholds List of threshold specifications. Each threshold is a list with:
#   - metric: "shift", "ratio", or "disparity"
#   - value: numeric threshold shorthand or Measurement threshold
#   - misrate: misclassification rate (default 0.001)
# @param seed Optional seed string for reproducible randomization (used for disparity bounds)
# @return List of projection results. Each projection has:
#   - threshold: the original threshold list (metric, value, misrate)
#   - estimate: Measurement object (point estimate)
#   - bounds: Bounds object (lower and upper bounds)
#   - verdict: "less", "greater", or "inconclusive"
#' @export
compare2 <- function(x, y, thresholds, seed = NULL) {
  sx <- as_compare_sample(x, SUBJECTS$X)
  sy <- as_compare_sample(y, SUBJECTS$Y)
  check_non_weighted("x", sx)
  check_non_weighted("y", sy)
  check_compatible_units(sx, sy)

  if (length(thresholds) == 0) {
    stop("thresholds list cannot be empty")
  }

  specs <- compare2_specs()
  normalized_values <- numeric(length(thresholds))
  misrates <- numeric(length(thresholds))
  projections <- vector("list", length(thresholds))
  shift_unit <- finer(sx$unit, sy$unit)

  for (i in seq_along(thresholds)) {
    threshold <- as_threshold(thresholds[[i]])
    thresholds[[i]] <- threshold

    metric <- threshold$metric
    if (!(metric %in% c(METRIC_SHIFT, METRIC_RATIO, METRIC_DISPARITY))) {
      stop(paste0("Metric ", metric, " is not supported by Compare2. Use Compare1 instead."))
    }

    normalized_values[[i]] <-
      if (metric == METRIC_SHIFT) {
        normalize_linear_threshold(threshold$value, shift_unit)
      } else if (metric == METRIC_RATIO) {
        normalize_ratio_threshold(threshold$value)
      } else {
        normalize_disparity_threshold(threshold$value)
      }
    misrates[[i]] <- threshold$misrate
  }

  by_metric <- list(shift = list(), ratio = list(), disparity = list())
  for (i in seq_along(thresholds)) {
    threshold <- thresholds[[i]]
    metric <- threshold$metric
    by_metric[[metric]] <- append(by_metric[[metric]], list(list(idx = i, threshold = threshold)))
  }

  for (metric in names(by_metric)) {
    entries <- by_metric[[metric]]
    if (length(entries) == 0) next

    spec <- specs[[metric]]
    estimate <- spec$estimate(sx, sy)

    for (entry in entries) {
      idx <- entry$idx
      misrate <- misrates[[idx]]

      bounds <-
        if (spec$has_seed && !is.null(seed)) {
          spec$bounds(sx, sy, misrate, seed)
        } else {
          spec$bounds(sx, sy, misrate)
        }

      verdict <- compute_verdict(bounds, normalized_values[[idx]])
      projections[[idx]] <- build_projection(entry$threshold, estimate, bounds, verdict)
    }
  }

  projections
}
