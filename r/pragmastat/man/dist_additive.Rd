\name{dist_additive}
\alias{dist_additive}
\title{Additive (Normal) Distribution}
\description{
Create an additive (normal/Gaussian) distribution.
Uses the Box-Muller transform to generate samples.
}
\usage{
dist_additive(mean, std_dev)
}
\arguments{
\item{mean}{Location parameter (center of the distribution)}
\item{std_dev}{Scale parameter (standard deviation, must be positive)}
}
\value{
A Distribution object with the following methods:
\itemize{
  \item \code{sample(rng)}: Generate a single sample
  \item \code{samples(rng, count)}: Generate multiple samples
}
}
\examples{
r <- rng("demo-dist-additive")
dist <- dist_additive(0, 1)  # Standard normal
dist$sample(r)
}
