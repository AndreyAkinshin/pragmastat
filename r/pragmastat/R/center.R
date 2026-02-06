center <- function(x) {
  # Check validity (priority 0)
  check_validity(x, SUBJECTS$X)
  # Use fast O(n log n) algorithm
  fast_center(x)
}
