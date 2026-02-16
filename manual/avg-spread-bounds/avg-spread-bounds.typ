#import "/manual/definitions.typ": *

== AvgSpreadBounds <sec-avg-spread-bounds>

$ AvgSpreadBounds(vx, vy, misrate) = [L_A, U_A] $

where $alpha = misrate / 2$,
$[L_x, U_x] = SpreadBounds(vx, alpha)$,
$[L_y, U_y] = SpreadBounds(vy, alpha)$,
$w_x = n / (n + m)$, $w_y = m / (n + m)$,
and
$[L_A, U_A] = [w_x L_x + w_y L_y, w_x U_x + w_y U_y]$.

Robust bounds on #link(<sec-avg-spread>)[$AvgSpread(vx, vy)$] with specified coverage.

#v(0.3em)
#list(marker: none, tight: true,
  [*Interpretation* --- $misrate$ is probability that true avg spread falls outside bounds],
  [*Domain* --- any real numbers, $n >= 2$, $m >= 2$, $alpha >= 2^(1-floor(n/2))$ and $alpha >= 2^(1-floor(m/2))$],
  [*Assumptions* --- #link(<sec-sparity>)[`sparity(x)`], #link(<sec-sparity>)[`sparity(y)`]],
  [*Unit* --- same as measurements],
  [*Note* --- Bonferroni combination of two #link(<sec-spread-bounds>)[$SpreadBounds$] calls with equal split $alpha = misrate/2$; no independence assumption needed; randomized pairing and cutoff, conservative with ties],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Symmetry* #h(2em) $AvgSpreadBounds(vx, vy, misrate) = AvgSpreadBounds(vy, vx, misrate)$ (equal split)],
  [*Shift invariance* #h(2em) adding constants to $vx$ and/or $vy$ does not change bounds],
  [*Scale equivariance* #h(2em) $AvgSpreadBounds(k dot vx, k dot vy, misrate) = abs(k) dot AvgSpreadBounds(vx, vy, misrate)$],
  [*Non-negativity* #h(2em) bounds are non-negative],
  [*Monotonicity in misrate* #h(2em) smaller $misrate$ produces wider bounds],
)

#v(0.3em)
*Example*

- `AvgSpreadBounds([1..30], [21..50], 0.02)` returns bounds containing `AvgSpread`

#v(0.5em)
$AvgSpreadBounds$ provides distribution-free uncertainty bounds for the pooled spread:
the weighted average of the two sample spreads.
The algorithm computes separate #link(<sec-spread-bounds>)[$SpreadBounds$] for each sample using an equal Bonferroni split
and then combines them linearly with weights $n/(n+m)$ and $m/(n+m)$.
This guarantees that the probability of missing the true $AvgSpread$ is at most $misrate$
without requiring independence between samples.

#v(0.5em)
*Minimum misrate* ---
because $alpha = misrate/2$ must satisfy the per-sample minimum,
the overall misrate must be large enough for both samples:

#v(0.3em)
$ misrate >= 2 dot max(2^(1-floor(n/2)), 2^(1-floor(m/2))) $

#pagebreak()
=== Algorithm <sec-alg-avg-spread-bounds>

#include "avg-spread-bounds-algorithm.typ"

#pagebreak()
=== Tests

#include "avg-spread-bounds-tests.typ"
