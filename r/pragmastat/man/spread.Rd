\name{spread}
\alias{spread}
\title{Spread Estimator}
\usage{
spread(x)
}
\arguments{
\item{x}{A numeric vector for which to compute the Spread estimator.}
}
\description{
Computes the Spread estimator - the median of all pairwise absolute differences |xi - xj|
where i < j. This measure estimates data dispersion (variability or scatter) and
provides a robust alternative to standard deviation.
This estimator is also known as the Shamos scale estimator.
}
\value{
A single numeric value representing the Spread estimation.
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