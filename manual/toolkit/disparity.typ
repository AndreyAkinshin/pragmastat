#import "/manual/definitions.typ": *

== Disparity

$ Disparity(vx, vy) = Shift(vx, vy) / AvgSpread(vx, vy) $

Robust effect size (shift normalized by pooled dispersion).

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — robust Cohen's d (@cohen1988; estimates differ due to robust construction)],
  [*Domain* — $AvgSpread(vx, vy) > 0$],
  [*Assumptions* — #link(<sec-sparity>)[`sparity(x)`], #link(<sec-sparity>)[`sparity(y)`]],
  [*Unit* — spread units],
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
Use $Disparity$ when you want to express a difference between groups in a way that does not depend on the original measurement units.
A disparity of 0.5 means the groups differ by half a spread unit; 1.0 means one full spread unit.
Because it is dimensionless, you can compare effect sizes across different studies, metrics, or measurement scales.
What counts as a "large" or "small" disparity depends entirely on your domain and what matters practically in your application.
Do not rely on universal thresholds; interpret the number in context.
