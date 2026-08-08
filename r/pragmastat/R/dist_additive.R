#' Additive (Normal) Distribution
#'
#' Create an additive (normal/Gaussian) distribution.
#' Uses the Box-Muller transform to generate samples.
#'
#' @param mean Location parameter (center of the distribution)
#' @param std_dev Scale parameter (standard deviation)
#' @return A Distribution object
#'
#' @examples
#' r <- rng("demo-dist-additive")
#' dist <- dist_additive(0, 1) # Standard normal
#' dist$sample(r)
#'
dist_additive <- function(mean, std_dev) {
  # is.finite() comes first on purpose. `!(NaN > 0)` is NA in R, and `if (NA)` raises base R's
  # "missing value where TRUE/FALSE needed", which names neither the package nor the parameter.
  if (!is.finite(mean)) {
    stop("mean must be finite")
  }
  if (!(is.finite(std_dev) && std_dev > 0)) {
    stop("std_dev must be finite and positive")
  }

  sample_fn <- function(rng) {
    # Box-Muller transform
    u1 <- rng$uniform_float()
    u2 <- rng$uniform_float()

    # Avoid log(0) - use smallest positive subnormal for cross-language consistency
    if (u1 == 0) {
      u1 <- .SMALLEST_POSITIVE_SUBNORMAL
    }

    r <- sqrt(-2.0 * log(u1))
    theta <- 2.0 * pi * u2

    # Use the first of the two Box-Muller outputs
    z <- r * cos(theta)

    mean + z * std_dev
  }


  density_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    z <- (x - mean) / std_dev
    exp_function(-z * z / 2) / (std_dev * .SQRT_2PI)
  }

  cumulative_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    additive_cumulative((x - mean) / std_dev)
  }

  # The inverse goes through additive_quantile, which is what that function was built for. It
  # replaces two Newton steps taken against an error function accurate to 1.5e-07, which had left
  # this quantile eight orders of magnitude coarser than the cumulative beside it. The infinities
  # at the ends are the ones additive_quantile already returns.
  quantile_at <- function(p) {
    if (is.na(p)) {
      return(p)
    }
    mean + std_dev * additive_quantile(p)
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
