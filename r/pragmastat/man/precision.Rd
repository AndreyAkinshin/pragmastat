\name{precision}
\alias{precision}
\title{Precision Estimator}
\usage{
precision(x)
}
\arguments{
\item{x}{A numeric vector for which to compute the Precision estimator.}
}
\description{
Computes the Precision estimator - defined as 2 * Spread(x) / sqrt(n), where n is
the sample size. This measure estimates the distance between two estimations of
independent random samples and provides a robust alternative to the standard error
of the mean.
}
\value{
A single numeric value representing the Precision estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
precision(x)

# Compare with standard error
x <- c(1, 2, 3, 4, 5, 6, 100)
sd(x) / sqrt(length(x))  # Standard error, affected by outlier
precision(x)             # Robust precision measure

}
