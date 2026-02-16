#import "/manual/definitions.typ": *

== Ratio <sec-ratio>

$ Ratio(vx, vy) = exp(Shift(log vx, log vy)) $

Robust measure of scale ratio between two samples — the multiplicative dual of #link(<sec-shift>)[$Shift$].

#v(0.3em)
#list(marker: none, tight: true,
  [*Asymptotic* — geometric median of pairwise ratios $x_i / y_j$ (via log-space aggregation)],
  [*Domain* — $x_i > 0$, $y_j > 0$],
  [*Assumptions* — #link(<sec-positivity>)[`positivity(x)`], #link(<sec-positivity>)[`positivity(y)`]],
  [*Unit* — dimensionless],
  [*Complexity* — $O((m + n) log L)$],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Self-ratio* #h(2em) $Ratio(vx, vx) = 1$],
  [*Scale equivariance* #h(2em) $Ratio(k_x dot vx, k_y dot vy) = (k_x / k_y) dot Ratio(vx, vy)$],
  [*Multiplicative antisymmetry* #h(2em) $Ratio(vx, vy) = 1 / Ratio(vy, vx)$],
)

#v(0.3em)
*Example*

- `Ratio([1, 2, 4, 8, 16], [2, 4, 8, 16, 32]) = 0.5`
- `Ratio(x, x) = 1` #h(1em) `Ratio(2x, 5y) = 0.4 · Ratio(x, y)`

#v(0.5em)
*Relationship to Shift*

$Ratio$ is the multiplicative analog of #link(<sec-shift>)[$Shift$].
While #link(<sec-shift>)[$Shift$] computes the median of pairwise differences $x_i - y_j$,
$Ratio$ computes the median of pairwise ratios $x_i / y_j$ via log-transformation.
This relationship is expressed formally as:

$ Ratio(vx, vy) = exp(Shift(log vx, log vy)) $

The log-transformation converts multiplicative relationships to additive ones,
  allowing the fast #link(<sec-shift>)[$Shift$] algorithm to compute the result efficiently.
The exp-transformation converts back to the ratio scale.

#v(0.5em)
$Ratio$ is appropriate for multiplicative relationships rather than additive differences.
If one system is "twice as fast" or prices are "30% lower," the underlying thinking is in ratios.
A result of 0.5 means the first group is typically half the size of the second; 2.0 means twice as large.
This estimator is appropriate for quantities like prices, response times, and concentrations where relative comparisons make more sense than absolute ones.
Both samples must contain strictly positive values.

#pagebreak()
=== Algorithm <sec-alg-ratio>

#include "ratio-algorithm.typ"

#pagebreak()
=== Tests

#include "ratio-tests.typ"
