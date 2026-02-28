# UnitRegistry stores measurement units and enables lookup by ID.

#' @export
UnitRegistry <- R6::R6Class(
  "UnitRegistry",
  public = list(
    #' @description Create a new empty UnitRegistry
    initialize = function() {
      private$.by_id <- list()
    },

    #' @description Register a unit in the registry
    #' @param unit MeasurementUnit to register
    register = function(unit) {
      if (!is.null(private$.by_id[[unit$id]])) {
        stop(paste0("unit with id '", unit$id, "' is already registered"))
      }
      private$.by_id[[unit$id]] <- unit
      invisible(self)
    },

    #' @description Look up a unit by ID
    #' @param id Unit identifier string
    #' @return MeasurementUnit
    resolve = function(id) {
      unit <- private$.by_id[[id]]
      if (is.null(unit)) {
        stop(paste0("unknown unit id: '", id, "'"))
      }
      unit
    }
  ),
  private = list(
    .by_id = NULL
  )
)

#' Create a standard registry pre-populated with Number, Ratio, and Disparity units.
#' @return UnitRegistry
#' @export
standard_registry <- function() {
  reg <- UnitRegistry$new()
  reg$register(number_unit)
  reg$register(ratio_unit)
  reg$register(disparity_unit)
  reg
}
