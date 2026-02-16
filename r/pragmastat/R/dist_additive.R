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
#' @export
dist_additive <- function(mean, std_dev) {
  if (std_dev <= 0) {
    stop("std_dev must be positive")
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

  list(
    sample = sample_fn,
    samples = function(rng, count) {
      sapply(seq_len(count), function(i) sample_fn(rng))
    }
  )
}
