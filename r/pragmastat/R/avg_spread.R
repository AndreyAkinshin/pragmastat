avg_spread <- function(x, y) {
  n <- length(x)
  m <- length(y)
  if (n == 0 || m == 0) {
    stop("Input vectors cannot be empty")
  }
  spread_x <- spread(x)
  spread_y <- spread(y)
  (n * spread_x + m * spread_y) / (n + m)
}
