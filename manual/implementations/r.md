<span id="r"></span> <!-- [pdf] DELETE -->

## R

Install from GitHub:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat",
                        subdir = "r/pragmastat", ref = "v3.1.24")
library(pragmastat)
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v3.1.24/r



Demo:

```r
library(pragmastat)

x <- c(0, 2, 4, 6, 8)
print(center(x)) # 4
print(center(x + 10)) # 14
print(center(x * 3)) # 12

print(spread(x)) # 4
print(spread(x + 10)) # 4
print(spread(x * 2)) # 8

print(rel_spread(x)) # 1
print(rel_spread(x * 5)) # 1

y <- c(10, 12, 14, 16, 18)
print(shift(x, y)) # -10
print(shift(x, x)) # 0
print(shift(x + 7, y + 3)) # -6
print(shift(x * 2, y * 2)) # -20
print(shift(y, x)) # 10

x <- c(1, 2, 4, 8, 16)
y <- c(2, 4, 8, 16, 32)
print(ratio(x, y)) # 0.5
print(ratio(x, x)) # 1
print(ratio(x * 2, y * 5)) # 0.2

x <- c(0, 3, 6, 9, 12)
y <- c(0, 2, 4, 6, 8)
print(spread(x)) # 6
print(spread(y)) # 4

print(avg_spread(x, y)) # 5
print(avg_spread(x, x)) # 6
print(avg_spread(x * 2, x * 3)) # 15
print(avg_spread(y, x)) # 5
print(avg_spread(x * 2, y * 2)) # 10

print(shift(x, y)) # 2
print(avg_spread(x, y)) # 5

print(disparity(x, y)) # 0.4
print(disparity(x + 5, y + 5)) # 0.4
print(disparity(x * 2, y * 2)) # 0.4
print(disparity(y, x)) # -0.4
```