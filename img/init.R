libs <- c(
  "ggplot2",
  "ggdark",
  "jsonlite",
  "dplyr",
  "tidyr",
  "latex2exp"
)
install_if_missing <- function(pkgs) {
  to_install <- pkgs[!pkgs %in% installed.packages()[, "Package"]]
  if (length(to_install)) install.packages(to_install)
}
install_if_missing(libs)
invisible(lapply(libs, require, character.only = TRUE))

library(ggplot2)
library(ggdark)
library(jsonlite)
library(dplyr)
library(tidyr)
library(latex2exp)

rm(list = ls())
