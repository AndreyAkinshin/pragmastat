\name{center_bounds_approx}
\alias{center_bounds_approx}
\title{Bootstrap-Based Nominal Bounds for Center}
\usage{
center_bounds_approx(x, misrate, seed = NULL)
}
\arguments{
\item{x}{Numeric vector of values (at least 2 elements).}
\item{misrate}{Misclassification rate, probability that the true center falls outside bounds. Must satisfy \code{misrate >= max(2/10000, 2^(1-n))}.}
\item{seed}{Optional seed string for deterministic results. When NULL (default), uses the fixed seed "center-bounds-approx" for cross-language reproducibility.}
}
\description{
Provides bootstrap-based nominal bounds for Center (Hodges-Lehmann pseudomedian).
No symmetry requirement, but provides only nominal (not exact) coverage.
Bootstrap percentile method has known undercoverage for small samples.
}
\value{
A list with components \code{lower} and \code{upper}.
}
\seealso{
\code{\link{center}}, \code{\link{center_bounds}}, \code{\link{median_bounds}}
}
\examples{
center_bounds_approx(c(1, 2, 3, 4, 5), 0.1, "demo-seed")

}
