#' Multiplicative (Log-Normal) Distribution
#'
#' Create a multiplicative (log-normal) distribution.
#' The logarithm of samples follows an Additive (Normal) distribution.
#'
#' @param log_mean Mean of log values (location parameter)
#' @param log_std_dev Standard deviation of log values (scale parameter)
#' @return A Distribution object
#'
#' @examples
#' r <- rng("demo-dist-multiplic")
#' dist <- dist_multiplic(0, 1)
#' dist$sample(r)
#'
dist_multiplic <- function(log_mean, log_std_dev) {
  if (!is.finite(log_mean)) {
    stop("log_mean must be finite")
  }
  if (!(is.finite(log_std_dev) && log_std_dev > 0)) {
    stop("log_std_dev must be finite and positive")
  }

  sample_fn <- function(rng) {
    quantile_at(.sampling_cumulance(rng))
  }

  # The support is x > 0, and that is the test. It used to be x < 1e-9, which is not the edge of
  # the support but an arbitrary point inside it: a dist_multiplic(-30, 1) has its median at
  # 9.4e-14, so that threshold reported zero density and zero probability across essentially the
  # whole distribution.
  density_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x <= 0.0) {
      return(0.0)
    }
    z <- (log_function(x) - log_mean) / log_std_dev
    exp_function(-z * z / 2) / (x * log_std_dev * .SQRT_2PI)
  }

  # A lognormal variable's distribution function is the normal one applied to log x, so this goes
  # through additive_cumulative rather than spelling the same thing out through erf.
  cumulative_at <- function(x) {
    if (is.na(x)) {
      return(x)
    }
    if (x <= 0.0) {
      return(0.0)
    }
    additive_cumulative((log_function(x) - log_mean) / log_std_dev)
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
    exp_function(log_mean + log_std_dev * additive_quantile(p))
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
