# Measurement represents a value with an associated unit.

#' @export
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
    }
  )
)

#' @export
as.numeric.Measurement <- function(x, ...) {
  x$value
}
