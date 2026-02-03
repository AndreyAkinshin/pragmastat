\name{dist_exp}
\alias{dist_exp}
\title{Exponential Distribution}
\description{
Create an exponential distribution with given rate parameter.
The mean of this distribution is 1/rate.
}
\usage{
dist_exp(rate)
}
\arguments{
\item{rate}{Rate parameter (lambda, must be positive)}
}
\value{
A Distribution object with the following methods:
\itemize{
  \item \code{sample(rng)}: Generate a single sample
  \item \code{samples(rng, count)}: Generate multiple samples
}
}
\examples{
r <- rng("demo-dist-exp")
dist <- dist_exp(1)  # rate=1, mean=1
dist$sample(r)
}
