#' O(n log n) implementation of the Center (Hodges-Lehmann) estimator
#'
#' Based on Monahan's Algorithm 616 (1984).
#' Computes the median of all pairwise averages efficiently.
#'
#' @param values Numeric vector of values
#' @param assume_sorted If TRUE, assumes values are already sorted ascending and skips the internal sort
#' @return The center estimate (Hodges-Lehmann estimator)
#' @keywords internal
center_impl_compute <- function(values, assume_sorted = FALSE) {
  if (!is.numeric(values)) {
    stop("Input must be a numeric vector")
  }
  if (length(values) == 0) {
    stop("Input vector cannot be empty")
  }

  # Call the C implementation
  .Call("center_impl_c", as.double(values), as.logical(assume_sorted), PACKAGE = "pragmastat")
}
