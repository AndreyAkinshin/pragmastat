#' Power (Pareto) Distribution
#'
#' Create a power (Pareto) distribution with minimum value and shape parameter.
#' Follows a power-law distribution where large values are rare but possible.
#'
#' @param min_val Minimum value (lower bound, > 0)
#' @param shape Shape parameter (alpha > 0, controls tail heaviness)
#' @return A Distribution object
#'
#' @examples
#' r <- rng("demo-dist-power")
#' dist <- dist_power(1, 2) # min=1, shape=2
#' dist$sample(r)
#'
#' @export
dist_power <- function(min_val, shape) {
  if (min_val <= 0) {
    stop("min must be positive")
  }
  if (shape <= 0) {
    stop("shape must be positive")
  }

  sample_fn <- function(rng) {
    # Inverse CDF method: min / (1 - U)^(1/shape)
    u <- rng$uniform_float()
    # Avoid division by zero - use machine epsilon for cross-language consistency
    if (u == 1.0) {
      u <- 1.0 - .MACHINE_EPSILON
    }
    min_val / (1.0 - u)^(1.0 / shape)
  }

  list(
    sample = sample_fn,
    samples = function(rng, count) {
      sapply(seq_len(count), function(i) sample_fn(rng))
    }
  )
}
