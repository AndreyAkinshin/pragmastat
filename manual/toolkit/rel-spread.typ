#import "/manual/definitions.typ": *

=== RelSpread

$ RelSpread(vx) = Spread(vx) / abs(Center(vx)) $

Relative dispersion normalized by location.

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — robust coefficient of variation],
  [*Domain* — $Center(vx) != 0$],
  [*Assumptions* — #link(<sec-positivity>)[`positivity(x)`]],
  [*Unit* — dimensionless],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Scale invariance* #h(2em) $RelSpread(k dot vx) = RelSpread(vx)$],
  [*Non-negativity* #h(2em) $RelSpread(vx) >= 0$],
)

#v(0.3em)
*Example*

- `RelSpread([1, 3, 5, 7, 9]) = 0.8`
- `RelSpread(5x) = 0.8`

#v(0.5em)
$RelSpread$ compares how "noisy" different datasets are, even if they have completely different scales or units.
A dataset centered around 100 with spread of 10 has the same relative variability as one centered around 1000 with spread of 100.
Both show 10% relative variation, and $RelSpread$ captures exactly this.
This makes it useful for comparing measurement quality across different experiments, instruments, or physical quantities where absolute numbers are not directly comparable.
