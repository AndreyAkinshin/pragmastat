\name{standard_registry}
\alias{standard_registry}
\title{Create a Standard Unit Registry}
\usage{
standard_registry()
}
\description{
Creates a \code{\link{UnitRegistry}} pre-populated with the three standard units:
\code{\link{number_unit}}, \code{\link{ratio_unit}}, and \code{\link{disparity_unit}}.
}
\value{
A \code{\link{UnitRegistry}} instance.
}
\examples{
reg <- standard_registry()
reg$resolve("number")$full_name    # "Number"
reg$resolve("ratio")$full_name     # "Ratio"
reg$resolve("disparity")$full_name # "Disparity"
}
\seealso{
\code{\link{UnitRegistry}}, \code{\link{number_unit}}
}
