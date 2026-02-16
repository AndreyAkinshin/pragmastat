\name{disparity_bounds}
\alias{disparity_bounds}
\title{Distribution-Free Bounds for Disparity}
\usage{
disparity_bounds(x, y, misrate = DEFAULT_MISRATE, seed = NULL)
}
\arguments{
\item{x}{Numeric vector of values (at least 2 elements).}
\item{y}{Numeric vector of values (at least 2 elements).}
\item{misrate}{Misclassification rate. Uses Bonferroni split between shift and avg_spread components.}
\item{seed}{Optional string seed for deterministic randomization.}
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
A list with components:
\item{lower}{The lower bound on the disparity estimator.}
\item{upper}{The upper bound on the disparity estimator.}
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
