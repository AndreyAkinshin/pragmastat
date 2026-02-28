# Bounds represents an interval [lower, upper] with an associated measurement unit.

#' @export
Bounds <- R6::R6Class(
  "Bounds",
  public = list(
    #' @field lower Lower bound value
    lower = NULL,

    #' @field upper Upper bound value
    upper = NULL,

    #' @field unit MeasurementUnit associated with the bounds
    unit = NULL,

    #' @description Create new Bounds
    #' @param lower Lower bound value
    #' @param upper Upper bound value
    #' @param unit MeasurementUnit (defaults to number_unit)
    initialize = function(lower, upper, unit = NULL) {
      if (is.null(unit)) {
        unit <- number_unit
      }
      self$lower <- lower
      self$upper <- upper
      self$unit <- unit
    },

    #' @description Check if a value is within the bounds
    #' @param value Numeric value to check
    #' @return TRUE if value is within [lower, upper]
    contains = function(value) {
      self$lower <= value && value <= self$upper
    }
  )
)
