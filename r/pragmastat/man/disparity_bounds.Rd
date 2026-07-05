\name{disparity_bounds}
\alias{disparity_bounds}
\title{Distribution-Free Bounds for Disparity}
\usage{
disparity_bounds(x, y, misrate = DEFAULT_MISRATE, seed = NULL, assume_sorted = FALSE)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} (at least 2 elements).}
\item{y}{A numeric vector or \code{\link{Sample}} (at least 2 elements).}
\item{misrate}{Misclassification rate. Uses Bonferroni split between shift and avg_spread components.}
\item{seed}{Optional string seed for deterministic randomization.}

\item{assume_sorted}{If \code{TRUE}, skip the internal sort of the
order-independent shift and sparity sub-computations for numeric vector inputs.
The disjoint-pair shuffle always runs on the original order, so the result is
unchanged. Ignored for \code{\link{Sample}} input. Passing \code{TRUE} on
unsorted input is undefined behavior for those sub-computations.}
}
\description{
Provides distribution-free bounds for the Disparity estimator (Shift / AvgSpread) using
Bonferroni combination of \code{shift_bounds} and \code{avg_spread_bounds}.
}
\details{
Splits the misrate budget between a \code{shift_bounds} call and an \code{avg_spread_bounds} call,
then combines the resulting intervals to bound the ratio Shift / AvgSpread.
}
\value{
A \code{\link{Bounds}} object when given \code{\link{Sample}} input,
or a list with components \code{lower} and \code{upper} when given numeric vectors.
}
\seealso{
\code{\link{shift_bounds}} for shift bounds,
\code{\link{disparity}} for the point estimator.
}
\examples{
# Basic usage with natural sequences
disparity_bounds(1:30, 21:50, 0.02, seed = "demo")

# Tighter misrate produces wider bounds
disparity_bounds(1:20, 5:24, 0.01, seed = "demo")
}
