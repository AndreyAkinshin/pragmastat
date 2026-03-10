# R

Install from GitHub:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat",
                        subdir = "r/pragmastat", ref = "v12.0.1")
library(pragmastat)
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v12.0.1/r

## Demo

```r
library(pragmastat)

# --- One-Sample (legacy vector interface) ---

x <- 1:200

print(center(x)) # 100.5
bounds <- center_bounds(x, 1e-3)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep = "")) # [86, 115]
print(spread(x)) # 59
bounds <- spread_bounds(x, 1e-3, seed = "demo")
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep = "")) # [44, 87]

# --- Two-Sample (legacy vector interface) ---

x <- 1:200
y <- 101:300

print(shift(x, y)) # -100
bounds <- shift_bounds(x, y, 1e-3)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep = "")) # [-120, -80]
print(ratio(x, y)) # 0.500835
bounds <- ratio_bounds(x, y, 1e-3)
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep = "")) # [0.406666666666667, 0.595833333333333]
print(disparity(x, y)) # -1.694915
bounds <- disparity_bounds(x, y, 1e-3, seed = "demo")
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep = "")) # [-3.1025641025641, -0.849462365591398]

# --- Sample-based interface ---

sx <- Sample$new(1:200)
m <- center(sx)
print(paste("center:", m$value, "unit:", m$unit$id))
b <- center_bounds(sx, 1e-3)
print(paste("center_bounds: [", b$lower, ", ", b$upper, "] unit:", b$unit$id, sep = ""))

sx <- Sample$new(1:200)
sy <- Sample$new(101:300, subject = "y")
m <- shift(sx, sy)
print(paste("shift:", m$value, "unit:", m$unit$id))
m <- ratio(sx, sy)
print(paste("ratio:", m$value, "unit:", m$unit$id))

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
```
