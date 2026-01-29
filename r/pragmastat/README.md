# R

Install from GitHub:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat",
                        subdir = "r/pragmastat", ref = "v4.0.3")
library(pragmastat)
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v4.0.3/r

## Demo

```r
library(pragmastat)

# --- Randomization ---

r <- rng(1729)
print(r$uniform()) # 0.3943034703296536
print(r$uniform()) # 0.5730893757071377

r <- rng("experiment-1")
print(r$uniform()) # 0.9535207726895857

r <- rng(1729)
print(r$sample(0:9, 3)) # [6, 8, 9]

r <- rng(1729)
print(r$shuffle(c(1, 2, 3, 4, 5))) # [4, 2, 3, 5, 1]

# --- Distribution Sampling ---

r <- rng(1729)
dist <- dist_uniform(0, 10)
print(dist$sample(r)) # 3.9430347032965365

r <- rng(1729)
dist <- dist_additive(0, 1)
print(dist$sample(r)) # -1.222932972163442

r <- rng(1729)
dist <- dist_exp(1)
print(dist$sample(r)) # 0.5013761944646019

r <- rng(1729)
dist <- dist_power(1, 2)
print(dist$sample(r)) # 1.284909255071668

r <- rng(1729)
dist <- dist_multiplic(0, 1)
print(dist$sample(r)) # 0.2943655336550937

# --- Single-Sample Statistics ---

x <- c(0, 2, 4, 6, 8)

print(median(x)) # 4
print(center(x)) # 4
print(spread(x)) # 4
print(spread(x + 10)) # 4
print(spread(x * 2)) # 8
print(rel_spread(x)) # 1

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

# --- Confidence Bounds ---

x <- 1:30
y <- 21:50

print(pairwise_margin(30, 30, 1e-4)) # 390
print(shift(x, y)) # -20
bounds <- shift_bounds(x, y, 1e-4) # [-30, -10]
print(paste("[", bounds$lower, ", ", bounds$upper, "]", sep=""))
```
