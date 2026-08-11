#' Piecewise-Linear Distribution
#'
#' Reconstruct a distribution from a sample by interpolating the order
#' statistics at the anchors (i-1)/(n-1), which is the Hyndman-Fan type 7 rule
#' (algorithm \code{dist-linear-v1}). Support is [min(x), max(x)].
#'
#' The model is immutable: the constructor copies and, unless the caller
#' promises otherwise, sorts the input, so repeated queries on a built model
#' never observe changing state.
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
#' dist <- linear_dist_model(c(1, 3, 5, 7, 9))
#' dist$quantile(c(0, 0.375, 1))
#'
#' @export
linear_dist_model <- function(values, assume_sorted = FALSE) {
  if (inherits(values, "Sample")) {
    check_no_assume_sorted(missing(assume_sorted))
    return(linear_dist_impl_measured(values))
  }
  check_dist_validity(values, SUBJECTS$X)
  linear_dist_impl_trusted(values, assume_sorted)
}

# Builds the model from a vector whose validity the caller has already checked.
linear_dist_impl_trusted <- function(values, assume_sorted) {
  linear_dist_impl(dist_prepare(values, assume_sorted))
}

# Builds the model from a measured sample. Sample construction already enforced
# validity and the values are ascending, so only the weight refusal is left.
linear_dist_impl_measured <- function(x) {
  check_non_weighted(SUBJECTS$X, x)
  linear_dist_impl(dist_prepare(x$sorted_values, TRUE), x$unit)
}

# Model over a prepared (ascending, distinct) private copy.
linear_dist_impl <- function(x, unit = number_unit) {
  n <- length(x)

  # NaN first, for the same reason dist_additive's quantile_at checks it: every branch below
  # compares false against it, and the fallback would otherwise depend on how the platform
  # converts a NaN cumulance to an index.
  quantile_at <- function(c) {
    if (is.na(c)) {
      return(c)
    }
    if (c <= 0.0) {
      value <- x[1L]
    } else if (c >= 1.0) {
      value <- x[n]
    } else {
      r <- (n - 1L) * c
      j <- as.integer(floor(r))
      g <- r - j
      value <- dist_lerp(x[j + 1L], x[j + 2L], g)
    }
    # Every branch hands back a value the sample supplied -- the clamped ones return an order
    # statistic verbatim, and dist_lerp reproduces its endpoints exactly -- so a -0.0 among the
    # observations would otherwise reach the caller unnormalized.
    normalize_zero(value)
  }

  cumulative_at <- function(v) {
    if (is.na(v)) {
      return(v)
    }
    if (v <= x[1L]) {
      return(0.0)
    }
    if (v >= x[n]) {
      return(1.0)
    }
    j <- dist_upper_bound(x, v) - 1L
    t <- dist_unit_ratio(v - x[j + 1L], x[j + 2L] - x[j + 1L])
    (j + t) / (n - 1L)
  }

  density_at <- function(v) {
    if (is.na(v)) {
      return(v)
    }
    if (v < x[1L] || v > x[n]) {
      return(0.0)
    }
    j <- if (v == x[n]) n - 2L else dist_upper_bound(x, v) - 1L
    1.0 / (n - 1L) / (x[j + 2L] - x[j + 1L])
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

  # The staircase this model is: cell boundaries, cumulative values and heights.
  #
  # Point evaluation answers what the density is at a value a caller picked; this answers what
  # the density *is*, on the boundaries the model itself uses. Every height is the expression
  # density_at evaluates inside the same cell, so the two agree bit for bit rather than
  # approximately.
  landscape_fn <- function() {
    cumulative <- vapply(x, cumulative_at, numeric(1))
    densities <- vapply(seq_len(n - 1L), function(j) 1.0 / (n - 1L) / (x[j + 1L] - x[j]), numeric(1))
    list(boundaries = x, cumulative = cumulative, densities = densities, cell_count = n - 1L)
  }

  list(
    algorithm = "dist-linear-v1",
    size = n,
    support_min = x[1L],
    support_max = x[n],
    unit = unit,
    quantile_at = quantile_at,
    cumulative_at = cumulative_at,
    density_at = density_at,
    quantile = quantile_checked,
    cumulative = cumulative_checked,
    density = density_checked,
    landscape = landscape_fn,
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
