#' Fast O(n log n) implementation of the Center (Hodges-Lehmann) estimator
#'
#' Based on Monahan's Algorithm 616 (1984).
#' Computes the median of all pairwise averages efficiently.
#'
#' @param values Numeric vector of values
#' @return The center estimate (Hodges-Lehmann estimator)
#' @keywords internal
fast_center <- function(values) {
  if (!is.numeric(values)) {
    stop("Input must be a numeric vector")
  }
  if (length(values) == 0) {
    stop("Input vector cannot be empty")
  }

  # Call the C implementation
  .Call("fast_center_c", as.double(values), PACKAGE = "pragmastat")
}
