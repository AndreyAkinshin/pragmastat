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
#' @export
dist_multiplic <- function(log_mean, log_std_dev) {
  if (log_std_dev <= 0) {
    stop("log_std_dev must be positive")
  }

  additive <- dist_additive(log_mean, log_std_dev)

  sample_fn <- function(rng) {
    exp(additive$sample(rng))
  }

  list(
    sample = sample_fn,
    samples = function(rng, count) {
      sapply(seq_len(count), function(i) sample_fn(rng))
    }
  )
}
