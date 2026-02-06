\name{median_bounds}
\alias{median_bounds}
\title{Distribution-Free Bounds for the Population Median}
\usage{
median_bounds(x, misrate)
}
\arguments{
\item{x}{Numeric vector of values (at least 2 elements).}
\item{misrate}{Misclassification rate, probability that the true median falls outside bounds. Must satisfy \code{misrate >= 2^(1-n)}.}
}
\description{
Provides distribution-free bounds for the population median using
binomial distribution to determine which order statistics form the bounds.
Requires only weak continuity assumption (no symmetry required).
}
\value{
A list with components \code{lower} and \code{upper}.
}
\seealso{
\code{\link{center_bounds}}, \code{\link{center_bounds_approx}}
}
\examples{
median_bounds(c(1, 2, 3, 4, 5), 0.1)

}
