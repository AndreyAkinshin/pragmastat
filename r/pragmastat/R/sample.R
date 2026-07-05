# Sample wraps values with optional weights and a measurement unit.

#' @export
Sample <- R6::R6Class(
  "Sample",
  public = list(
    #' @description Create a new Sample
    #' @param values Numeric vector of values
    #' @param weights Optional numeric vector of weights (same length as values)
    #' @param unit MeasurementUnit (defaults to number_unit)
    initialize = function(values, weights = NULL, unit = NULL) {
      if (is.null(unit)) {
        unit <- number_unit
      }
      # Construction errors always report subject "x": construction cannot know
      # whether this sample will be used as arg1 ("x") or arg2 ("y"). The error
      # subject is supplied positionally by the validating estimator instead.
      if (length(values) == 0) {
        stop(assumption_error(ASSUMPTION_IDS$VALIDITY, SUBJECTS$X))
      }
      if (any(is.na(values) | is.nan(values) | is.infinite(values))) {
        stop(assumption_error(ASSUMPTION_IDS$VALIDITY, SUBJECTS$X))
      }
      private$.values <- as.double(values)
      private$.unit <- unit

      if (!is.null(weights)) {
        if (length(weights) != length(values)) {
          stop("weights length must match values length")
        }
        if (any(weights < 0)) {
          stop("all weights must be non-negative")
        }
        total_w <- sum(weights)
        if (total_w < 1e-9) {
          stop("total weight must be positive")
        }
        private$.weights <- as.double(weights)
        private$.total_weight <- total_w
        private$.weighted_size <- (total_w * total_w) / sum(weights * weights)
      } else {
        private$.total_weight <- 1.0
        private$.weighted_size <- as.double(length(values))
      }
    },

    #' @description Convert sample to a compatible target unit
    #' @param target MeasurementUnit
    #' @return A new Sample with converted values
    convert_to = function(target) {
      if (!private$.unit$is_compatible(target)) {
        stop(paste0("can't convert ", private$.unit$full_name, " to ", target$full_name))
      }
      if (identical(private$.unit, target)) {
        return(self)
      }
      factor <- conversion_factor(private$.unit, target)
      converted <- private$.values * factor
      Sample$new(converted,
        weights = private$.weights, unit = target
      )
    },

    #' @description Log-transform the sample values (positivity required)
    #' @return A new Sample with log-transformed values and NumberUnit
    log_transform = function() {
      if (any(private$.values <= 0)) {
        stop(assumption_error(ASSUMPTION_IDS$POSITIVITY, SUBJECTS$X))
      }
      Sample$new(log(private$.values),
        weights = private$.weights,
        unit = number_unit
      )
    }
  ),
  active = list(
    #' @field sorted_values Lazily computed sorted copy of values
    sorted_values = function() {
      if (is.null(private$.sorted_values)) {
        private$.sorted_values <- sort(private$.values)
      }
      private$.sorted_values
    },

    #' @field size Number of values
    size = function() {
      length(private$.values)
    },

    #' @field is_weighted TRUE if sample has weights
    is_weighted = function() {
      !is.null(private$.weights)
    },

    #' @field total_weight Total weight (1.0 for unweighted)
    total_weight = function() {
      private$.total_weight
    },

    #' @field weighted_size Effective sample size
    weighted_size = function() {
      private$.weighted_size
    },

    #' @field values Raw values vector
    values = function() {
      private$.values
    },

    #' @field weights Weights vector (NULL if unweighted)
    weights = function() {
      private$.weights
    },

    #' @field unit The measurement unit
    unit = function() {
      private$.unit
    }
  ),
  private = list(
    .values = NULL,
    .weights = NULL,
    .unit = NULL,
    .sorted_values = NULL,
    .total_weight = NULL,
    .weighted_size = NULL
  )
)

# Check that a sample is not weighted; stop with error if it is.
check_non_weighted <- function(name, s) {
  if (is.null(s)) {
    stop(paste0(name, " cannot be NULL"))
  }
  if (s$is_weighted) {
    stop(paste0("weighted samples are not supported for ", name))
  }
}

# Check that two samples have compatible units.
check_compatible_units <- function(a, b) {
  if (!a$unit$is_compatible(b$unit)) {
    stop(paste0("can't convert ", a$unit$full_name, " to ", b$unit$full_name))
  }
}

# Convert both samples to the finer unit.
convert_to_finer <- function(a, b) {
  if (identical(a$unit, b$unit)) {
    return(list(a = a, b = b))
  }
  target <- finer(a$unit, b$unit)
  list(a = a$convert_to(target), b = b$convert_to(target))
}

# S3 operators for Sample

#' @export
`*.Sample` <- function(e1, e2) {
  if (inherits(e1, "Sample")) {
    Sample$new(e1$values * e2,
      weights = e1$weights, unit = e1$unit
    )
  } else {
    Sample$new(e1 * e2$values,
      weights = e2$weights, unit = e2$unit
    )
  }
}

#' @export
`+.Sample` <- function(e1, e2) {
  if (inherits(e1, "Sample")) {
    Sample$new(e1$values + e2,
      weights = e1$weights, unit = e1$unit
    )
  } else {
    Sample$new(e1 + e2$values,
      weights = e2$weights, unit = e2$unit
    )
  }
}
