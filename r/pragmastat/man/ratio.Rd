\name{ratio}
\alias{ratio}
\title{Ratio Estimator}
\usage{
ratio(x, y)
}
\arguments{
\item{x}{A numeric vector representing the first sample (must be strictly positive).}
\item{y}{A numeric vector representing the second sample (must be strictly positive).}
}
\description{
Computes the Ratio estimator - the geometric median of pairwise ratios via log-space
aggregation: exp(Shift(log(x), log(y))). This is the multiplicative dual of Shift,
providing exact multiplicative antisymmetry: Ratio(x,y) * Ratio(y,x) = 1.
This measure estimates how many times larger x is compared to y and provides
a robust alternative to the ratio of means.
}
\value{
A single numeric value representing the Ratio estimation.
}
\examples{
# Basic usage
x <- c(2, 4, 6, 8, 10)
y <- c(1, 2, 3, 4, 5)
ratio(x, y)  # Should be around 2

# Compare with ratio of means
x <- c(2, 4, 6, 8, 1000)  # Contains outlier
y <- c(1, 2, 3, 4, 5)
mean(x) / mean(y)  # Affected by outlier
ratio(x, y)        # Robust to outlier

# When samples have similar scale
x <- c(1, 2, 3, 4, 5)
y <- c(1, 2, 3, 4, 5)
ratio(x, y)  # Should be close to 1

}
\references{
This estimator provides a robust measure of the multiplicative relationship
between two samples, resistant to outliers in either sample.
}