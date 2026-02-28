library(pragmastat)

# --- One-Sample ---

x <- 1:22

print(center(x)) # 11.5
bounds <- center_bounds(x, 1e-3)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [6, 17]
print(spread(x)) # 7
bounds <- spread_bounds(x, 1e-3, seed = "demo")
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [1, 18]

# --- Two-Sample ---

x <- 1:30
y <- 21:50

print(shift(x, y)) # -20
bounds <- shift_bounds(x, y, 1e-3)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [-28, -12]
print(ratio(x, y)) # 0.436698
bounds <- ratio_bounds(x, y, 1e-3)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [0.232558139534884, 0.642857142857143]
print(disparity(x, y)) # -2.222222
bounds <- disparity_bounds(x, y, 1e-3, seed = "demo")
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [-29, -0.478260869565217]

# --- Randomization ---

r <- rng("demo-uniform")
print(r$uniform_float()) # 0.2640554
print(r$uniform_float()) # 0.9348535

r <- rng("demo-uniform-int")
print(r$uniform_int(0, 100)) # 41

r <- rng("demo-sample")
print(r$sample(0:9, 3)) # 3 8 9

r <- rng("demo-resample")
print(r$resample(c(1, 2, 3, 4, 5), 7)) # 3 1 3 2 4 1 2

r <- rng("demo-shuffle")
print(r$shuffle(c(1, 2, 3, 4, 5))) # 4 2 3 5 1

# --- Distributions ---

r <- rng("demo-dist-additive")
dist <- dist_additive(0, 1)
print(dist$sample(r)) # 0.1741045

r <- rng("demo-dist-multiplic")
dist <- dist_multiplic(0, 1)
print(dist$sample(r)) # 1.127324

r <- rng("demo-dist-exp")
dist <- dist_exp(1)
print(dist$sample(r)) # 0.6589065

r <- rng("demo-dist-power")
dist <- dist_power(1, 2)
print(dist$sample(r)) # 1.023678

r <- rng("demo-dist-uniform")
dist <- dist_uniform(0, 10)
print(dist$sample(r)) # 6.540437
