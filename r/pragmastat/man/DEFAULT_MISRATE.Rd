\name{DEFAULT_MISRATE}
\alias{DEFAULT_MISRATE}
\docType{data}
\title{Default Misclassification Rate}
\description{
The default misclassification rate for every bounds estimator and for
\code{\link{Threshold}}: the probability that the true value falls outside the
computed interval.

It appears in the \code{\\usage} line of \code{\link{center_bounds}},
\code{\link{spread_bounds}}, \code{\link{shift_bounds}}, \code{\link{ratio_bounds}},
\code{\link{disparity_bounds}}, \code{\link{compare1}} and \code{\link{compare2}}, so a reader
of those pages can now type the name and get the value.
}
\usage{
DEFAULT_MISRATE
}
\format{
A single numeric value, \code{1e-3}.
}
\examples{
DEFAULT_MISRATE          # 0.001
center_bounds(1:100)     # same as center_bounds(1:100, DEFAULT_MISRATE)
}
\seealso{
\code{\link{center_bounds}}, \code{\link{Threshold}}
}
