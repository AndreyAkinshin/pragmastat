# Bounds represents an interval [lower, upper] with an associated measurement unit.

# Pairs two endpoints into the list(lower, upper) that the native-array bounds
# API returns, normalizing both (see normalize_zero).
#
# Every bounds result passes through here: the vector path returns this list
# as-is, and the Sample path builds its Bounds object out of the same list. One
# spot covers both public surfaces, instead of the endpoint construction sites
# scattered across the bounds estimators.
bounds_list <- function(lower, upper) {
  list(lower = normalize_zero(lower), upper = normalize_zero(upper))
}

Bounds <- R6::R6Class(
  "Bounds",
  public = list(
    #' @field lower Lower bound value
    lower = NULL,

    #' @field upper Upper bound value
    upper = NULL,

    #' @field unit MeasurementUnit associated with the bounds
    unit = NULL,

    #' @description Create new Bounds, normalizing both endpoints (see normalize_zero)
    #' @param lower Lower bound value
    #' @param upper Upper bound value
    #' @param unit MeasurementUnit (defaults to number_unit)
    initialize = function(lower, upper, unit = NULL) {
      if (is.null(unit)) {
        unit <- number_unit
      }
      self$lower <- normalize_zero(lower)
      self$upper <- normalize_zero(upper)
      self$unit <- unit
    },

    #' @description Check if a value is within the bounds
    #' @param value Numeric value to check
    #' @return TRUE if value is within [lower, upper]
    contains = function(value) {
      self$lower <= value && value <= self$upper
    },

    #' @description Render the interval the way every port renders it
    #' @param ... Ignored, present for compatibility with the format generic
    #' @return "[lower;upper]", with the unit's abbreviation when it has one
    format = function(...) {
      interval <- paste0("[", format_value(self$lower), ";", format_value(self$upper), "]")
      if (nzchar(self$unit$abbreviation)) paste(interval, self$unit$abbreviation) else interval
    }
  )
)
