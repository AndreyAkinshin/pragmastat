\name{volatility}
\alias{volatility}
\title{Volatility Estimator}
\usage{
volatility(x)
}
\arguments{
\item{x}{A numeric vector for which to compute the Volatility estimator.}
}
\description{
Computes the Volatility estimator - the ratio of Spread to absolute Center (Spread(x) / |Center(x)|).
This measure provides the relative dispersion of a sample and serves as a robust
alternative to the coefficient of variation.
}
\value{
A single numeric value representing the Volatility estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
volatility(x)

# Compare with coefficient of variation
x <- c(1, 2, 3, 4, 5, 6, 100)
sd(x) / mean(x)  # Traditional CV, affected by outlier
volatility(x)    # Robust volatility measure

}