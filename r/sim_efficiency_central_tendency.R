source("pragmastat/R/center.R")

simulation_efficiency_central_n <- function(n,iterations) {
  samples <- lapply(1:iterations, function(i) rnorm(n))
  var_mean <- var(sapply(samples, mean))
  var_median <- var(sapply(samples, median))
  var_center <- var(sapply(samples, center))
  c(
    n = n,
    median = min(var_mean / var_median, 1),
    center = min(var_mean / var_center, 1)
  )
}

simulation_efficiency_central <- function(sampleSizes, iterations) {
  sim_n <- function(n) simulation_efficiency_central_n(n, iterations)
  do.call(rbind, lapply(sampleSizes, sim_n))
}

df <- simulation_efficiency_central(3:100, 1000000)
df <- round(df, 3)
write.csv(df, "../simulations/efficiency_central_tendency.csv", row.names = FALSE, quote = FALSE)
