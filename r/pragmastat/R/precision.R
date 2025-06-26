precision <- function(x) {
  n <- length(x)
  if (n == 0) {
    stop("Input vector cannot be empty")
  }
  2 * spread(x) / sqrt(n)
}
