\name{Projection}
\alias{Projection}
\title{Compare Projection Result}
\description{
An R6 class representing the result of a confirmatory comparison.
Contains the threshold, point estimate, confidence bounds, and verdict.
Returned by \code{\link{compare1}} and \code{\link{compare2}}.
}
\section{Fields}{
\describe{
\item{\code{threshold}}{\code{\link{Threshold}}. The threshold used for comparison.}
\item{\code{estimate}}{\code{\link{Measurement}}. Point estimate.}
\item{\code{bounds}}{\code{\link{Bounds}}. Confidence bounds.}
\item{\code{verdict}}{Character. One of \code{"less"}, \code{"greater"}, or \code{"inconclusive"}.}
}
}
\section{Methods}{
\describe{
\item{\code{initialize(threshold, estimate, bounds, verdict)}}{Create a new Projection.}
}
}
\seealso{
\code{\link{Threshold}}, \code{\link{compare1}}, \code{\link{compare2}}
}
