shift <- function(x, y) {
  if (length(x) == 0 || length(y) == 0) {
    stop("Input vectors cannot be empty")
  }
  # Use fast O((m + n) * log(precision)) algorithm
  fast_shift(x, y, p = 0.5, assume_sorted = FALSE)
}
