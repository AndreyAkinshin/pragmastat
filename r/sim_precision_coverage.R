source("pragmastat/R/center.R")
source("pragmastat/R/spread.R")
source("pragmastat/R/precision.R")
library(knitr)

coverage_single <- function(n) {
  x <- rnorm(n)
  x_abs_center <- abs(center(x))
  x_precision <- precision(x)
  c(
    n = n,
    k1 = x_abs_center < 1 * x_precision,
    k2 = x_abs_center < 2 * x_precision,
    k3 = x_abs_center < 3 * x_precision
  )
}

coverage <- function(n, iterations = 10000000) {
  experiments <- do.call(rbind, lapply(1:iterations, \(i) coverage_single(n)))
  colSums(experiments) / iterations
}

set.seed(1729)
coverage_table <- do.call(rbind, lapply(2:30, \(n) coverage(n)))
coverage_table <- round(coverage_table, 5)
kable(coverage_table)
