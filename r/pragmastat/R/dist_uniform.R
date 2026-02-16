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
#' @export
dist_uniform <- function(min_val, max_val) {
  if (min_val >= max_val) {
    stop("min must be less than max")
  }

  sample_fn <- function(rng) {
    min_val + rng$uniform_float() * (max_val - min_val)
  }

  list(
    sample = sample_fn,
    samples = function(rng, count) {
      sapply(seq_len(count), function(i) sample_fn(rng))
    }
  )
}
