center <- function(x) {
  n <- length(x)
  if (n == 0) {
    stop("Input vector cannot be empty")
  }
  # Use fast O(n log n) algorithm
  fast_center(x)
}
