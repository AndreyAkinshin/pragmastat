#import "/manual/definitions.typ": *

=== Spread

$ Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j) $

Robust measure of dispersion (variability, scatter).

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — Shamos scale estimator],
  [*Asymptotic* — median of the absolute difference between two random measurements from $X$],
  [*Complexity* — $O(n^2 log n)$ naive, $O(n log n)$ fast (see #link(<sec-fast-spread>)[Fast Spread])],
  [*Domain* — any real numbers],
  [*Assumptions* — #link(<sec-sparity>)[`sparity(x)`]],
  [*Unit* — same as measurements],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift invariance* #h(2em) $Spread(vx + k) = Spread(vx)$],
  [*Scale equivariance* #h(2em) $Spread(k dot vx) = abs(k) dot Spread(vx)$],
  [*Non-negativity* #h(2em) $Spread(vx) >= 0$],
)

#v(0.3em)
*Example*

- `Spread([0, 2, 4, 6, 8]) = 4`
- `Spread(x + 10) = 4` #h(1em) `Spread(2x) = 8`

#v(0.5em)
*References*

- @shamos1976

#v(0.5em)
$Spread$ measures how much measurements vary from each other.
It serves the same purpose as standard deviation but does not explode with outliers or heavy-tailed data.
The result comes in the same units as the measurements, so if $Spread$ is 5 milliseconds, that indicates how much values typically differ.
Like $Center$, it tolerates up to 29% corrupted data.
When comparing variability across datasets, $Spread$ gives a reliable answer even when standard deviation would be misleading or infinite.
