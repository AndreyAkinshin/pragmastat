shift <- function(x, y) {
  # Check validity (priority 0)
  check_validity(x, SUBJECTS$X)
  check_validity(y, SUBJECTS$Y)
  # Use fast O((m + n) * log(precision)) algorithm
  fast_shift(x, y, p = 0.5, assume_sorted = FALSE)
}
