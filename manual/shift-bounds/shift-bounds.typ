#import "/manual/definitions.typ": *

== ShiftBounds <sec-shift-bounds>

$ ShiftBounds(vx, vy, misrate) = [z_((k_"left")), z_((k_"right"))] $

where $vz = { x_i - y_j }$ (sorted),
$k_"left" = floor(PairwiseMargin / 2) + 1$,
$k_"right" = n m - floor(PairwiseMargin / 2)$

Robust bounds on #link(<sec-shift>)[$Shift(vx, vy)$] with specified coverage.

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — distribution-free confidence interval for Hodges-Lehmann],
  [*Interpretation* — $misrate$ is probability that true shift falls outside bounds],
  [*Domain* — any real numbers, $misrate >= 2 / binom(n+m, n)$],
  [*Unit* — same as measurements],
  [*Note* — assumes weak continuity (ties from measurement resolution are tolerated but may yield conservative bounds)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift invariance* #h(2em) $ShiftBounds(vx + k, vy + k, misrate) = ShiftBounds(vx, vy, misrate)$],
  [*Scale equivariance* #h(2em) $ShiftBounds(k dot vx, k dot vy, misrate) = k dot ShiftBounds(vx, vy, misrate)$],
)

#v(0.3em)
*Example*

- `ShiftBounds([1..30], [21..50], 1e-4) = [-30, -10]` where `Shift = -20`
- Bounds fail to cover true shift with probability $approx misrate$

#v(0.5em)
$ShiftBounds$ provides not just the estimated shift but also the uncertainty of that estimate.
The function returns an interval of plausible shift values given the data.
Set $misrate$ to control how often the bounds might fail to contain the true shift: use $10^(-3)$ for everyday analysis or $10^(-6)$ for critical decisions where errors are costly.
These bounds require no assumptions about your data distribution, so they remain valid for any continuous measurements.
If the bounds exclude zero, that suggests a reliable difference between the two groups.

#pagebreak()
=== Algorithm <sec-alg-shift-bounds>

#include "shift-bounds-algorithm.typ"

#pagebreak()
=== Tests

#include "shift-bounds-tests.typ"
