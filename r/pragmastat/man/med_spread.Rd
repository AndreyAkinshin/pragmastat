\name{med_spread}
\alias{med_spread}
\title{Median Spread Estimator}
\usage{
med_spread(x, y)
}
\arguments{
\item{x}{A numeric vector for the first sample.}
\item{y}{A numeric vector for the second sample.}
}
\description{
Computes the MedSpread estimator - a weighted average of individual spreads
defined as (n * Spread(x) + m * Spread(y)) / (n + m), where n and m are the
sample sizes. This measure estimates the typical variability when considering
both samples together.
}
\value{
A single numeric value representing the Median Spread estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- x * 3

# Compare individual spreads
spread(x)
spread(y)
med_spread(x, y)

}
