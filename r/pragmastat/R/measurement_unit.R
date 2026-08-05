# MeasurementUnit represents a unit of measurement with identity, family, and conversion support.

MeasurementUnit <- R6::R6Class(
  "MeasurementUnit",
  private = list(
    .id = NULL,
    .family = NULL,
    .abbreviation = NULL,
    .full_name = NULL,
    .base_units = NULL
  ),
  # Active bindings rather than public fields. A unit is not a value a caller holds: number_unit
  # is what every Sample, every Measurement and every conversion in the session reaches, so
  # `number_unit$base_units <- 5` does not corrupt one object, it rescales conversion_factor and
  # finer for everything computed afterwards, inside the package as well as outside. Nothing
  # detects that and nothing can, since a unit is never revalidated after construction, so the
  # wrong number surfaces somewhere else entirely with no link back to the assignment.
  #
  # An R6 public field takes an assignment silently. An active binding with no assignment branch
  # raises, which is what makes the refusal real rather than documented.
  active = list(
    #' @field id Unique identifier for this unit
    id = function(value) {
      if (missing(value)) private$.id else stop("MeasurementUnit is read-only")
    },

    #' @field family Unit family for compatibility checks
    family = function(value) {
      if (missing(value)) private$.family else stop("MeasurementUnit is read-only")
    },

    #' @field abbreviation Short display abbreviation
    abbreviation = function(value) {
      if (missing(value)) private$.abbreviation else stop("MeasurementUnit is read-only")
    },

    #' @field full_name Human-readable full name
    full_name = function(value) {
      if (missing(value)) private$.full_name else stop("MeasurementUnit is read-only")
    },

    #' @field base_units Number of base units (for conversion)
    base_units = function(value) {
      if (missing(value)) private$.base_units else stop("MeasurementUnit is read-only")
    }
  ),
  public = list(
    #' @description Create a new MeasurementUnit
    #' @param id Unique identifier
    #' @param family Unit family
    #' @param abbreviation Short abbreviation
    #' @param full_name Full name
    #' @param base_units Number of base units
    initialize = function(id, family, abbreviation, full_name, base_units) {
      # A non-positive base_units poisons every conversion through this unit, and it
      # makes finer pick it over any well-formed one. An empty family makes unrelated
      # units compatible. An empty abbreviation is legal: the three dimensionless
      # units have none.
      if (!nzchar(id)) {
        stop("id cannot be empty")
      }
      if (!nzchar(family)) {
        stop("family cannot be empty")
      }
      if (!is.finite(base_units) || base_units <= 0) {
        stop("base_units must be positive")
      }
      # R has one numeric type, so nothing above stops a fractional scale. The four ports that type
      # this as a 64-bit integer cannot express one at all, so a unit that constructs here and
      # nowhere else breaks the promise that one program reads the same in every port. is.finite
      # above is what rules out Inf: round(Inf) is Inf, so the test below would pass it.
      if (base_units != round(base_units)) {
        stop("base_units must be a whole number")
      }
      private$.id <- id
      private$.family <- family
      private$.abbreviation <- abbreviation
      private$.full_name <- full_name
      private$.base_units <- base_units
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
