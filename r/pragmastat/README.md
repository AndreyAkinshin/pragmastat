# R

Install from GitHub:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat",
                        subdir = "r/pragmastat", ref = "v9.0.0")
library(pragmastat)
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v9.0.0/r

## Demo

```r
library(pragmastat)

# --- One-Sample ---

x <- 1:20

print(center(x)) # 10.5
bounds <- center_bounds(x, 0.05)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [7.5, 13.5]
print(spread(x)) # 6
bounds <- spread_bounds(x, 0.05, seed = "demo")
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [2, 10]

# --- Two-Sample ---

x <- 1:30
y <- 21:50

print(shift(x, y)) # -20
bounds <- shift_bounds(x, y, 0.05)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [-25, -15]
print(ratio(x, y)) # 0.43669798282695127
bounds <- ratio_bounds(x, y, 0.05)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [0.31250000000000006, 0.5599999999999999]
print(disparity(x, y)) # -2.2222222222222223
bounds <- disparity_bounds(x, y, 0.05, seed = "demo")
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep="")) # [-13, -0.8235294117647058]

# --- Randomization ---

r <- rng("demo-uniform")
print(r$uniform_float()) # 0.2640554428629759
print(r$uniform_float()) # 0.9348534835582796

r <- rng("demo-uniform-int")
print(r$uniform_int(0, 100)) # 41

r <- rng("demo-sample")
print(r$sample(0:9, 3)) # [3, 8, 9]

r <- rng("demo-resample")
print(r$resample(c(1, 2, 3, 4, 5), 7)) # [3, 1, 3, 2, 4, 1, 2]

r <- rng("demo-shuffle")
print(r$shuffle(c(1, 2, 3, 4, 5))) # [4, 2, 3, 5, 1]

# --- Distributions ---

r <- rng("demo-dist-additive")
dist <- dist_additive(0, 1)
print(dist$sample(r)) # 0.17410448679568188

r <- rng("demo-dist-multiplic")
dist <- dist_multiplic(0, 1)
print(dist$sample(r)) # 1.1273244602673853

r <- rng("demo-dist-exp")
dist <- dist_exp(1)
print(dist$sample(r)) # 0.6589065267276553

r <- rng("demo-dist-power")
dist <- dist_power(1, 2)
print(dist$sample(r)) # 1.023677535537084

r <- rng("demo-dist-uniform")
dist <- dist_uniform(0, 10)
print(dist$sample(r)) # 6.54043657816832
```
