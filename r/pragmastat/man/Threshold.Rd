\name{Threshold}
\alias{Threshold}
\title{Compare Threshold}
\description{
An R6 class representing a threshold for confirmatory comparison.
Encapsulates a metric name, threshold value, and misclassification rate.
Used as input to \code{\link{compare1}} and \code{\link{compare2}}.
}
\section{Fields}{
\describe{
\item{\code{metric}}{Character. Metric name (e.g. \code{"center"}, \code{"shift"}).}
\item{\code{value}}{Numeric or \code{\link{Measurement}}. Threshold value (must be finite).}
\item{\code{misrate}}{Numeric. Misclassification rate in (0, 1]. Defaults to 0.001.}
}
}
\section{Methods}{
\describe{
\item{\code{initialize(metric, value, misrate = 0.001)}}{Create a new Threshold with validation.}
}
}
\examples{
t <- Threshold$new("shift", 1.0)
t$metric  # "shift"
t$value   # 1.0
t$misrate # 0.001
}
\seealso{
\code{\link{Projection}}, \code{\link{compare1}}, \code{\link{compare2}}
}
