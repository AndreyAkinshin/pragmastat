source("init.R")
source("utils.R")

################################################################################
# Simulations                                                                  #
################################################################################

figure_efficiency_central_tendency <- function() {
  df <- read.csv("../simulations/efficiency_central_tendency.csv") %>%
    pivot_longer(cols = c(median, center), names_to = "estimator", values_to = "efficiency") %>%
    mutate(estimator = case_when(
      estimator == "median" ~ "Median",
      estimator == "center" ~ "Center"
    ))

  # Create the plot
  ggplot(df, aes(x = n, y = efficiency, color = estimator)) +
    geom_line() +
    geom_hline(yintercept = 3 / pi, col = cbp$green, linetype = "dotted") +
    geom_hline(yintercept = 2 / pi, col = cbp$red, linetype = "dotted") +
    scale_color_manual(
      values = c("Center" = cbp$green, "Median" = cbp$red),
      breaks = c("Center", "Median")
    ) +
    labs(
      x = "Sample Size",
      y = "Relative Efficiency",
      color = "Estimator",
      title = "Relative Gaussian Efficiency to the Mean"
    ) +
    scale_y_continuous(limits = c(0, 1), breaks = 0:10 / 10)
}

figure_efficiency_dispersion <- function() {
  df <- read.csv("../simulations/efficiency_dispersion.csv") %>%
    pivot_longer(cols = c(spread, mad), names_to = "estimator", values_to = "efficiency") %>%
    mutate(estimator = case_when(
      estimator == "spread" ~ "Spread",
      estimator == "mad" ~ "MAD"
    ))

  # Create the plot
  ggplot(df, aes(x = n, y = efficiency, color = estimator)) +
    geom_line() +
    geom_hline(yintercept = 0.864, col = cbp$green, linetype = "dotted") +
    geom_hline(yintercept = 0.368, col = cbp$red, linetype = "dotted") +
    scale_color_manual(
      values = c("Spread" = cbp$green, "MAD" = cbp$red),
      breaks = c("Spread", "MAD")
    ) +
    labs(
      x = "Sample Size",
      y = "Relative Efficiency",
      color = "Estimator",
      title = "Relative Gaussian Efficiency to the StdDev"
    ) +
    scale_y_continuous(limits = c(0, 1), breaks = 0:10 / 10)
}

regenerate_figures <- function() {
  ## Remove all existing images (except logo.png)
  for (file in list.files()) {
    if ((endsWith(file, ".png") || endsWith(file, ".svg")) && file != "logo.png") {
      file.remove(file)
    }
  }

  ## Draw all the defined figures
  for (func in lsf.str(envir = .GlobalEnv)) {
    if (startsWith(func, "figure_") || startsWith(func, "figure.")) {
      name <- substring(func, nchar("figure_") + 1)
      name <- gsub("_", "-", name)
      ggsave_(name, get(func))
    }
  }
}
regenerate_figures()
