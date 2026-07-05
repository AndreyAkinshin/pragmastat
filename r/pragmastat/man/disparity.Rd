\name{disparity}
\alias{disparity}
\title{Disparity Estimator}
\usage{
disparity(x, y, assume_sorted = FALSE)
}
\arguments{
\item{x}{A numeric vector or \code{\link{Sample}} representing the first sample.}
\item{y}{A numeric vector or \code{\link{Sample}} representing the second sample.}

\item{assume_sorted}{If \code{TRUE}, assume numeric vector inputs are already
sorted ascending and skip the internal sort. Ignored for \code{\link{Sample}}
input. Passing \code{TRUE} on unsorted input is undefined behavior.}
}
\description{
Computes the Disparity estimator - defined as Shift(x,y) / AvgSpread(x,y).
This measure provides effect size: a normalized difference between x and y
expressed in standardized spread units. It serves as a robust alternative to
traditional effect size measures like Cohen's d.
}
\value{
A \code{\link{Measurement}} when given \code{\link{Sample}} input,
or a single numeric value when given numeric vectors.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- c(6, 7, 8, 9, 10)
disparity(x, y)  # Large disparity due to clear separation

# Overlapping samples
x <- c(1, 2, 3, 4, 5)
y <- c(3, 4, 5, 6, 7)
disparity(x, y)  # Smaller disparity due to overlap

# Identical samples
x <- c(1, 2, 3, 4, 5)
y <- c(1, 2, 3, 4, 5)
disparity(x, y)  # Should be close to 0

# With outliers
x <- c(1, 2, 3, 4, 500)
y <- c(6, 7, 8, 9, 10)
disparity(x, y)  # Robust to outliers

}
\references{
This estimator provides a robust alternative to traditional effect size measures,
combining the robustness of median-based estimators with standardization.
}