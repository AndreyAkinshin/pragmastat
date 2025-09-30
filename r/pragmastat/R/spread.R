spread <- function(x) {
  n <- length(x)
  if (n == 0) {
    stop("Input vector cannot be empty")
  }
  if (n == 1) {
    return(0)
  }
  # Use fast O(n log n) algorithm
  fast_spread(x)
}
