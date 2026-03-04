#import "/manual/definitions.typ": *

== Compare1 <sec-compare1>

$ Compare1(vx, [T_1, ..., T_k]) = [P_1, ..., P_k] $

where $T_i = (M_i, t_i, misrate_i)$ is a threshold with metric $M_i$ ($Center$ or $Spread$),
$P_i = (e_i, [L_i, U_i], v_i)$ is the projection with estimate $e_i$, bounds $[L_i, U_i]$,
and verdict $v_i = upright("Greater")$ if $L_i > t_i$; $upright("Less")$ if $U_i < t_i$; $upright("Inconclusive")$ otherwise.

One-sample confirmatory analysis: compares estimates against practical thresholds.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$vx = (x_1, ..., x_n)$ — sample of measurements],
  [$T_i = (M_i, t_i, misrate_i)$ — list of $k$ thresholds: $M_i$ is $Center$ or $Spread$, $t_i$ is the threshold value, $misrate_i$ is the per-threshold error rate],
  [$"seed"$ — optional string for reproducible randomization (passed to $SpreadBounds$)],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* — list of $k$ projections in input order, each $P_i = (e_i, [L_i, U_i], v_i)$],
  [*Unit* — per-projection: same unit as the underlying estimator ($Center$ or $Spread$)],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Independence* — each threshold evaluated independently; no family-wise guarantee],
  [*Unit compatibility* — threshold unit must be compatible with sample unit],
  [*See also* — #link(<sec-compare2>)[$Compare2$] for two-sample metrics ($Shift$, $Ratio$, $Disparity$)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Order preservation* #h(2em) $P_i$ corresponds to input $T_i$],
  [*Metric deduplication* #h(2em) each distinct metric computed once regardless of threshold count],
)

#v(0.3em)
*Example*

- `Compare1([1..10], [(Center, 20, 1e-3)])` → `[Projection(5.5, [...], Less)]`
- `Compare1([1..10], [(Spread, 0.1, 1e-3)])` → `[Projection(3, [...], Greater)]`

#v(0.5em)
$Compare1$ automates the pattern of computing an estimate, constructing bounds, and comparing the bounds against a practical threshold.
Instead of asking whether $Center$ is significantly different from zero, it answers whether $Center$ is reliably above or below a practical threshold.
Each threshold produces a ternary verdict that respects both statistical uncertainty and practical relevance.
When multiple thresholds are needed (different metrics or different misrates), pass them all in one call to avoid redundant computation.

#pagebreak()
=== Algorithm <sec-alg-compare1>

#include "compare1-algorithm.typ"

#pagebreak()
=== Notes

#include "compare1-notes.typ"

#pagebreak()
=== Tests

#include "compare1-tests.typ"
