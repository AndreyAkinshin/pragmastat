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
#' dist <- dist_exp(1) # rate=1, mean=1
#' dist$sample(r)
#'
dist_exp <- function(rate) {
  if (!(is.finite(rate) && rate > 0)) {
    stop("rate must be finite and positive")
  }

  sample_fn <- function(rng) {
    quantile_at(rng$uniform_float())
  }

  density_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x < 0.0) 0.0 else rate * exp_function(-rate * x)
  }

  cumulative_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x < 0.0) 0.0 else 1.0 - exp_function(-rate * x)
  }

  quantile_at <- function(p) {
    if (is.na(p)) {
      return(p)
    }
    if (p <= 0.0) {
      return(0.0)
    }
    if (p >= 1.0) {
      return(Inf)
    }
    -log_function(1.0 - p) / rate
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
