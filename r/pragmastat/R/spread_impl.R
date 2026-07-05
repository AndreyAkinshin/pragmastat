#' O(n log n) implementation of the Spread (Shamos) estimator
#'
#' Based on Monahan's selection algorithm adapted for pairwise differences.
#' Computes the median of all pairwise absolute differences efficiently.
#'
#' @param values Numeric vector of values
#' @param assume_sorted If TRUE, assumes values are already sorted ascending and skips the internal sort
#' @return The spread estimate (Shamos estimator)
#' @keywords internal
spread_impl_compute <- function(values, assume_sorted = FALSE) {
  if (!is.numeric(values)) {
    stop("Input must be a numeric vector")
  }

  # Call the C implementation
  .Call("spread_impl_c", as.double(values), as.logical(assume_sorted), PACKAGE = "pragmastat")
}
