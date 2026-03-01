\name{spread_bounds}
\alias{spread_bounds}
\title{Distribution-Free Bounds for Spread}
\usage{
spread_bounds(x, misrate = DEFAULT_MISRATE, seed = NULL)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} (at least 2 elements).}
\item{misrate}{Misclassification rate. Must satisfy \code{misrate >= 2^(1 - floor(n/2))}.}
\item{seed}{Optional string seed for deterministic randomization.}
}
\description{
Provides distribution-free bounds for the Spread using disjoint pairs with sign-test inversion.
Randomizes the cutoff between adjacent ranks to match the requested misrate.
}
\details{
The algorithm draws a random disjoint pairing of the sample, computes absolute differences
within each pair, and inverts a sign test on these differences to obtain bounds.
The cutoff is randomized between adjacent ranks so that the actual misrate matches
the requested value exactly under weak continuity.

The misrate must satisfy \code{misrate >= 2^(1 - floor(n/2))}.
For small samples, achievable misrates are limited by the discrete sign-test distribution.

With ties or small samples, the bounds may be conservative (wider than necessary).
Conservatism decreases as the sample size grows.
}
\value{
A \code{\link{Bounds}} object when given \code{\link{Sample}} input,
or a list with components \code{lower} and \code{upper} when given a numeric vector.
}
\seealso{
\code{\link{spread}} for the point estimator.
}
\examples{
# Basic usage with natural sequence
spread_bounds(c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), 0.5, seed = "test")

# Tighter misrate produces wider bounds
bounds <- spread_bounds(1:30, 0.01, seed = "demo")
bounds$lower
bounds$upper

}
