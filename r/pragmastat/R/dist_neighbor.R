#' Local Step-Density Distribution
#'
#' Reconstruct a distribution from a sample by giving each observation a cell
#' whose height reflects how tightly its nearest neighbors are packed, so the
#' reconstruction follows local structure instead of imposing a single global
#' shape (algorithm \code{dist-neighbor-v1}). The window is indexed by
#' k0 = ceil(sqrt(n)) with radius r = floor(k0 / 2); windows near the ends are
#' truncated and the numerator is always the actual neighbor count.
#'
#' The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry
#' estimator: support is bounded by the observed range and the cell masses are
#' explicitly normalized, so the density integrates to one.
#'
#' The model takes either a native numeric vector or a metrology \code{Sample},
#' the way every estimator does. The vector path is unitless; the \code{Sample}
#' path stores the sample's unit on the model and exposes the measured view
#' \code{dist$measured}. A \code{Sample} already keeps its values ascending, so
#' that path never sorts again, and a weighted \code{Sample} is refused: the
#' reconstruction has no weighted form, and ignoring the weights would answer a
#' different question than the caller asked.
#'
#' @param values Fitting vector or \code{Sample}; must be non-empty, finite, of
#'   size at least two, with a finite representable range and pairwise distinct
#'   elements
#' @param assume_sorted If TRUE, the caller promises the input is already
#'   ascending and the internal copy is not sorted. Passing an unsorted vector
#'   under this flag is a contract violation with an unspecified but terminating
#'   result. It applies to the vector path only: a \code{Sample} carries its own
#'   ascending promise, so supplying this argument alongside one is an error.
#' @return A Distribution object
#'
#' @examples
#' dist <- neighbor_dist_model(c(1, 2, 3, 4, 5))
#' dist$density(c(1, 3, 5))
#'
#' @export
neighbor_dist_model <- function(values, assume_sorted = FALSE) {
  if (inherits(values, "Sample")) {
    check_no_assume_sorted(missing(assume_sorted))
    return(neighbor_dist_impl_measured(values))
  }
  check_dist_validity(values, SUBJECTS$X)
  neighbor_dist_impl_trusted(values, assume_sorted)
}

# Builds the model from a vector whose validity the caller has already checked.
neighbor_dist_impl_trusted <- function(values, assume_sorted) {
  neighbor_dist_impl(dist_prepare(values, assume_sorted))
}

# Builds the model from a measured sample. Sample construction already enforced
# validity and the values are ascending, so only the weight refusal is left.
neighbor_dist_impl_measured <- function(x) {
  check_non_weighted(SUBJECTS$X, x)
  neighbor_dist_impl(dist_prepare(x$sorted_values, TRUE), x$unit)
}

# Model over a prepared (ascending, distinct) private copy.
neighbor_dist_impl <- function(x, unit = number_unit) {
  n <- length(x)
  scale_index <- dist_ceil_sqrt(n)
  radius <- scale_index %/% 2L

  positions <- seq_len(n)
  left <- pmax(1L, positions - radius)
  right <- pmin(n, positions + radius)
  counts <- right - left + 1L
  spans <- x[right] - x[left]

  e <- numeric(n + 1L)
  e[1L] <- x[1L]
  for (i in seq_len(n - 1L)) {
    e[i + 1L] <- dist_midpoint(x[i], x[i + 1L])
  }
  e[n + 1L] <- x[n]

  # Rounding can collapse the midpoint of two adjacent binary64 observations
  # onto a neighboring boundary. Such a cell carries no mass and is dropped;
  # the surviving cells keep the index of the observation they came from.
  widths <- e[-1L] - e[-(n + 1L)]
  surviving <- widths != 0.0
  sources <- (positions - 1L)[surviving]
  b <- c(e[1L], e[-1L][surviving])
  s <- length(sources)

  a <- counts[surviving] / n * (widths[surviving] / spans[surviving])

  total <- 0.0
  for (j in seq_len(s)) {
    total <- total + a[j]
  }
  # A broken assume_sorted promise can make a cell wider than the window it normalizes against,
  # so its unnormalized mass overflows, or it can collapse every cell and leave a zero total.
  # Either way the divisor is unusable and NaN would reach the query kernels. The sample itself
  # satisfies domain(x) and distinctness(x), so this is not an assumption violation: it is a
  # numerical-conformance failure, reported through the same channel dist_unit_ratio uses.
  # For a genuinely ascending sample the total is finite and strictly positive.
  # It is not bounded by 1: the total is a normalizer, not a probability. It sits above 1 and
  # decays toward it as n grows.
  if (!is.finite(total) || total <= 0.0) {
    stop(paste0(
      "Numerical conformance failure: the neighbor_dist_model normalizer is not usable; ",
      "this indicates an assume_sorted promise that the input does not satisfy"
    ))
  }

  # Reserve one representable step for every cell that has not been placed yet,
  # so a mass too small to register at this point in the prefix still advances
  # the cumulative value instead of flattening it into a plateau.
  cap <- numeric(s + 1L)
  cap[s + 1L] <- 1.0
  for (j in s:1L) {
    cap[j] <- dist_next_down(cap[j + 1L])
  }

  f <- numeric(s + 1L)
  f[1L] <- 0.0
  for (j in seq_len(s - 1L)) {
    raw <- f[j] + a[j] / total
    f[j + 1L] <- min(cap[j + 1L], max(dist_next_up(f[j]), raw))
  }
  f[s + 1L] <- 1.0

  p <- f[-1L] - f[-(s + 1L)]
  d <- p / (b[-1L] - b[-(s + 1L)])

  landscape <- list(
    scale_index = scale_index,
    radius = radius,
    boundaries = b,
    cumulative = f,
    masses = p,
    densities = d,
    sources = sources,
    cell_count = s
  )

  # NaN first, for the same reason linear_dist_model's quantile_at checks it: every branch
  # below compares false against it, and the fallback would otherwise depend on how the
  # platform converts a NaN cumulance to an index.
  quantile_at <- function(c) {
    if (is.na(c)) {
      return(c)
    }
    if (c <= 0.0) {
      value <- b[1L]
    } else if (c >= 1.0) {
      value <- b[s + 1L]
    } else {
      j <- dist_lower_bound(f, 1L, s + 1L, c) - 1L
      t <- dist_unit_ratio(c - f[j + 1L], p[j + 1L])
      value <- dist_lerp(b[j + 1L], b[j + 2L], t)
    }
    # Every branch hands back a value built from the sample's own boundaries -- the clamped
    # ones return one verbatim, and dist_lerp reproduces its endpoints exactly -- so a -0.0
    # among the observations would otherwise reach the caller unnormalized.
    normalize_zero(value)
  }

  cumulative_at <- function(v) {
    if (is.na(v)) {
      return(v)
    }
    if (v <= b[1L]) {
      return(0.0)
    }
    if (v >= b[s + 1L]) {
      return(1.0)
    }
    j <- dist_upper_bound(b, v) - 1L
    t <- dist_unit_ratio(v - b[j + 1L], b[j + 2L] - b[j + 1L])
    dist_lerp(f[j + 1L], f[j + 2L], t)
  }

  density_at <- function(v) {
    if (is.na(v)) {
      return(v)
    }
    if (v < b[1L] || v > b[s + 1L]) {
      return(0.0)
    }
    j <- if (v == b[s + 1L]) s - 1L else dist_upper_bound(b, v) - 1L
    p[j + 1L] / (b[j + 2L] - b[j + 1L])
  }

  quantile_trusted <- function(cumulances) {
    vapply(as.double(cumulances), quantile_at, numeric(1))
  }

  cumulative_trusted <- function(values) {
    vapply(as.double(values), cumulative_at, numeric(1))
  }

  density_trusted <- function(values) {
    vapply(as.double(values), density_at, numeric(1))
  }

  quantile_checked <- function(cumulances) {
    check_query_validity(cumulances)
    check_cumulance(cumulances)
    quantile_trusted(cumulances)
  }

  cumulative_checked <- function(values) {
    check_query_validity(values)
    cumulative_trusted(values)
  }

  density_checked <- function(values) {
    check_query_validity(values)
    density_trusted(values)
  }

  sample_fn <- function(rng) {
    quantile_at(rng$uniform_float())
  }

  list(
    algorithm = "dist-neighbor-v1",
    cell_count = s,
    support_min = b[1L],
    support_max = b[s + 1L],
    unit = unit,
    quantile_at = quantile_at,
    cumulative_at = cumulative_at,
    density_at = density_at,
    quantile = quantile_checked,
    cumulative = cumulative_checked,
    density = density_checked,
    # The staircase this model is: k0, radius, cell boundaries, cumulative values, masses,
    # heights and source indices. Point evaluation answers what the density is at a value a
    # caller picked; this answers what the density *is*, on the boundaries the model itself
    # uses. Handing the vectors out cannot mutate the model: R copies them on write.
    landscape = function() landscape,
    # ── Measured facade ─────────────────────────────────────────────────────
    #
    # The metrology path, alongside the numeric one, exactly as every estimator
    # offers both. Only quantile carries a unit -- the sample's own; cumulative
    # and density are plain numbers.
    #
    # It lives behind its own accessor rather than replacing the numeric
    # methods: a cumulance is a plain number in R, so quantile takes the same
    # argument in both worlds and only the result differs. R cannot dispatch on
    # a return type, so the caller states which world the call belongs to.
    measured = list(
      quantile = function(cumulances) {
        dist_to_measurement(quantile_checked(cumulances), unit)
      },
      cumulative = function(values) {
        cumulative_checked(dist_to_raw(values, unit))
      },
      density = function(values) {
        density_checked(dist_to_raw(values, unit))
      }
    ),
    sample = sample_fn,
    samples = function(rng, count) {
      vapply(seq_len(count), function(i) sample_fn(rng), numeric(1))
    },
    # Query entry points for callers that have already run the assumptions in
    # the canonical order. Internal seam, not public API.
    .quantile_trusted = quantile_trusted,
    .cumulative_trusted = cumulative_trusted,
    .density_trusted = density_trusted
  )
}
