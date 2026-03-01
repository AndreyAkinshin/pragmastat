\name{Bounds}
\alias{Bounds}
\title{Bounds Interval with Unit}
\description{
An R6 class representing an interval \code{[lower, upper]} with an associated
\code{\link{MeasurementUnit}}. Returned by bounds estimator functions
(e.g. \code{\link{center_bounds}}, \code{\link{shift_bounds}}) when given
\code{\link{Sample}} input.
}
\section{Fields}{
\describe{
\item{\code{lower}}{Numeric. Lower bound value.}
\item{\code{upper}}{Numeric. Upper bound value.}
\item{\code{unit}}{\code{\link{MeasurementUnit}}. The associated unit (defaults to \code{\link{number_unit}}).}
}
}
\section{Methods}{
\describe{
\item{\code{initialize(lower, upper, unit = NULL)}}{Create new Bounds. If \code{unit} is \code{NULL}, defaults to \code{\link{number_unit}}.}
\item{\code{contains(value)}}{Check if \code{value} is within \code{[lower, upper]}. Returns \code{TRUE} or \code{FALSE}.}
}
}
\examples{
b <- Bounds$new(1.5, 4.5)
b$lower       # 1.5
b$upper       # 4.5
b$contains(3) # TRUE
b$contains(5) # FALSE
}
\seealso{
\code{\link{Measurement}}, \code{\link{center_bounds}}, \code{\link{shift_bounds}}
}
