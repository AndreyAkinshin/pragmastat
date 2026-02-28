# Center estimates the central value of the data (Hodges-Lehmann estimator).
# Calculates the median of all pairwise averages (x[i] + x[j])/2.
#
# @param x Numeric vector or Sample object
# @return Measurement (when Sample input) or numeric (when vector input)
center <- function(x) {
  if (inherits(x, "Sample")) {
    return(center_estimator(x))
  }
  # Legacy vector interface
  check_validity(x, SUBJECTS$X)
  fast_center(x)
}

# Internal Sample-based estimator
center_estimator <- function(x) {
  check_non_weighted("x", x)
  result <- fast_center(x$values)
  Measurement$new(result, x$unit)
}
