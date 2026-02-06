# R

Install from GitHub:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat",
                        subdir = "r/pragmastat", ref = "v6.0.1")
library(pragmastat)
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v6.0.1/r

## Demo

```r
library(pragmastat)

# --- Randomization ---

r <- rng("demo-uniform")
print(r$uniform()) # 0.2640554428629759
print(r$uniform()) # 0.9348534835582796

r <- rng("demo-sample")
print(r$sample(0:9, 3)) # [3, 8, 9]

r <- rng("demo-shuffle")
print(r$shuffle(c(1, 2, 3, 4, 5))) # [4, 2, 3, 5, 1]

r <- rng("demo-resample")
print(r$resample(c(1, 2, 3, 4, 5), 7)) # [5, 1, 1, 3, 3, 4, 5]

# --- Distribution Sampling ---

r <- rng("demo-dist-uniform")
dist <- dist_uniform(0, 10)
print(dist$sample(r)) # 6.54043657816832

r <- rng("demo-dist-additive")
dist <- dist_additive(0, 1)
print(dist$sample(r)) # 0.17410448679568188

r <- rng("demo-dist-exp")
dist <- dist_exp(1)
print(dist$sample(r)) # 0.6589065267276553

r <- rng("demo-dist-power")
dist <- dist_power(1, 2)
print(dist$sample(r)) # 1.023677535537084

r <- rng("demo-dist-multiplic")
dist <- dist_multiplic(0, 1)
print(dist$sample(r)) # 1.1273244602673853

# --- Single-Sample Statistics ---

x <- c(1, 3, 5, 7, 9)

print(median(x)) # 5
print(center(x)) # 5
print(spread(x)) # 4
print(spread(x + 10)) # 4
print(spread(x * 2)) # 8
print(rel_spread(x)) # 0.8

# --- Two-Sample Comparison ---

x <- c(0, 3, 6, 9, 12)
y <- c(0, 2, 4, 6, 8)

print(shift(x, y)) # 2
print(shift(y, x)) # -2
print(avg_spread(x, y)) # 5
print(disparity(x, y)) # 0.4
print(disparity(y, x)) # -0.4

x <- c(1, 2, 4, 8, 16)
y <- c(2, 4, 8, 16, 32)
print(ratio(x, y)) # 0.5
print(ratio(y, x)) # 2

# --- One-Sample Bounds ---

x <- 1:10

print(signed_rank_margin(10, 0.05)) # 18
print(center(x)) # 5.5
bounds <- center_bounds(x, 0.05) # [lower=3.5, upper=7.5]
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep=""))
bounds <- median_bounds(x, 0.05) # [lower=2, upper=9]
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep=""))

# --- Two-Sample Bounds ---

x <- 1:30
y <- 21:50

print(pairwise_margin(30, 30, 1e-4)) # 390
print(shift(x, y)) # -20
bounds <- shift_bounds(x, y, 1e-4) # [lower=-30, upper=-10]
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep=""))

x <- c(1, 2, 3, 4, 5)
y <- c(2, 3, 4, 5, 6)
bounds <- ratio_bounds(x, y, 0.05) # [lower=0.333..., upper=1.5]
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep=""))
```
