\name{number_unit}
\alias{number_unit}
\alias{ratio_unit}
\alias{disparity_unit}
\title{Standard Measurement Units}
\description{
Pre-defined \code{\link{MeasurementUnit}} instances for common estimator result types.

\describe{
\item{\code{number_unit}}{Dimensionless number. Default unit for \code{\link{Sample}} and \code{\link{Measurement}}.}
\item{\code{ratio_unit}}{Unit for ratio estimator results (see \code{\link{ratio}}).}
\item{\code{disparity_unit}}{Unit for disparity estimator results (see \code{\link{disparity}}).}
}
}
\usage{
number_unit
ratio_unit
disparity_unit
}
\examples{
number_unit$id    # "number"
ratio_unit$id     # "ratio"
disparity_unit$id # "disparity"

# All three are pre-registered in standard_registry()
reg <- standard_registry()
identical(reg$resolve("number"), number_unit)  # TRUE
}
\seealso{
\code{\link{MeasurementUnit}}, \code{\link{standard_registry}}
}
