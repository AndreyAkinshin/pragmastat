\name{center_bounds}
\alias{center_bounds}
\title{Exact Bounds for Center (Hodges-Lehmann Pseudomedian)}
\usage{
center_bounds(x, misrate = DEFAULT_MISRATE, assume_sorted = FALSE)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} (at least 2 elements).}
\item{misrate}{Misclassification rate, probability that the true center falls outside bounds. Must satisfy \code{misrate >= 2^(1-n)}.}

\item{assume_sorted}{If \code{TRUE}, assume a numeric vector input is already
sorted ascending and skip the internal sort. Ignored for \code{\link{Sample}}
input. Passing \code{TRUE} on unsorted input is undefined behavior.}
}
\description{
Provides exact distribution-free bounds for the Center (Hodges-Lehmann pseudomedian).
Uses SignedRankMargin to determine which pairwise averages form the bounds.
Requires weak symmetry and weak continuity assumptions.
}
\value{
A \code{\link{Bounds}} object when given \code{\link{Sample}} input,
or a list with components \code{lower} and \code{upper} when given a numeric vector.
}
\seealso{
\code{\link{center}}
}
\examples{
center_bounds(c(1, 2, 3, 4, 5), 0.1)

}
