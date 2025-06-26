\name{med_disparity}
\alias{med_disparity}
\title{Median Disparity Estimator}
\usage{
med_disparity(x, y)
}
\arguments{
\item{x}{A numeric vector representing the first sample.}
\item{y}{A numeric vector representing the second sample.}
}
\description{
Computes the MedDisparity estimator - defined as MedShift(x,y) / MedSpread(x,y).
This measure provides effect size: a normalized absolute difference between x and y
expressed in standardized spread units. It serves as a robust alternative to
traditional effect size measures like Cohen's d.
}
\value{
A single numeric value representing the Disparity estimation.
}
\examples{
# Basic usage
x <- c(1, 2, 3, 4, 5)
y <- c(6, 7, 8, 9, 10)
med_disparity(x, y)  # Large disparity due to clear separation

# Overlapping samples
x <- c(1, 2, 3, 4, 5)
y <- c(3, 4, 5, 6, 7)
med_disparity(x, y)  # Smaller disparity due to overlap

# Identical samples
x <- c(1, 2, 3, 4, 5)
y <- c(1, 2, 3, 4, 5)
med_disparity(x, y)  # Should be close to 0

# With outliers
x <- c(1, 2, 3, 4, 500)
y <- c(6, 7, 8, 9, 10)
med_disparity(x, y)  # Robust to outliers

}
\references{
This estimator provides a robust alternative to traditional effect size measures,
combining the robustness of median-based estimators with standardization.
} 