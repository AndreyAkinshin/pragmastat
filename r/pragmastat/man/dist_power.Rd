\name{dist_power}
\alias{dist_power}
\title{Power (Pareto) Distribution}
\description{
Create a power (Pareto) distribution with given minimum and shape parameters.
}
\usage{
dist_power(min_val, shape)
}
\arguments{
\item{min_val}{Minimum value (scale parameter, must be positive)}
\item{shape}{Shape parameter (alpha, must be positive)}
}
\value{
A Distribution object with the following methods:
\itemize{
  \item \code{sample(rng)}: Generate a single sample
  \item \code{samples(rng, count)}: Generate multiple samples
}
}
\examples{
r <- rng("demo-dist-power")
dist <- dist_power(1, 2)
dist$sample(r)
}
