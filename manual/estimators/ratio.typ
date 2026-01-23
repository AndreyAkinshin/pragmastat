#import "/manual/definitions.typ": *

== Ratio

$ Ratio(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i / y_j) $

- Measures the median of pairwise ratios between elements of two samples
- Asymptotically, $Ratio[X, Y]$ is the median of the ratio of random measurements from $X$ and $Y$
- Note: $Ratio(vx, vy) != 1 / Ratio(vy, vx)$ in general (example: $vx=(1, 100)$, $vy=(1, 10)$)
- Practical Domain: $x_i, y_j > 0$ or $x_i, y_j < 0$. In practice, exclude values with $abs(y_j)$ near zero.
- Unit: relative

$ Ratio(vx, vx) = 1 $

$ Ratio(k_x dot vx, k_y dot vy) = k_x / k_y dot Ratio(vx, vy) $
