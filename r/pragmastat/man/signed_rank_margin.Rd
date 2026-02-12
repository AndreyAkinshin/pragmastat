\name{signed_rank_margin}
\alias{signed_rank_margin}
\title{Signed Rank Margin for One-Sample Bounds}
\usage{
signed_rank_margin(n, misrate)
}
\arguments{
\item{n}{Sample size (at least 2).}
\item{misrate}{Misclassification rate. Must satisfy \code{misrate >= 2^(1-n)}.}
}
\description{
Computes the margin for one-sample signed-rank bounds.
Uses Wilcoxon signed-rank distribution to determine the margin that achieves
the specified misrate.
Uses exact calculation for n <= 63 and Edgeworth approximation for larger n.
}
\value{
An integer representing the total margin.
}
\seealso{
\code{\link{center_bounds}}
}
\keyword{internal}
