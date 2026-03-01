\name{ratio_bounds}
\alias{ratio_bounds}
\title{Bounds on the Ratio Estimator}
\usage{
ratio_bounds(x, y, misrate = DEFAULT_MISRATE)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} representing the first sample (must be strictly positive).}
\item{y}{A numeric vector or \code{\link{Sample}} representing the second sample (must be strictly positive).}
\item{misrate}{Misclassification rate, the probability that the true ratio falls outside bounds.}
}
\description{
Provides bounds on the Ratio estimator with a specified misclassification rate.
The misrate represents the probability that the true ratio falls outside the computed bounds.
This is a pragmatic alternative to traditional confidence intervals for ratio estimation.
}
\value{
A \code{\link{Bounds}} object when given \code{\link{Sample}} input,
or a list with components \code{lower} and \code{upper} when given numeric vectors.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
y <- c(2, 3, 4, 5, 6, 7, 8, 9, 10, 11)
ratio_bounds(x, y, 1e-3)

# With larger samples
set.seed(42)
x <- exp(rnorm(50, mean = 0))
y <- exp(rnorm(50, mean = 0.5))
bounds <- ratio_bounds(x, y, 1e-3)
bounds$lower
bounds$upper

}
\seealso{
\code{\link{ratio}} for the point estimator.
}
