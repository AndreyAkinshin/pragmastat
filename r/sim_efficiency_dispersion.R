source("pragmastat/R/spread.R")

mad <- function(x) {
  m <- median(x)
  median(abs(x - m))
}

simulation_efficiency_dispersion_n <- function(n,iterations) {
  samples <- lapply(1:iterations, function(i) rnorm(n))
  estimations_sd <- sapply(samples, sd)
  estimations_spread <- sapply(samples, spread)
  estimations_mad <- sapply(samples, mad)
  
  estimations_sd <- estimations_sd / mean(estimations_sd)
  estimations_spread <- estimations_spread / mean(estimations_spread)
  estimations_mad <- estimations_mad / mean(estimations_mad)
  
  var_sd <- var(estimations_sd)
  var_spread <- var(estimations_spread)
  var_mad <- var(estimations_mad)
  c(
    n = n,
    spread = min(var_sd / var_spread, 1),
    mad = min(var_sd / var_mad, 1)
  )
}

simulation_efficiency_dispersion <- function(sampleSizes, iterations) {
  sim_n <- function(n) simulation_efficiency_dispersion_n(n, iterations)
  do.call(rbind, lapply(sampleSizes, sim_n))
}

df <- simulation_efficiency_dispersion(3:100, 100000)
df <- round(df, 3)
write.csv(df, "../simulations/efficiency_dispersion.csv", row.names = FALSE, quote = FALSE)
