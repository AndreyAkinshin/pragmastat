\name{shift_bounds}
\alias{shift_bounds}
\title{Bounds on the Shift Estimator}
\usage{
shift_bounds(x, y, misrate = DEFAULT_MISRATE)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} representing the first sample.}
\item{y}{A numeric vector or \code{\link{Sample}} representing the second sample.}
\item{misrate}{Misclassification rate, the probability that the true shift falls outside bounds.}
}
\description{
Provides bounds on the Shift estimator with a specified misclassification rate.
The misrate represents the probability that the true shift falls outside the computed bounds.
This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
}
\value{
A \code{\link{Bounds}} object when given \code{\link{Sample}} input,
or a list with components \code{lower} and \code{upper} when given numeric vectors.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
y <- c(3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
shift_bounds(x, y, 1e-3)

# With larger samples
set.seed(42)
x <- rnorm(50, mean = 0)
y <- rnorm(50, mean = 2)
bounds <- shift_bounds(x, y, 1e-3)
bounds$lower
bounds$upper

}
\seealso{
\code{\link{shift}} for the point estimator.
}
