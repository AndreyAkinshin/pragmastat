source("init.R")
source("utils.R")

################################################################################
# Simulations                                                                  #
################################################################################

generate_avg_drift <- function() {
  raw <- read_json("../simulations/avg-drift.json")
  df <- data.frame(do.call("rbind", lapply(raw, \(it) c(
    distribution = it$distribution,
    n = as.numeric(it$sampleSize),
    mean = it$drifts$Mean,
    median = it$drifts$Median,
    center = it$drifts$Center
  ))))
  df <- df %>% gather("estimator", "drift", -distribution, -n)
  df$n <- as.numeric(df$n)
  df$drift2 <- as.numeric(df$drift)^2
  df$estimator <- factor(df$estimator, c("center", "mean", "median"))
  distributions <- unique(df$distribution)
  for (distribution in distributions) {
    df_d <- df[df$distribution == distribution,]
    y_breaks <- pretty(c(0, max(df_d$drift2)), 9)
    p <- ggplot(df_d, aes(n, drift2, col = estimator)) +
      geom_point() +
      scale_y_continuous(breaks = y_breaks, limits = c(0, NA)) +
      scale_color_manual(
        values = c(cbp$green, cbp$blue, cbp$red),
        labels = c("Center", "Mean", "Median")
      ) +
      labs(
        title = paste(distribution, "distribution"),
        y = "Drift²",
        col = "Estimator"
      )
    ggsave_(paste0("avg-drift-", tolower(distribution)), p)
  }
}

generate_disp_drift <- function() {
  raw <- read_json("../simulations/disp-drift.json")
  df <- data.frame(do.call("rbind", lapply(raw, \(it) c(
    distribution = it$distribution,
    n = as.numeric(it$sampleSize),
    spread = it$drifts$Spread,
    stddev = it$drifts$StdDev,
    mad = it$drifts$MAD
  ))))
  df <- df %>% gather("estimator", "drift", -distribution, -n)
  df$n <- as.numeric(df$n)
  df$drift2 <- as.numeric(df$drift)^2
  df$estimator <- factor(df$estimator, c("spread", "stddev", "mad"))
  distributions <- unique(df$distribution)
  for (distribution in distributions) {
    df_d <- df[df$distribution == distribution,]
    y_breaks <- pretty(c(0, max(df_d$drift2)), 9)
    p <- ggplot(df_d, aes(n, drift2, col = estimator)) +
      geom_point() +
      scale_y_continuous(breaks = y_breaks, limits = c(0, NA)) +
      scale_color_manual(
        values = c(cbp$green, cbp$blue, cbp$red),
        labels = c("Spread", "StdDev", "MAD")
      ) +
      labs(
        title = paste(distribution, "distribution"),
        y = "Drift²",
        col = "Estimator"
      )
    ggsave_(paste0("disp-drift-", tolower(distribution)), p)
  }
}

figure_distribution_additive <- function() {
  x <- seq(-3, 3, by = 0.01)
  y <- dnorm(x)
  df <- data.frame(x, y)
  ggplot(df, aes(x, y)) +
    geom_line() +
    scale_x_continuous(breaks = pretty(x, 9)) +
    labs(
      title = "Density of Additive(0, 1)",
      y = "Density"
      )
}

figure_distribution_multiplic <- function() {
  x <- seq(0.01, 5, by = 0.01)
  y <- dlnorm(x)
  df <- data.frame(x, y)
  ggplot(df, aes(x, y)) +
    geom_line() +
    scale_x_continuous(breaks = pretty(x, 9)) +
    labs(
      title = "Density of Multiplic(0, 1)",
      y = "Density"
      )
}

figure_distribution_exponential <- function() {
  x <- seq(0, 5, by = 0.01)
  y <- dexp(x, rate = 1)
  df <- data.frame(x, y)
  ggplot(df, aes(x, y)) +
    geom_line() +
    scale_x_continuous(breaks = pretty(x, 9)) +
    labs(
      title = "Density of Exponential(1)",
      y = "Density"
      )
}

figure_distribution_power <- function() {
  x <- seq(1, 10, by = 0.01)
  y <- (2 * 1^2) / x^3  # Pareto density with scale=1, shape=2
  df <- data.frame(x, y)
  ggplot(df, aes(x, y)) +
    geom_line() +
    scale_x_continuous(breaks = pretty(x, 9)) +
    scale_y_continuous(limits = c(0, NA)) +
    labs(
      title = "Density of Power(1, 2)",
      y = "Density"
      )
}

figure_distribution_uniform <- function() {
  x <- seq(-0.5, 1.5, by = 0.01)
  y <- dunif(x, min = 0, max = 1)
  df <- data.frame(x, y)
  ggplot(df, aes(x, y)) +
    geom_line() +
    scale_x_continuous(breaks = pretty(x, 9)) +
    labs(
      title = "Density of Uniform(0, 1)",
      y = "Density"
      )
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
generate_avg_drift()
generate_disp_drift()
