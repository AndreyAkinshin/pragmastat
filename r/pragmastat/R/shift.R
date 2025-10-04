shift <- function(x, y) {
  if (length(x) == 0 || length(y) == 0) {
    stop("Input vectors cannot be empty")
  }
  pairwise_shifts <- outer(x, y, "-")
  median(pairwise_shifts)
}
