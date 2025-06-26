## A color palette adopted for color-blind people based on https://jfly.uni-koeln.de/color/
cbp <- list(
  red = "#D55E00", blue = "#56B4E9", green = "#009E73", orange = "#E69F00",
  navy = "#0072B2", pink = "#CC79A7", yellow = "#F0E442", grey = "#999999"
)
cbp$values <- unname(unlist(cbp))

## Mixture of two normal distributions
dnormMix <- function(x, mean1 = 0, sd1 = 1, mean2 = 0, sd2 = 1, p.mix = 0.5) {
  # p.mix is the probability of the second distribution
  # Returns: (1 - p.mix) * dnorm(x, mean1, sd1) + p.mix * dnorm(x, mean2, sd2)
  (1 - p.mix) * dnorm(x, mean = mean1, sd = sd1) + p.mix * dnorm(x, mean = mean2, sd = sd2)
}

## A smart ggsave wrapper
ggsave_ <- function(name, plot = last_plot(), basic_theme = theme_bw(), multithemed = T, ext = "png",
                    dpi = 300, width_px = 1.5 * 1600, height_px = 1.6 * 900) {
  if (is.function(name)) {
    plot <- name
    name <- as.character(match.call()[2])
    if (startsWith(name, "figure_") || startsWith(name, "figure.")) {
      name <- substring(name, nchar("figure_") + 1)
    }
  }

  get_plot <- function() if (is.function(plot)) plot() else plot

  light_transparent_theme <- theme(
    panel.background = element_rect(fill = NA, colour = NA),
    plot.background = element_rect(fill = NA, colour = NA),
    legend.background = element_rect(fill = NA, colour = NA),
    legend.box.background = element_rect(fill = NA, colour = NA),
  )
  dark_transparent_theme <- theme(
    panel.background = element_rect(fill = NA, colour = NA),
    plot.background = element_rect(fill = NA, colour = NA),
    legend.background = element_rect(fill = NA, colour = NA),
    legend.box.background = element_rect(fill = NA, colour = NA),

    panel.grid.major = element_line(color = "#b8cfe620"),
    panel.grid.minor = element_line(color = "#b8cfe610")
  )

  width <- width_px / dpi
  height <- height_px / dpi
  if (multithemed) {
    old_theme <- theme_set(basic_theme + light_transparent_theme)
    ggsave(paste0(name, "_light.", ext), get_plot(), width = width, height = height, dpi = dpi)
    message("SAVED  : ./", paste0(name, "_light.", ext))
    theme_set(dark_mode(basic_theme, verbose = FALSE) + dark_transparent_theme)
    p <- get_plot()
    show(p)
    ggsave(paste0(name, "_dark.", ext), p, width = width, height = height, dpi = dpi)
    message("SAVED  : ./", paste0(name, "_dark.", ext))
    theme_set(old_theme)
    invert_geom_defaults()
  } else {
    old_theme <- theme_set(basic_theme)
    p <- get_plot()
    show(p)
    ggsave(paste0(name, ".", ext), p, width = width, height = height, dpi = dpi)
    message("SAVED  : ./", paste0(name, ".", ext))
    theme_set(old_theme)
  }
}
