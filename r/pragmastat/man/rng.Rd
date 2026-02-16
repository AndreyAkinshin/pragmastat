\name{rng}
\alias{rng}
\alias{Rng}
\title{Deterministic Random Number Generator}
\description{
A deterministic PRNG based on xoshiro256++ that produces identical sequences
across all Pragmastat language implementations when initialized with the same seed.
}
\usage{
rng(seed = NULL)
}
\arguments{
\item{seed}{Integer seed, string seed, or NULL for system time}
}
\value{
An Rng object (R6 class) with the following methods:
\itemize{
  \item \code{uniform_float()}: Generate a uniform random float in [0, 1)
  \item \code{uniform_float_range(min_val, max_val)}: Generate a uniform random float in [min, max)
  \item \code{uniform_int(min_val, max_val)}: Generate a uniform random integer in [min, max)
  \item \code{uniform_bool()}: Generate a uniform random boolean
  \item \code{shuffle(x)}: Return a shuffled copy of the input vector
  \item \code{sample(x, k)}: Sample k elements from the input vector without replacement
  \item \code{resample(x, k)}: Resample k elements with replacement (bootstrap sampling)
}
}
\examples{
# Create from string seed
r <- rng("demo-uniform")
r$uniform_float()

# Shuffle a vector
r <- rng("demo-shuffle")
r$shuffle(1:5)

# Sample k elements
r <- rng("demo-sample")
r$sample(0:9, 3)

# Bootstrap resample
r <- rng("demo-resample")
r$resample(1:5, 5)
}
