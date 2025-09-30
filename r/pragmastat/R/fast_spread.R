#' Fast O(n log n) implementation of the Spread (Shamos) estimator
#'
#' Based on Monahan's selection algorithm adapted for pairwise differences.
#' Computes the median of all pairwise absolute differences efficiently.
#'
#' @param values Numeric vector of values
#' @return The spread estimate (Shamos estimator)
#' @keywords internal
fast_spread <- function(values) {
  if (!is.numeric(values)) {
    stop("Input must be a numeric vector")
  }

  # Call the C implementation
  .Call("fast_spread_c", as.double(values), PACKAGE = "pragmastat")
}
