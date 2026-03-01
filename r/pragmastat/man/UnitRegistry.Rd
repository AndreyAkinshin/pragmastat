\name{UnitRegistry}
\alias{UnitRegistry}
\title{Measurement Unit Registry}
\description{
An R6 class that stores \code{\link{MeasurementUnit}} instances and enables
lookup by identifier. Use \code{\link{standard_registry}} to create a registry
pre-populated with the standard units.
}
\section{Methods}{
\describe{
\item{\code{initialize()}}{Create a new empty UnitRegistry.}
\item{\code{register(unit)}}{Register a \code{\link{MeasurementUnit}}. Raises an error if a unit with the same id is already registered. Returns \code{self} (invisibly) for chaining.}
\item{\code{resolve(id)}}{Look up a unit by its identifier string. Raises an error if the id is not found.}
}
}
\examples{
reg <- UnitRegistry$new()
ms <- MeasurementUnit$new("ms", "time", "ms", "milliseconds", 1)
reg$register(ms)
reg$resolve("ms")$full_name  # "milliseconds"

# Or use the pre-populated factory
reg <- standard_registry()
reg$resolve("number")$full_name  # "Number"
}
\seealso{
\code{\link{standard_registry}}, \code{\link{MeasurementUnit}}
}
