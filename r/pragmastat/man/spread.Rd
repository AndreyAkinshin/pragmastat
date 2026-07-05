\name{spread}
\alias{spread}
\title{Spread Estimator}
\usage{
spread(x, assume_sorted = FALSE)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} for which to compute the Spread estimator.}

\item{assume_sorted}{If \code{TRUE}, assume a numeric vector input is already
sorted ascending and skip the internal sort. Ignored for \code{\link{Sample}}
input. Passing \code{TRUE} on unsorted input is undefined behavior.}
}
\description{
Computes the Spread estimator - the median of all pairwise absolute differences |xi - xj|
where i < j. This measure estimates data dispersion (variability or scatter) and
provides a robust alternative to standard deviation.
This estimator is also known as the Shamos scale estimator.
}
\value{
A \code{\link{Measurement}} when given \code{\link{Sample}} input,
or a single numeric value when given a numeric vector.
}
\examples{
# Single sample usage
x <- c(1, 2, 3, 4, 5, 6, 7)
spread(x)

# Compare with standard deviation
x <- c(1, 2, 3, 4, 5, 6, 273)
sd(x)      # Affected by outlier
spread(x)  # Robust to outlier

}
\references{
Shamos, M. I. (1976). Geometry and statistics: Problems at the interface.
} 