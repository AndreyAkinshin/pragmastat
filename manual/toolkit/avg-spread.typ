#import "/manual/definitions.typ": *

== AvgSpread

$ AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m) $

Weighted average of dispersions (pooled scale).

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — robust pooled standard deviation],
  [*Domain* — any real numbers],
  [*Assumptions* — #link(<sec-sparity>)[`sparity(x)`], #link(<sec-sparity>)[`sparity(y)`]],
  [*Unit* — same as measurements],
  [*Caveat* — $AvgSpread(vx, vy) != Spread(vx union vy)$ (pooled scale, not concatenated spread)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Self-average* #h(2em) $AvgSpread(vx, vx) = Spread(vx)$],
  [*Symmetry* #h(2em) $AvgSpread(vx, vy) = AvgSpread(vy, vx)$],
  [*Scale equivariance* #h(2em) $AvgSpread(k dot vx, k dot vy) = abs(k) dot AvgSpread(vx, vy)$],
  [*Mixed scaling* #h(2em) $AvgSpread(k_1 dot vx, k_2 dot vx) = (abs(k_1) + abs(k_2)) / 2 dot Spread(vx)$],
)

#v(0.3em)
*Example*

- `AvgSpread(x, y) = 5` where `Spread(x) = 6`, `Spread(y) = 4`, `n = m`
- `AvgSpread(x, y) = AvgSpread(y, x)`

#v(0.5em)
Use $AvgSpread$ when you need a single number representing the typical variability across two groups.
It combines the spread of both samples, giving more weight to larger samples since they provide more reliable estimates.
This pooled spread serves as a common reference scale, which is essential when you want to express a difference in relative terms.
$Disparity$ uses $AvgSpread$ internally to normalize the shift into a scale-free effect size.
