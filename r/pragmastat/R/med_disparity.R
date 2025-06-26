med_disparity <- function(x, y) {
  med_spread_val <- med_spread(x, y)
  if (med_spread_val == 0) {
    return(Inf)
  }
  med_shift(x, y) / med_spread_val
}
