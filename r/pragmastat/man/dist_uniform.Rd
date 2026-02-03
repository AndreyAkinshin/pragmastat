\name{dist_uniform}
\alias{dist_uniform}
\title{Uniform Distribution}
\description{
Create a uniform distribution on [min, max).
}
\usage{
dist_uniform(min_val, max_val)
}
\arguments{
\item{min_val}{Lower bound (inclusive)}
\item{max_val}{Upper bound (exclusive, must be greater than min_val)}
}
\value{
A Distribution object with the following methods:
\itemize{
  \item \code{sample(rng)}: Generate a single sample
  \item \code{samples(rng, count)}: Generate multiple samples
}
}
\examples{
r <- rng("demo-dist-uniform")
dist <- dist_uniform(0, 10)
dist$sample(r)
}
