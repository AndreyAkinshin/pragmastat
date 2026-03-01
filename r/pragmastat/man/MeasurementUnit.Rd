\name{MeasurementUnit}
\alias{MeasurementUnit}
\title{Measurement Unit}
\description{
An R6 class representing a unit of measurement with identity, family, and conversion support.
Units within the same family are compatible and can be converted between each other
using base unit ratios.
}
\section{Fields}{
\describe{
\item{\code{id}}{Character. Unique identifier for this unit.}
\item{\code{family}}{Character. Unit family for compatibility checks.}
\item{\code{abbreviation}}{Character. Short display abbreviation.}
\item{\code{full_name}}{Character. Human-readable full name.}
\item{\code{base_units}}{Numeric. Number of base units (used for conversion).}
}
}
\section{Methods}{
\describe{
\item{\code{initialize(id, family, abbreviation, full_name, base_units)}}{Create a new MeasurementUnit.}
\item{\code{is_compatible(other)}}{Check if this unit belongs to the same family as \code{other}. Returns \code{TRUE} or \code{FALSE}.}
}
}
\examples{
ms <- MeasurementUnit$new("ms", "time", "ms", "milliseconds", 1)
sec <- MeasurementUnit$new("s", "time", "s", "seconds", 1000)
ms$is_compatible(sec)  # TRUE
}
\seealso{
\code{\link{number_unit}}, \code{\link{Measurement}}, \code{\link{Sample}}
}
