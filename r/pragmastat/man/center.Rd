\name{center}
\alias{center}
\title{Center Estimator}
\usage{
center(x)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} for which to compute the Center estimator.}
}
\description{
Computes the Center estimator - the median of all pairwise averages (xi + xj)/2
where i <= j. This measure estimates the central value of the data and is more
robust than the mean while being more efficient than the median.
This estimator is also known as the Hodges-Lehmann location estimator.
}
\value{
A \code{\link{Measurement}} when given \code{\link{Sample}} input,
or a single numeric value when given a numeric vector.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5, 6, 7)
center(x)

# Compare with mean and median
x <- c(1, 2, 3, 4, 5, 6, 273)
mean(x)    # Affected by outlier
median(x)  # Robust to outlier
center(x)  # Also robust to outlier

}
\references{
Hodges, J. L., & Lehmann, E. L. (1963). Estimates of location based on rank tests.
The Annals of Mathematical Statistics, 34(2), 598-611.
}
