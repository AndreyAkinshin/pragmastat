#import "/manual/definitions.typ": *

== Compare2 <sec-compare2>

$ Compare2(vx, vy, [T_1, ..., T_k]) = [P_1, ..., P_k] $

where $T_i = (M_i, t_i, misrate_i)$ is a threshold with metric $M_i$ ($Shift$, $Ratio$, or $Disparity$),
$P_i = (e_i, [L_i, U_i], v_i)$ is the projection with estimate $e_i$, bounds $[L_i, U_i]$,
and verdict $v_i = upright("Greater")$ if $L_i > t_i$; $upright("Less")$ if $U_i < t_i$; $upright("Inconclusive")$ otherwise.

Two-sample confirmatory analysis: compares estimates against practical thresholds.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$vx = (x_1, ..., x_n)$ — first sample of measurements],
  [$vy = (y_1, ..., y_m)$ — second sample of measurements],
  [$T_i = (M_i, t_i, misrate_i)$ — list of $k$ thresholds: $M_i$ is $Shift$, $Ratio$, or $Disparity$; $t_i$ is the threshold value; $misrate_i$ is the per-threshold error rate],
  [$"seed"$ — optional string for reproducible randomization (passed to $DisparityBounds$)],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* — list of $k$ projections in input order, each $P_i = (e_i, [L_i, U_i], v_i)$],
  [*Unit* — per-projection: same unit as the underlying estimator ($Shift$, $Ratio$, or $Disparity$)],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Independence* — each threshold evaluated independently; no family-wise guarantee],
  [*Unit compatibility* — $Shift$: compatible with sample units; $Ratio$: dimensionless or $Ratio$, requires $t_i > 0$; $Disparity$: dimensionless or $Disparity$],
  [*See also* — #link(<sec-compare1>)[$Compare1$] for one-sample metrics ($Center$, $Spread$)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Order preservation* #h(2em) $P_i$ corresponds to input $T_i$],
  [*Metric deduplication* #h(2em) each distinct metric computed once regardless of threshold count],
)

#v(0.3em)
*Example*

- `Compare2([1..30], [21..50], [(Shift, 0, 1e-3)])` → `[Projection(-20, [...], Less)]`
- `Compare2([21..50], [1..30], [(Shift, 0, 1e-3)])` → `[Projection(20, [...], Greater)]`

#v(0.5em)
$Compare2$ automates the pattern of computing a two-sample estimate, constructing bounds, and comparing the bounds against a practical threshold.
Instead of asking whether $Shift$ is significantly different from zero, it answers whether $Shift$ is reliably above or below a practical threshold.
Each threshold produces a ternary verdict that respects both statistical uncertainty and practical relevance.
When multiple thresholds are needed (different metrics or different misrates), pass them all in one call to avoid redundant computation.

#pagebreak()
=== Algorithm <sec-alg-compare2>

#include "compare2-algorithm.typ"

#pagebreak()
=== Notes

#include "compare2-notes.typ"

#pagebreak()
=== Tests

#include "compare2-tests.typ"
