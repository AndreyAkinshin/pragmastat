\name{med_ratio}
\alias{med_ratio}
\title{Median Ratio Estimator}
\usage{
med_ratio(x, y)
}
\arguments{
\item{x}{A numeric vector representing the first sample (numerator).}
\item{y}{A numeric vector representing the second sample (denominator).}
}
\description{
Computes the MedRatio estimator - the median of all pairwise ratios (xi / yj)
where xi comes from the first sample and yj from the second sample.
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
med_ratio(x, y)  # Should be around 2

# Compare with ratio of means
x <- c(2, 4, 6, 8, 1000)  # Contains outlier
y <- c(1, 2, 3, 4, 5)
mean(x) / mean(y)  # Affected by outlier
med_ratio(x, y)        # Robust to outlier

# When samples have similar scale
x <- c(1, 2, 3, 4, 5)
y <- c(1, 2, 3, 4, 5)
med_ratio(x, y)  # Should be close to 1

}
\references{
This estimator provides a robust measure of the multiplicative relationship
between two samples, resistant to outliers in either sample.
} 