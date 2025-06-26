center <- function(x) {
  n <- length(x)
  if (n == 0) {
    stop("Input vector cannot be empty")
  }
  pairwise_averages <- outer(x, x, "+") / 2
  median(pairwise_averages[upper.tri(pairwise_averages, diag = TRUE)])
}
