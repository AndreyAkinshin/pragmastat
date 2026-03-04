\name{compare1}
\alias{compare1}
\title{Compare1: One-Sample Confirmatory Analysis}
\usage{
compare1(x, thresholds, seed = NULL)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}}.}
\item{thresholds}{
A non-empty list of threshold specifications. Each item must be a list with:
\code{metric} (\code{"center"} or \code{"spread"}),
\code{value} (finite numeric threshold shorthand or \code{\link{Measurement}}),
and optional \code{misrate} in \code{(0, 1]} (defaults to \code{DEFAULT_MISRATE}).
}
\item{seed}{Optional string seed for deterministic randomization in spread bounds.}
}
\description{
Performs one-sample confirmatory analysis by comparing \code{center} or \code{spread}
against practical thresholds and returning a verdict for each threshold.
}
\details{
Each threshold is evaluated independently. The output order always matches
the input threshold order.

When \code{x} is a \code{\link{Sample}}, threshold values may be provided as
\code{\link{Measurement}} objects and will be converted to the sample unit.
Plain numeric thresholds remain supported as shorthand on the comparison scale.

Verdicts are assigned as follows:
\itemize{
\item \code{"greater"} if lower bound is strictly greater than the threshold value
\item \code{"less"} if upper bound is strictly less than the threshold value
\item \code{"inconclusive"} otherwise
}
}
\value{
A list of \code{\link{Projection}} objects. Each projection has fields:
\itemize{
\item \code{threshold} (\code{\link{Threshold}})
\item \code{estimate} (\code{\link{Measurement}})
\item \code{bounds} (\code{\link{Bounds}} with \code{$lower} and \code{$upper})
\item \code{verdict} (\code{"less"}, \code{"greater"}, or \code{"inconclusive"})
}
}
\examples{
thresholds <- list(
  list(metric = "center", value = 20, misrate = 1e-3),
  list(metric = "spread", value = 0.1, misrate = 1e-3)
)

result <- compare1(1:30, thresholds, seed = "demo")
result[[1]]$verdict
result[[2]]$verdict
}
\seealso{
\code{\link{compare2}},
\code{\link{center_bounds}},
\code{\link{spread_bounds}}
}
