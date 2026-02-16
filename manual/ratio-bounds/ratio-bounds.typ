#import "/manual/definitions.typ": *

== RatioBounds <sec-ratio-bounds>

$ RatioBounds(vx, vy, misrate) = exp(ShiftBounds(log vx, log vy, misrate)) $

Robust bounds on #link(<sec-ratio>)[$Ratio(vx, vy)$] with specified coverage — the multiplicative dual of #link(<sec-shift-bounds>)[$ShiftBounds$].

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — distribution-free confidence interval for Hodges-Lehmann ratio],
  [*Interpretation* — $misrate$ is probability that true ratio falls outside bounds],
  [*Domain* — $x_i > 0$, $y_j > 0$, $misrate >= 2 / binom(n+m, n)$],
  [*Assumptions* — #link(<sec-positivity>)[`positivity(x)`], #link(<sec-positivity>)[`positivity(y)`]],
  [*Unit* — dimensionless],
  [*Note* — assumes weak continuity (ties from measurement resolution are tolerated but may yield conservative bounds)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Scale invariance* #h(2em) $RatioBounds(k dot vx, k dot vy, misrate) = RatioBounds(vx, vy, misrate)$],
  [*Scale equivariance* #h(2em) $RatioBounds(k_x dot vx, k_y dot vy, misrate) = (k_x / k_y) dot RatioBounds(vx, vy, misrate)$],
  [*Multiplicative antisymmetry* #h(2em) $RatioBounds(vx, vy, misrate) = 1 / RatioBounds(vy, vx, misrate)$ (bounds reversed)],
)

#v(0.3em)
*Example*

- `RatioBounds([1..30], [10..40], 1e-4)` where `Ratio ≈ 0.5` yields bounds containing $0.5$
- Bounds fail to cover true ratio with probability $approx misrate$

#v(0.5em)
*Relationship to ShiftBounds*

$RatioBounds$ is computed via log-transformation:

$ RatioBounds(vx, vy, misrate) = exp(ShiftBounds(log vx, log vy, misrate)) $

This means if #link(<sec-shift-bounds>)[$ShiftBounds$] returns $[a, b]$ for the log-transformed samples,
$RatioBounds$ returns $[e^a, e^b]$.

#v(0.5em)
$RatioBounds$ provides not just the estimated ratio but also the uncertainty of that estimate.
The function returns an interval of plausible ratio values given the data.
Set $misrate$ to control how often the bounds might fail to contain the true ratio: use $10^(-3)$ for everyday analysis or $10^(-6)$ for critical decisions where errors are costly.
These bounds require no assumptions about your data distribution, so they remain valid for any continuous positive measurements.
If the bounds exclude $1$, that suggests a reliable multiplicative difference between the two groups.

#pagebreak()
=== Algorithm <sec-alg-ratio-bounds>

#include "ratio-bounds-algorithm.typ"

#pagebreak()
=== Tests

#include "ratio-bounds-tests.typ"
