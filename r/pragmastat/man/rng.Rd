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
  \item \code{uniform()}: Generate a uniform random float in [0, 1)
  \item \code{uniform_int(min_val, max_val)}: Generate a uniform random integer in [min, max)
  \item \code{shuffle(x)}: Return a shuffled copy of the input vector
  \item \code{sample(x, k)}: Sample k elements from the input vector without replacement
}
}
\examples{
# Create from integer seed
r <- rng(1729)
r$uniform()

# Create from string seed
r <- rng("experiment-1")

# Shuffle a vector
r <- rng(1729)
r$shuffle(1:5)

# Sample k elements
r <- rng(1729)
r$sample(0:9, 3)
}
