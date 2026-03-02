#import "/manual/definitions.typ": *

== DisparityBounds <sec-disparity-bounds>

$ DisparityBounds(vx, vy, misrate) = [L_S, U_S] / [L_A, U_A] $

where $[L_S, U_S] = ShiftBounds(vx, vy, alpha_S)$,
$[L_A, U_A] = AvgSpreadBounds(vx, vy, alpha_A)$,
and $alpha_S + alpha_A = misrate$ (Bonferroni split).

Robust bounds on #link(<sec-disparity>)[$Disparity(vx, vy)$] with specified coverage.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$vx = (x_1, x_2, ..., x_n)$ — first sample of measurements, where $n >= 2$, requires #link(<sec-sparity>)[`sparity(x)`]],
  [$vy = (y_1, y_2, ..., y_m)$ — second sample of measurements, where $m >= 2$, requires #link(<sec-sparity>)[`sparity(y)`]],
  [$misrate$ — probability that true disparity falls outside bounds in the long run (minimum depends on $n$, $m$; see Algorithm)],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- interval $[L, U]$ bounding $Disparity(vx, vy)$],
  [*Unit* --- dimensionless (spread units)],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Note* --- Bonferroni split between shift and avg-spread bounds; no independence assumption needed; bounds may be unbounded when pooled spread cannot be certified positive],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Location invariance* #h(2em) $DisparityBounds(vx + k, vy + k, misrate) = DisparityBounds(vx, vy, misrate)$],
  [*Scale invariance* #h(2em) $DisparityBounds(k dot vx, k dot vy, misrate) = op("sign")(k) dot DisparityBounds(vx, vy, misrate)$],
  [*Antisymmetry* #h(2em) $DisparityBounds(vx, vy, misrate) = -DisparityBounds(vy, vx, misrate)$ (bounds reversed)],
  [*Monotonicity in misrate* #h(2em) smaller $misrate$ produces wider bounds],
)

#v(0.3em)
*Example*

- `DisparityBounds([1..30], [21..50], 0.02)` returns bounds containing `Disparity`

#pagebreak()
=== Algorithm <sec-alg-disparity-bounds>

#include "disparity-bounds-algorithm.typ"

#pagebreak()
=== Tests

#include "disparity-bounds-tests.typ"
