\name{Measurement}
\alias{Measurement}
\alias{as.numeric.Measurement}
\title{Measurement Value with Unit}
\description{
An R6 class representing a numeric value paired with a \code{\link{MeasurementUnit}}.
Returned by estimator functions when given \code{\link{Sample}} input.

The S3 method \code{as.numeric} extracts the raw numeric value.
}
\section{Fields}{
\describe{
\item{\code{value}}{Numeric. The measurement value.}
\item{\code{unit}}{\code{\link{MeasurementUnit}}. The associated unit (defaults to \code{\link{number_unit}}).}
}
}
\section{Methods}{
\describe{
\item{\code{initialize(value, unit = NULL)}}{Create a new Measurement. If \code{unit} is \code{NULL}, defaults to \code{\link{number_unit}}.}
}
}
\examples{
m <- Measurement$new(42.0)
m$value       # 42.0
m$unit$id     # "number"
as.numeric(m) # 42.0

ms <- MeasurementUnit$new("ms", "time", "ms", "milliseconds", 1)
m <- Measurement$new(3.14, ms)
m$unit$id  # "ms"
}
\seealso{
\code{\link{MeasurementUnit}}, \code{\link{Sample}}, \code{\link{Bounds}}
}
