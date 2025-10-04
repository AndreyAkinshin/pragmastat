rel_spread <- function(x) {
  center_val <- center(x)
  if (center_val == 0) {
    stop("RelSpread is undefined when Center equals zero")
  }
  spread(x) / abs(center_val)
}
