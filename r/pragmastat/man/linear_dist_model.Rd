\name{linear_dist_model}
\alias{linear_dist_model}
\title{Piecewise-Linear Distribution}
\description{
Reconstruct a distribution from a sample by interpolating the order statistics
at the anchors (i-1)/(n-1), which is the Hyndman-Fan type 7 rule (algorithm
\code{dist-linear-v1}). Support is [min(x), max(x)].

The model is immutable: the constructor copies and, unless the caller promises
otherwise, sorts the input, so repeated queries on a built model never observe
changing state.

The model takes either a native numeric vector or a metrology
\code{\link{Sample}}, the way every estimator does. The vector path is unitless.
The \code{Sample} path stores the sample's unit on \code{dist$unit} and exposes
the measured view \code{dist$measured}; because a \code{Sample} already keeps its
values ascending, that path never sorts again. A \strong{weighted} \code{Sample}
is refused: the reconstruction has no weighted form, and ignoring the weights
would answer a different question than the caller asked.
}
\usage{
linear_dist_model(values, assume_sorted = FALSE)
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
  \item \code{landscape()}: The staircase this model is -- cell boundaries, cumulative
    values and heights -- for a consumer of the local structure that wants the model's own
    grid rather than probing \code{density} repeatedly
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
dist <- linear_dist_model(c(1, 3, 5, 7, 9))
dist$quantile(c(0, 0.375, 1))
dist$cumulative(c(1, 4, 9))
dist$density(c(2, 8))

r <- rng("demo-dist-linear")
dist$samples(r, 3)

ns <- MeasurementUnit$new("ns", "Time", "ns", "nanosecond", 1)
measured <- linear_dist_model(Sample$new(c(1, 3, 5, 7, 9), unit = ns))
measured$measured$quantile(0.375)$value              # 4
measured$measured$density(Measurement$new(4, ns))     # 0.125
}
\seealso{
\code{\link{neighbor_dist_model}}
}
