spread <- function(x) {
  n <- length(x)
  if (n == 0) {
    stop("Input vector cannot be empty")
  }
  if (n == 1) {
    return(0)
  }
  pairwise_diffs <- outer(x, x, "-")
  pairwise_abs_diffs <- abs(pairwise_diffs)
  median(pairwise_abs_diffs[upper.tri(pairwise_abs_diffs, diag = FALSE)])
}
