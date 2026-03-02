#import "/manual/definitions.typ": *

== SignMargin <sec-sign-margin>

$ SignMargin(n, misrate) $

Randomized exclusion count for disjoint-pair sign-test bounds.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$n >= 1$ — number of pairs],
  [$misrate >= 2^(1-n)$ — error rate],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- total margin split evenly between lower and upper tails],
  [*Unit* --- count],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Purpose* --- determines extreme pairwise absolute differences to exclude when constructing bounds],
  [*Based on* --- $"Binomial"(n, 1 / 2)$ CDF inversion with randomized cutoff between adjacent grid points],
  [*Used by* --- #link(<sec-spread-bounds>)[$SpreadBounds$] to select appropriate order statistics of disjoint-pair differences],
  [*Complexity* --- exact for all $n$],
  [*Note* --- randomized to match the requested misrate exactly (deterministic rounding is conservative)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Bounds* #h(2em) $0 <= SignMargin(n, misrate) <= 2n$],
  [*Monotonicity* #h(2em) lower misrate $arrow.r$ smaller margin $arrow.r$ wider bounds],
)

#v(0.3em)
*Example*

Each call returns one of two adjacent grid points (randomized):

- `SignMargin(15, 1e-3)` returns 2 or 4
- `SignMargin(15, 0.01)` returns 4 or 6
- `SignMargin(30, 1e-4)` returns 8 or 10

#v(0.5em)
This is a supporting function that #link(<sec-spread-bounds>)[$SpreadBounds$] uses internally, so most users do not need to call it directly.
It calculates how many extreme disjoint-pair absolute differences should be excluded when constructing bounds, based on the number of pairs and the desired error rate.
Because the $"Binomial"(n, 1 / 2)$ CDF is discrete, exact matching of an arbitrary misrate requires randomizing the cutoff between two adjacent integer values.
A lower misrate (higher confidence) results in a smaller margin, which produces wider bounds.

#pagebreak()
=== Algorithm <sec-alg-sign-margin>

#include "sign-margin-algorithm.typ"

#pagebreak()
=== Tests

#include "sign-margin-tests.typ"
