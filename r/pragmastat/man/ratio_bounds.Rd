\name{ratio_bounds}
\alias{ratio_bounds}
\title{Bounds on the Ratio Estimator}
\usage{
ratio_bounds(x, y, misrate = DEFAULT_MISRATE)
}
\arguments{
\item{x}{A numeric vector representing the first sample (must be strictly positive).}
\item{y}{A numeric vector representing the second sample (must be strictly positive).}
\item{misrate}{Misclassification rate, the probability that the true ratio falls outside bounds.}
}
\description{
Provides bounds on the Ratio estimator with a specified misclassification rate.
The misrate represents the probability that the true ratio falls outside the computed bounds.
This is a pragmatic alternative to traditional confidence intervals for ratio estimation.
}
\value{
A list with components:
\item{lower}{The lower bound on the ratio estimator.}
\item{upper}{The upper bound on the ratio estimator.}
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- c(2, 3, 4, 5, 6)
ratio_bounds(x, y, 0.05)

# With larger samples
set.seed(42)
x <- exp(rnorm(50, mean = 0))
y <- exp(rnorm(50, mean = 0.5))
bounds <- ratio_bounds(x, y, 0.05)
bounds$lower
bounds$upper

}
\seealso{
\code{\link{ratio}} for the point estimator,
\code{\link{pairwise_margin}} for the underlying margin calculation.
}
