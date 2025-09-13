\name{shift}
\alias{shift}
\title{Shift Estimator}
\usage{
shift(x, y)
}
\arguments{
\item{x}{A numeric vector representing the first sample.}
\item{y}{A numeric vector representing the second sample.}
}
\description{
Computes the Shift estimator - the median of all pairwise differences (xi - yj)
where xi comes from the first sample and yj from the second sample.
This measure estimates the typical difference between elements of x and y and
provides a robust alternative to the difference of means.
This estimator is also known as the Hodges-Lehmann shift estimator.
}
\value{
A single numeric value representing the Shift estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- c(3, 4, 5, 6, 7)
shift(x, y)  # Should be negative since y values are generally larger

# Compare with difference of means
x <- c(1, 2, 3, 4, 500)  # Contains outlier
y <- c(3, 4, 5, 6, 7)
mean(x) - mean(y)  # Affected by outlier
shift(x, y)        # Robust to outlier

# When samples are similar
x <- c(1, 2, 3, 4, 5)
y <- c(1, 2, 3, 4, 5)
shift(x, y)  # Should be close to 0

}
\references{
Hodges, J. L., & Lehmann, E. L. (1963). Estimates of location based on rank tests.
The Annals of Mathematical Statistics, 34(2), 598-611.
}