source("init.R")
source("utils.R")

figure_logo <- function() {
  l <- 0; r <- 18; w <- r - l
  x <- seq(l, r, length.out = 10000)
  y <- 1 * dnorm(x, mean = 3, sd = 1) +
    2 * dnorm(x, mean = 8, sd = 1) +
    2 * dnorm(x, 5, 5) +
    0.02 * sin(x * 2) + 0.015 * sin(x * 5)
  x <- 0.05 + x / r * 0.9
  y <- 0.15 + y / max(y) * 0.8

  plot(x, y, 
       type = "n", 
       xlim = c(0, 1),
       ylim = c(0, 1),
       axes = FALSE,
       xlab = "",
       ylab = "",
       main = "",
       bty = "n")
  
  c <- 0.5
  radius <- 0.5
  symbols(c, c, circles = radius, 
          bg = cbp$blue, fg = NA, add = TRUE, inches = FALSE)

  inside_circle <- sqrt((x - c)^2 + (y - c)^2) <= radius - 0.02
  lines(x[inside_circle], y[inside_circle], lwd = 50, col = "black")
}

png("logo.png", width = 800, height = 800, bg = "transparent")
figure_logo()
dev.off()

figure_logo()