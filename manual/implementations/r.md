<span id="r"></span> <!-- [pdf] DELETE -->

## R

Source code of the latest version: https://github.com/AndreyAkinshin/pragmastat/tree/main/r

Installation:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat", subdir = "r/pragmastat")
```

The R implementation provides all toolkit functions as a lightweight package with minimal dependencies.
Each function implements the exact mathematical definition from the toolkit, using R's built-in vector operations
  for efficient computation.

Demo:

```r
<!-- INCLUDE r/pragmastat/inst/examples/demo.R -->
```
