# MeasurementUnit represents a unit of measurement with identity, family, and conversion support.

#' @export
MeasurementUnit <- R6::R6Class(
  "MeasurementUnit",
  public = list(
    #' @field id Unique identifier for this unit
    id = NULL,

    #' @field family Unit family for compatibility checks
    family = NULL,

    #' @field abbreviation Short display abbreviation
    abbreviation = NULL,

    #' @field full_name Human-readable full name
    full_name = NULL,

    #' @field base_units Number of base units (for conversion)
    base_units = NULL,

    #' @description Create a new MeasurementUnit
    #' @param id Unique identifier
    #' @param family Unit family
    #' @param abbreviation Short abbreviation
    #' @param full_name Full name
    #' @param base_units Number of base units
    initialize = function(id, family, abbreviation, full_name, base_units) {
      self$id <- id
      self$family <- family
      self$abbreviation <- abbreviation
      self$full_name <- full_name
      self$base_units <- base_units
    },

    #' @description Check if this unit is compatible with another
    #' @param other Another MeasurementUnit
    #' @return TRUE if both units belong to the same family
    is_compatible = function(other) {
      self$family == other$family
    }
  )
)

# Standard units
number_unit <- MeasurementUnit$new("number", "Number", "", "Number", 1)
ratio_unit <- MeasurementUnit$new("ratio", "Ratio", "", "Ratio", 1)
disparity_unit <- MeasurementUnit$new("disparity", "Disparity", "", "Disparity", 1)

# Returns the unit with smaller base_units (higher precision).
finer <- function(a, b) {
  if (a$base_units <= b$base_units) a else b
}

# Returns the multiplier to convert from one unit to another.
conversion_factor <- function(from, to) {
  from$base_units / to$base_units
}
