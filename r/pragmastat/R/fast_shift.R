#' Fast O((m + n) * log(precision)) implementation of the Shift estimator
#'
#' Computes quantiles of all pairwise differences \{x_i - y_j\} efficiently
#' using binary search with two-pointer counting to avoid materializing
#' all m*n differences.
#'
#' @param x Numeric vector of values
#' @param y Numeric vector of values
#' @param p Numeric vector of probabilities in [0, 1]
#' @param assume_sorted Logical; if TRUE, assume x and y are already sorted
#' @return Numeric vector of quantile values
#' @keywords internal
fast_shift <- function(x, y, p = 0.5, assume_sorted = FALSE) {
  if (!is.numeric(x) || !is.numeric(y)) {
    stop("x and y must be numeric vectors")
  }
  if (length(x) == 0 || length(y) == 0) {
    stop("x and y cannot be empty")
  }
  if (!is.numeric(p)) {
    stop("p must be numeric")
  }
  if (any(is.na(p)) || any(p < 0) || any(p > 1)) {
    stop("Probabilities must be within [0, 1]")
  }
  if (!is.logical(assume_sorted)) {
    stop("assume_sorted must be logical")
  }

  # Call the C implementation
  .Call("fast_shift_c", as.double(x), as.double(y), as.double(p), as.logical(assume_sorted), PACKAGE = "pragmastat")
}
