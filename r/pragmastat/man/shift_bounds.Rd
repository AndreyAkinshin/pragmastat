\name{shift_bounds}
\alias{shift_bounds}
\title{Bounds on the Shift Estimator}
\usage{
shift_bounds(x, y, misrate)
}
\arguments{
\item{x}{A numeric vector representing the first sample.}
\item{y}{A numeric vector representing the second sample.}
\item{misrate}{Misclassification rate, the probability that the true shift falls outside bounds.}
}
\description{
Provides bounds on the Shift estimator with a specified misclassification rate.
The misrate represents the probability that the true shift falls outside the computed bounds.
This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
}
\value{
A list with components:
\item{lower}{The lower bound on the shift estimator.}
\item{upper}{The upper bound on the shift estimator.}
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- c(3, 4, 5, 6, 7)
shift_bounds(x, y, 0.05)

# With larger samples
set.seed(42)
x <- rnorm(50, mean = 0)
y <- rnorm(50, mean = 2)
bounds <- shift_bounds(x, y, 0.05)
bounds$lower
bounds$upper

}
\seealso{
\code{\link{shift}} for the point estimator,
\code{\link{pairwise_margin}} for the underlying margin calculation.
}
