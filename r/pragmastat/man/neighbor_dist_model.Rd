\name{neighbor_dist_model}
\alias{neighbor_dist_model}
\title{Local Step-Density Distribution}
\description{
Reconstruct a distribution from a sample by giving each observation a cell whose
height reflects how tightly its nearest neighbors are packed, so the
reconstruction follows local structure instead of imposing a single global shape
(algorithm \code{dist-neighbor-v1}). The window is indexed by k0 = ceil(sqrt(n))
with radius r = floor(k0 / 2); windows near the ends are truncated and the
numerator is always the actual neighbor count.

The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry
estimator: support is bounded by the observed range and the cell masses are
explicitly normalized, so the density integrates to one.

The model takes either a native numeric vector or a metrology
\code{\link{Sample}}, the way every estimator does. The vector path is unitless.
The \code{Sample} path stores the sample's unit on \code{dist$unit} and exposes
the measured view \code{dist$measured}; because a \code{Sample} already keeps its
values ascending, that path never sorts again. A \strong{weighted} \code{Sample}
is refused: the reconstruction has no weighted form, and ignoring the weights
would answer a different question than the caller asked.
}
\usage{
neighbor_dist_model(values, assume_sorted = FALSE)
}
\arguments{
\item{values}{Fitting vector or \code{\link{Sample}}; must be non-empty, finite,
of size at least two, with a finite representable range and pairwise distinct
elements.}

\item{assume_sorted}{If \code{TRUE}, the caller promises the input is already
ascending and the internal copy is not sorted. Passing an unsorted vector under
this flag is a contract violation with an unspecified but terminating result.
It applies to the vector path only: a \code{Sample} carries its own ascending
promise, so supplying this argument alongside one is an error.}
}
\value{
A Distribution object with the following methods:
\itemize{
  \item \code{quantile_at(p)}, \code{cumulative_at(x)}, \code{density_at(x)}: the scalar
    contract every parametric family also answers, at a single point
  \item \code{quantile(cumulances)}: Values at the requested cumulances
  \item \code{cumulative(values)}: Probabilities of not exceeding the requested values
  \item \code{density(values)}: Densities at the requested values
  \item \code{sample(rng)}: Generate a single sample
  \item \code{samples(rng, count)}: Generate multiple samples
  \item \code{landscape()}: The staircase this model is -- k0, radius, cell boundaries,
    cumulative values, masses, heights and source indices -- for a consumer of the local
    structure that wants the model's own grid rather than probing \code{density} repeatedly
}
Each vector query returns a fresh numeric vector in the order the query was given, and
accepts an empty query.

The model also carries \code{unit} and the measured view \code{measured}, whose
three queries carry the dimension their result actually has:
\itemize{
  \item \code{measured$quantile(cumulances)}: a \code{\link{Measurement}} in the
    unit of the sample (a cumulance is dimensionless, so the argument stays a
    plain numeric vector)
  \item \code{measured$cumulative(values)}: a plain numeric vector of
    probabilities, from a \code{\link{Measurement}} of values
  \item \code{measured$density(values)}: a plain numeric vector, from a
    \code{\link{Measurement}} of values -- a density is a terminal value,
    never converted or combined, so it stays a plain number
}
A measured query given in a compatible but different unit is converted; an
incompatible unit is an error.
}
\examples{
dist <- neighbor_dist_model(c(1, 2, 3, 4, 5))
dist$density(c(1, 3, 5))
dist$quantile(c(0, 0.5, 1))

r <- rng("demo-dist-neighbor")
dist$samples(r, 3)

ns <- MeasurementUnit$new("ns", "Time", "ns", "nanosecond", 1)
measured <- neighbor_dist_model(Sample$new(c(1, 2, 3, 4, 5), unit = ns))
measured$measured$quantile(0.5)$value                # 3
measured$measured$density(Measurement$new(3, ns))     # 0.2307692
}
\seealso{
\code{\link{linear_dist_model}}
}
