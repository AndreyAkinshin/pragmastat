disparity <- function(x, y) {
  avg_spread_val <- avg_spread(x, y)
  if (avg_spread_val == 0) {
    return(Inf)
  }
  shift(x, y) / avg_spread_val
}
