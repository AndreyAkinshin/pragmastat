\name{pairwise_margin}
\alias{pairwise_margin}
\title{Pairwise Margin for Shift Bounds}
\usage{
pairwise_margin(n, m, misrate)
}
\arguments{
\item{n}{Sample size of the first sample (must be positive).}
\item{m}{Sample size of the second sample (must be positive).}
\item{misrate}{Misclassification rate, a value in the range [0, 1].}
}
\description{
Determines how many extreme pairwise differences to exclude when constructing
bounds based on the distribution of dominance statistics.
Uses exact calculation for small samples (n+m <= 400) and Edgeworth
approximation for larger samples.
}
\value{
An integer representing the total margin split between lower and upper tails.
}
\examples{
# Compute margin for small samples
pairwise_margin(10, 10, 0.05)

# Compute margin for larger samples (uses Edgeworth approximation)
pairwise_margin(300, 300, 0.05)

}
\references{
Loeffler, H. (1982). Ueber eine Partition der nat. Zahlen und ihre Anwendung beim U-Test.
Metrika, 29(1), 55-61.
}
