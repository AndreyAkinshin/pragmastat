#' Uniform Distribution
#'
#' Create a uniform distribution on [min, max).
#'
#' @param min_val Lower bound (inclusive)
#' @param max_val Upper bound (exclusive)
#' @return A Distribution object
#'
#' @examples
#' r <- rng("demo-dist-uniform")
#' dist <- dist_uniform(0, 10)
#' dist$sample(r)
#'
dist_uniform <- function(min_val, max_val) {
  # is.finite() comes first on purpose. `!(NaN < 1)` is NA in R, and `if (NA)` raises base R's
  # "missing value where TRUE/FALSE needed", which names neither the package nor the parameter.
  if (!is.finite(min_val)) {
    stop("min must be finite")
  }
  if (!is.finite(max_val)) {
    stop("max must be finite")
  }
  if (!(min_val < max_val)) {
    stop("min must be less than max")
  }

  sample_fn <- function(rng) {
    quantile_at(rng$uniform_float())
  }

  # The support is half-open, so the density is zero at max and positive at min. This is the one
  # family assembled from arithmetic alone: no primitive of the toolkit appears on any of its
  # three paths.
  density_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x < min_val || x >= max_val) 0.0 else 1.0 / (max_val - min_val)
  }

  cumulative_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x < min_val) {
      return(0.0)
    }
    if (x >= max_val) {
      return(1.0)
    }
    (x - min_val) / (max_val - min_val)
  }

  quantile_at <- function(p) {
    if (is.na(p)) {
      return(p)
    }
    if (p <= 0.0) {
      return(min_val)
    }
    if (p >= 1.0) {
      return(max_val)
    }
    min_val + p * (max_val - min_val)
  }

  list(
    sample = sample_fn,
    density_at = density_at,
    cumulative_at = cumulative_at,
    quantile_at = quantile_at,
    samples = function(rng, count) {
      sapply(seq_len(count), function(i) sample_fn(rng))
    }
  )
}
