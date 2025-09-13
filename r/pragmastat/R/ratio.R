ratio <- function(x, y) {
  if (length(x) == 0 || length(y) == 0) {
    stop("Input vectors cannot be empty")
  }
  if (any(y <= 0)) {
    stop("All values in y must be strictly positive")
  }
  pairwise_ratios <- outer(x, y, "/")
  median(pairwise_ratios)
}