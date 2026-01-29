#import "/manual/definitions.typ": *

== Ratio

$ Ratio(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i / y_j) $

Robust measure of scale ratio between two samples.

#v(0.3em)
#list(marker: none, tight: true,
  [*Asymptotic* — median of the ratio of random measurements from $X$ and $Y$],
  [*Domain* — $x_i, y_j > 0$ or $x_i, y_j < 0$ (exclude $abs(y_j) approx 0$)],
  [*Unit* — dimensionless],
  [*Caveat* — $Ratio(vx, vy) != 1 / Ratio(vy, vx)$ in general],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Self-ratio* #h(2em) $Ratio(vx, vx) = 1$],
  [*Scale equivariance* #h(2em) $Ratio(k_x dot vx, k_y dot vy) = (k_x / k_y) dot Ratio(vx, vy)$],
)

#v(0.3em)
*Example*

- `Ratio([1, 2, 4, 8, 16], [2, 4, 8, 16, 32]) = 0.5`
- `Ratio(x, x) = 1` #h(1em) `Ratio(2x, 5y) = 0.4 · Ratio(x, y)`

#v(0.5em)
Use $Ratio$ when you care about multiplicative relationships rather than additive differences.
If one system is "twice as fast" or prices are "30% lower," you are thinking in ratios.
A result of 0.5 means the first group is typically half the size of the second; 2.0 means twice as large.
This estimator is appropriate for quantities like prices, response times, and concentrations where relative comparisons make more sense than absolute ones.
All values must have the same sign, and you should avoid values close to zero in the denominator sample.
