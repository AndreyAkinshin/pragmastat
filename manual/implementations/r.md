---
title: Pragmastat / R
url: r
---

The R implementation provides all toolkit functions as a lightweight package with minimal dependencies.
Each function implements the exact mathematical definition from the toolkit, using R's built-in vector operations
  for efficient computation.

Demo usage of R functions:

```r
x <- c(1, 2, 3, 4, 5, 6, 273)
center(x) # 4
spread(x) # 3
volatility(x) # 0.75
precision(x) # 2.2677868380553634

shift(x, x - 10) # 10
ratio(x, x / 10) # 10

x <- c(-3, -2, -1, 0, 1, 2, 3)
disparity(x, x * 10) # 0
disparity(x, x - 10) # 5
disparity(x * 10, x * 10 - 100) # 5
```

For quick use without installation, copy individual functions directly into scripts.
Each function is self-contained and requires only base R:

```r
<!-- INCLUDE r/pragmastat/R/center.R -->

<!-- INCLUDE r/pragmastat/R/spread.R -->

<!-- INCLUDE r/pragmastat/R/rel_spread.R -->

<!-- INCLUDE r/pragmastat/R/shift.R -->

<!-- INCLUDE r/pragmastat/R/ratio.R -->

<!-- INCLUDE r/pragmastat/R/avg_spread.R -->

<!-- INCLUDE r/pragmastat/R/disparity.R -->
```

For regular use, install the complete package from GitHub.
This provides all functions with documentation and examples:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat", subdir = "r/pragmastat")
```
