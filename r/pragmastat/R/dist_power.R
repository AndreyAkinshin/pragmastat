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
dist_power <- function(min_val, shape) {
  if (!(is.finite(min_val) && min_val > 0)) {
    stop("min must be finite and positive")
  }
  if (!(is.finite(shape) && shape > 0)) {
    stop("shape must be finite and positive")
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


  # shape * min^shape / x^(shape+1), rearranged as (shape / x) * exp(shape * (log min - log x)).
  # The toolkit owns an exponential and a logarithm and owns no general power, so a real exponent
  # is spelled as the exponential of a logarithm. This family is the reason the logarithm had to
  # exist before the trio could.
  #
  # The support is [min, infinity), closed at the left, so the density at min is shape / min
  # rather than zero, while the cumulative there is zero. Uniform reads the same way at its own
  # lower endpoint.
  density_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x < min_val) {
      return(0.0)
    }
    (shape / x) * exp_function(shape * (log_function(min_val) - log_function(x)))
  }

  cumulative_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x <= min_val) {
      return(0.0)
    }
    1.0 - exp_function(shape * (log_function(min_val) - log_function(x)))
  }

  quantile_at <- function(p) {
    if (is.na(p)) {
      return(p)
    }
    if (p <= 0.0) {
      return(min_val)
    }
    if (p >= 1.0) {
      return(Inf)
    }
    min_val * exp_function(-log_function(1.0 - p) / shape)
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
