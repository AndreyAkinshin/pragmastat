#import "/manual/definitions.typ": *

== PairwiseMargin

$ PairwiseMargin(n, m, misrate) $

Exclusion count for dominance-based bounds.

#v(0.3em)
#list(marker: none, tight: true,
  [*Purpose* — determines extreme pairwise differences to exclude when constructing bounds],
  [*Based on* — distribution of $Dominance(vx, vy) = sum_(i=1)^n sum_(j=1)^m bb(1)(x_i > y_j)$ under random sampling],
  [*Returns* — total margin split evenly between lower and upper tails],
  [*Used by* — $ShiftBounds$ to select appropriate order statistics],
  [*Complexity* — exact for small samples, approximated for large (see #link(<sec-fast-pairwise-margin>)[Fast PairwiseMargin])],
  [*Domain* — $n, m >= 1$, $misrate in (0, 1)$],
  [*Unit* — count],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Symmetry* #h(2em) $PairwiseMargin(n, m, misrate) = PairwiseMargin(m, n, misrate)$],
  [*Bounds* #h(2em) $0 <= PairwiseMargin(n, m, misrate) <= n m$],
)

#v(0.3em)
*Example*

- `PairwiseMargin(30, 30, 1e-6) = 276`
- `PairwiseMargin(30, 30, 1e-4) = 390`
- `PairwiseMargin(30, 30, 1e-3) = 464`

#v(0.5em)
This is a supporting function that $ShiftBounds$ uses internally, so most users do not need to call it directly.
It calculates how many extreme pairwise differences should be excluded when constructing bounds, based on sample sizes and the desired error rate.
When you request a lower misrate (higher confidence), the margin becomes smaller, which produces wider bounds.
The function automatically chooses between exact computation for small samples and a fast approximation for large samples.
