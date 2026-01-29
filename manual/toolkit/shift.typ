#import "/manual/definitions.typ": *

== Shift

$ Shift(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i - y_j) $

Robust measure of location difference between two samples.

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — Hodges-Lehmann estimator for two samples],
  [*Asymptotic* — median of the difference between random measurements from $X$ and $Y$],
  [*Complexity* — $O(m n log(m n))$ naive, $O((m+n) log L)$ fast (see #link(<sec-fast-shift>)[Fast Shift])],
  [*Domain* — any real numbers],
  [*Unit* — same as measurements],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Self-difference* #h(2em) $Shift(vx, vx) = 0$],
  [*Shift equivariance* #h(2em) $Shift(vx + k_x, vy + k_y) = Shift(vx, vy) + k_x - k_y$],
  [*Scale equivariance* #h(2em) $Shift(k dot vx, k dot vy) = k dot Shift(vx, vy)$],
  [*Antisymmetry* #h(2em) $Shift(vx, vy) = -Shift(vy, vx)$],
)

#v(0.3em)
*Example*

- `Shift([0, 2, 4, 6, 8], [10, 12, 14, 16, 18]) = -10`
- `Shift(y, x) = -Shift(x, y)`

#v(0.5em)
*References*

- @hodges1963
- @sidak1999

#v(0.5em)
Use $Shift$ when you have two groups and want to know how much one differs from the other.
If you are comparing response times between version A and version B, $Shift$ tells you by how many milliseconds A is faster or slower than B.
A negative result means the first group tends to be lower; positive means it tends to be higher.
Unlike comparing means, $Shift$ handles outliers gracefully and works well with skewed data.
The result comes in the same units as your measurements, making it easy to interpret.
