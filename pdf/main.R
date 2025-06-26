# Libraries --------------------------------------------------------------------

## rmarkdown
library(tidyverse)
library(rmarkdown)
library(tinytex)

## Plotting
library(ggpubr)
library(gridExtra)

## knitr
library(knitr)
library(kableExtra)
library(bookdown)

# Setup ------------------------------------------------------------------------
knitr::opts_chunk$set(
  echo = FALSE,
  warning = FALSE,
  cache = TRUE,
  cache.lazy = FALSE, # Ensures consistent image caching behavior
  fig.align = "center",
  fig.pos = "H",
  fig.height = 3)
options(knitr.kable.NA = '-', scipen = 999)
theme_set(theme_minimal() + theme(axis.title = element_text(colour = "#999999")))

