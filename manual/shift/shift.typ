#import "/manual/definitions.typ": *

== Shift <sec-shift>

$ Shift(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i - y_j) $

Robust measure of location difference between two samples.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$vx = (x_1, x_2, ..., x_n)$ — first sample of measurements],
  [$vy = (y_1, y_2, ..., y_m)$ — second sample of measurements],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* — estimation of the median of the difference between random measurements from $X$ and $Y$],
  [*Unit* — same unit as $vx$, $vy$],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Also known as* — Hodges-Lehmann estimator for two samples],
  [*Complexity* — $O((m+n) log L)$],
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
$Shift$ measures how much one group differs from another.
When comparing response times between version A and version B, $Shift$ tells by how many milliseconds A is faster or slower than B.
A negative result means the first group tends to be lower; positive means it tends to be higher.
Unlike comparing means, $Shift$ handles outliers gracefully and works well with skewed data.
The result comes in the same units as your measurements, making it easy to interpret.

#pagebreak()
=== Algorithm <sec-alg-shift>

#include "shift-algorithm.typ"

#pagebreak()
=== Tests

#include "shift-tests.typ"

=== References

#include "shift-references.typ"
