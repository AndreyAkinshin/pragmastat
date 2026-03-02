#import "/manual/definitions.typ": *

== Disparity <sec-disparity>

$ Disparity(vx, vy) = Shift(vx, vy) / AvgSpread(vx, vy) $

where $AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m)$ is the weighted average of dispersions (pooled scale).

Robust effect size (shift normalized by pooled spread).

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$vx = (x_1, x_2, ..., x_n)$ — first sample of measurements, requires #link(<sec-sparity>)[`sparity(x)`]],
  [$vy = (y_1, y_2, ..., y_m)$ — second sample of measurements, requires #link(<sec-sparity>)[`sparity(y)`]],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* — $Shift(vx, vy) / AvgSpread(vx, vy)$],
  [*Unit* — spread units],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Also known as* — robust Cohen's d (@cohen1988; estimates differ due to robust construction)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Location invariance* #h(2em) $Disparity(vx + k, vy + k) = Disparity(vx, vy)$],
  [*Scale invariance* #h(2em) $Disparity(k dot vx, k dot vy) = op("sign")(k) dot Disparity(vx, vy)$],
  [*Antisymmetry* #h(2em) $Disparity(vx, vy) = -Disparity(vy, vx)$],
)

#v(0.3em)
*Example*

- `Disparity(x, y) = 0.4` where `Shift = 2`, `AvgSpread = 5`
- `Disparity(x + c, y + c) = Disparity(x, y)` #h(1em) `Disparity(kx, ky) = Disparity(x, y)`

#v(0.5em)
$Disparity$ expresses a difference between groups in a way that does not depend on the original measurement units.
A disparity of 0.5 means the groups differ by half a spread unit; 1.0 means one full spread unit.
Being dimensionless allows comparison of effect sizes across different studies, metrics, or measurement scales.
What counts as a "large" or "small" disparity depends entirely on the domain and what matters practically in a given application.
Do not rely on universal thresholds; interpret the number in context.

#pagebreak()
=== Algorithm <sec-alg-disparity>

#include "disparity-algorithm.typ"

#pagebreak()
=== Tests

#include "disparity-tests.typ"

=== References

#include "disparity-references.typ"
