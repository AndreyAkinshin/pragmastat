\name{avg_spread}
\alias{avg_spread}
\title{Average Spread Estimator}
\usage{
avg_spread(x, y)
}
\arguments{
\item{x}{A numeric vector for the first sample.}
\item{y}{A numeric vector for the second sample.}
}
\description{
Computes the AvgSpread estimator - a weighted average of individual spreads
defined as (n * Spread(x) + m * Spread(y)) / (n + m), where n and m are the
sample sizes. This measure estimates the typical variability when considering
both samples together.
}
\value{
A single numeric value representing the Average Spread estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- x * 3

# Compare individual spreads
spread(x)
spread(y)
avg_spread(x, y)

}