#import "/manual/definitions.typ": *

== DisparityBounds <sec-disparity-bounds>

$ DisparityBounds(vx, vy, misrate) = [L_D, U_D] $

Let
$min_S = 2 / binom(n + m, n)$ and
$min_A = 2 dot max(2^(1-floor(n/2)), 2^(1-floor(m/2)))$.
Require $misrate >= min_S + min_A$.
Let $"extra" = misrate - (min_S + min_A)$,
$alpha_S = min_S + "extra" / 2$,
$alpha_A = min_A + "extra" / 2$.
Compute
$[L_S, U_S] = ShiftBounds(vx, vy, alpha_S)$ and
$[L_A, U_A] = AvgSpreadBounds(vx, vy, alpha_A)$.

If $L_A > 0$, return
$[L_D, U_D] = [min(L_S/L_A, L_S/U_A, U_S/L_A, U_S/U_A), max(L_S/L_A, L_S/U_A, U_S/L_A, U_S/U_A)]$.

If $L_A = 0$, return the tightest single interval that is always valid:

#v(0.2em)
#list(marker: none, tight: true,
  [$L_S > 0$: $[L_S / U_A, +infinity)$],
  [$U_S < 0$: $(-infinity, U_S / U_A]$],
  [$L_S = 0$ and $U_S = 0$: $[0, 0]$],
  [$L_S = 0$ and $U_S > 0$: $[0, +infinity)$],
  [$L_S < 0$ and $U_S = 0$: $(-infinity, 0]$],
  [otherwise: $(-infinity, +infinity)$],
)

If $U_A = 0$, use the sign-only rule:
$[0, +infinity)$ if $L_S >= 0$,
$(-infinity, 0]$ if $U_S <= 0$,
$(-infinity, +infinity)$ otherwise (with $[0, 0]$ when $L_S = U_S = 0$).

Robust bounds on #link(<sec-disparity>)[$Disparity(vx, vy)$] with specified coverage.

#v(0.3em)
#list(marker: none, tight: true,
  [*Interpretation* --- $misrate$ is probability that true disparity falls outside bounds],
  [*Domain* --- any real numbers, $n >= 2$, $m >= 2$, $misrate >= min_S + min_A$],
  [*Assumptions* --- #link(<sec-sparity>)[`sparity(x)`], #link(<sec-sparity>)[`sparity(y)`]],
  [*Unit* --- dimensionless (spread units)],
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
