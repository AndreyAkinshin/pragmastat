#' Exponential Distribution
#'
#' Create an exponential distribution with given rate parameter.
#' The mean of this distribution is 1/rate.
#'
#' @param rate Rate parameter (lambda > 0)
#' @return A Distribution object
#'
#' @examples
#' r <- rng("demo-dist-exp")
#' dist <- dist_exp(1)  # rate=1, mean=1
#' dist$sample(r)
#'
#' @export
dist_exp <- function(rate) {
  if (rate <= 0) {
    stop("rate must be positive")
  }

  sample_fn <- function(rng) {
    # Inverse CDF method: -ln(1 - U) / rate
    u <- rng$uniform()
    # Avoid log(0) - use machine epsilon for cross-language consistency
    if (u == 1.0) {
      u <- 1.0 - .MACHINE_EPSILON
    }
    -log(1.0 - u) / rate
  }

  list(
    sample = sample_fn,
    samples = function(rng, count) {
      sapply(seq_len(count), function(i) sample_fn(rng))
    }
  )
}
