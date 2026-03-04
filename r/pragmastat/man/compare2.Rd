\name{compare2}
\alias{compare2}
\title{Compare2: Two-Sample Confirmatory Analysis}
\usage{
compare2(x, y, thresholds, seed = NULL)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} for the first sample.}
\item{y}{A numeric vector or \code{\link{Sample}} for the second sample.}
\item{thresholds}{
A non-empty list of threshold specifications. Each item must be a list with:
\code{metric} (\code{"shift"}, \code{"ratio"}, or \code{"disparity"}),
\code{value} (finite numeric threshold shorthand or \code{\link{Measurement}}),
and optional \code{misrate} in \code{(0, 1]} (defaults to \code{DEFAULT_MISRATE}).
For \code{"ratio"}, \code{value} must be strictly positive.
}
\item{seed}{Optional string seed for deterministic randomization in disparity bounds.}
}
\description{
Performs two-sample confirmatory analysis by comparing \code{shift}, \code{ratio},
or \code{disparity} against practical thresholds and returning a verdict
for each threshold.
}
\details{
Each threshold is evaluated independently. The output order always matches
the input threshold order.

When \code{x} and \code{y} are \code{\link{Sample}} objects, threshold values may be
provided as \code{\link{Measurement}} objects and will be normalized to the working
comparison scale. Plain numeric thresholds remain supported as shorthand on that scale.

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
  list(metric = "shift", value = 0, misrate = 1e-3),
  list(metric = "ratio", value = 1, misrate = 1e-3)
)

result <- compare2(1:30, 21:50, thresholds, seed = "demo")
result[[1]]$verdict
result[[2]]$verdict
}
\seealso{
\code{\link{compare1}},
\code{\link{shift_bounds}},
\code{\link{ratio_bounds}},
\code{\link{disparity_bounds}}
}
