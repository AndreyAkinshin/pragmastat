# Measurement represents a value with an associated unit.

Measurement <- R6::R6Class(
  "Measurement",
  public = list(
    #' @field value Numeric measurement value
    value = NULL,

    #' @field unit MeasurementUnit associated with the value
    unit = NULL,

    #' @description Create a new Measurement
    #' @param value Numeric value
    #' @param unit MeasurementUnit (defaults to number_unit)
    initialize = function(value, unit = NULL) {
      if (is.null(unit)) {
        unit <- number_unit
      }
      self$value <- value
      self$unit <- unit
    },

    #' @description Render the value the way every port renders it
    #' @param ... Ignored, present for compatibility with the format generic
    #' @return The value, with the unit's abbreviation when it has one
    format = function(...) {
      rendered <- format_value(self$value)
      if (nzchar(self$unit$abbreviation)) paste(rendered, self$unit$abbreviation) else rendered
    }
  )
)

as.double.Measurement <- function(x, ...) {
  x$value
}
