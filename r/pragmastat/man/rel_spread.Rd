\name{rel_spread}
\alias{rel_spread}
\title{Relative Spread Estimator}
\usage{
rel_spread(x)
}
\arguments{
\item{x}{A numeric vector for which to compute the RelSpread estimator.}
}
\description{
Computes the RelSpread estimator - the ratio of Spread to absolute Center (Spread(x) / |Center(x)|).
This measure provides the relative dispersion of a sample and serves as a robust
alternative to the coefficient of variation.
}
\value{
A single numeric value representing the RelSpread estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
rel_spread(x)

# Compare with coefficient of variation
x <- c(1, 2, 3, 4, 5, 6, 100)
sd(x) / mean(x)  # Traditional CV, affected by outlier
rel_spread(x)    # Robust relative spread measure

}