\name{Sample}
\alias{Sample}
\alias{*.Sample}
\alias{+.Sample}
\title{Validated Numeric Sample}
\description{
An R6 class representing a validated collection of numeric values with optional
weights and a \code{\link{MeasurementUnit}}. Validates input at construction time:
empty vectors, NaN, Inf, and -Inf are rejected.

Original value order is preserved in \code{values}. A sorted copy is available
via \code{sorted_values} (lazily computed and cached on first access).

Arithmetic operators \code{*} and \code{+} are supported via S3 methods for
scalar transformations (e.g. \code{sample * 1000}).
}
\section{Constructor}{
\describe{
\item{\code{Sample$new(values, weights = NULL, unit = NULL, subject = "x")}}{
  \describe{
    \item{\code{values}}{Numeric vector (must be non-empty, finite, no NaN).}
    \item{\code{weights}}{Optional numeric vector of non-negative weights (same length as \code{values}).}
    \item{\code{unit}}{\code{\link{MeasurementUnit}} (defaults to \code{\link{number_unit}}).}
    \item{\code{subject}}{Subject label for error messages (\code{"x"} or \code{"y"}).}
  }
}
}
}
\section{Active Bindings}{
\describe{
\item{\code{values}}{Numeric vector. Original values in input order.}
\item{\code{sorted_values}}{Numeric vector. Lazily computed sorted copy (cached after first access).}
\item{\code{weights}}{Numeric vector or \code{NULL}. Weights vector if weighted, \code{NULL} otherwise.}
\item{\code{size}}{Integer. Number of values.}
\item{\code{is_weighted}}{Logical. \code{TRUE} if sample has weights.}
\item{\code{total_weight}}{Numeric. Sum of weights (1.0 for unweighted samples).}
\item{\code{weighted_size}}{Numeric. Kish's effective sample size.}
\item{\code{unit}}{\code{\link{MeasurementUnit}}. The measurement unit.}
\item{\code{subject}}{Character. Subject label (\code{"x"} or \code{"y"}).}
}
}
\section{Methods}{
\describe{
\item{\code{convert_to(target)}}{Convert values to a compatible target unit. Returns a new Sample.}
\item{\code{log_transform()}}{Log-transform values (all must be positive). Returns a new Sample with \code{\link{number_unit}}.}
\item{\code{with_subject(subject)}}{Return a new Sample with a different subject label.}
}
}
\examples{
# Basic usage
x <- Sample$new(c(5.6, 1.2, 3.4))
x$values         # c(5.6, 1.2, 3.4) - original order
x$sorted_values  # c(1.2, 3.4, 5.6) - lazy sorted copy
x$size           # 3

# With unit
ms <- MeasurementUnit$new("ms", "time", "ms", "milliseconds", 1)
x <- Sample$new(c(100, 200, 300), unit = ms)
m <- center(x)
m$unit$id  # "ms"

# Scalar arithmetic
x2 <- x * 0.001  # scale values
}
\seealso{
\code{\link{MeasurementUnit}}, \code{\link{Measurement}}, \code{\link{center}}, \code{\link{spread}}
}
