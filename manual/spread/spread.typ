#import "/manual/definitions.typ": *

== Spread <sec-spread>

$ Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j) $

Robust measure of dispersion (variability, scatter).

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — Shamos scale estimator],
  [*Asymptotic* — median of the absolute difference between two random measurements from $X$],
  [*Complexity* — $O(n log n)$],
  [*Domain* — any real numbers],
  [*Assumptions* — #link(<sec-sparity>)[`sparity(x)`]],
  [*Unit* — same as measurements],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift invariance* #h(2em) $Spread(vx + k) = Spread(vx)$],
  [*Scale equivariance* #h(2em) $Spread(k dot vx) = abs(k) dot Spread(vx)$],
  [*Non-negativity* #h(2em) $Spread(vx) >= 0$],
)

#v(0.3em)
*Example*

- `Spread([0, 2, 4, 6, 8]) = 4`
- `Spread(x + 10) = 4` #h(1em) `Spread(2x) = 8`

#v(0.5em)
$Spread$ measures how much measurements vary from each other.
It serves the same purpose as standard deviation but does not explode with outliers or heavy-tailed data.
The result comes in the same units as the measurements, so if $Spread$ is 5 milliseconds, that indicates how much values typically differ.
Like #link(<sec-center>)[$Center$], it tolerates up to 29% corrupted data.
When comparing variability across datasets, $Spread$ gives a reliable answer even when standard deviation would be misleading or infinite.
When all values are positive, $Spread$ can be conveniently expressed in relative units by dividing by $abs(Center)$: the ratio $Spread(vx) / abs(Center(vx))$ is a robust alternative to the coefficient of variation that is scale-invariant.

#pagebreak()
=== Algorithm <sec-alg-spread>

#include "spread-algorithm.typ"

#pagebreak()
=== Notes

#include "spread-notes.typ"

#pagebreak()
=== Tests

#include "spread-tests.typ"

=== References

#include "spread-references.typ"
