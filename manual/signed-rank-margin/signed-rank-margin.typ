#import "/manual/definitions.typ": *

== SignedRankMargin <sec-signed-rank-margin>

$ SignedRankMargin(n, misrate) $

Exclusion count for one-sample signed-rank based bounds.

#v(0.3em)
#list(marker: none, tight: true,
  [*Purpose* — determines extreme pairwise averages to exclude when constructing bounds],
  [*Based on* — Wilcoxon signed-rank distribution under weak symmetry],
  [*Returns* — total margin split evenly between lower and upper tails],
  [*Used by* — #link(<sec-center-bounds>)[$CenterBounds$] to select appropriate order statistics],
  [*Complexity* — exact for $n <= 63$, approximated for larger],
  [*Domain* — $n >= 2$, $misrate >= 2^(1-n)$],
  [*Unit* — count],
  [*Note* — assumes weak symmetry and weak continuity],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Bounds* #h(2em) $0 <= SignedRankMargin(n, misrate) <= n(n+1)\/2$],
  [*Monotonicity* #h(2em) lower misrate $arrow.r$ smaller margin $arrow.r$ wider bounds],
)

#v(0.3em)
*Example*

- `SignedRankMargin(20, 1e-3) = 44`
- `SignedRankMargin(30, 1e-4) = 112`
- `SignedRankMargin(100, 1e-6) = 706`

#v(0.5em)
This is a supporting function that #link(<sec-center-bounds>)[$CenterBounds$] uses internally, so most users do not need to call it directly.
It calculates how many extreme pairwise averages should be excluded when constructing bounds, based on sample size and the desired error rate.
A lower misrate (higher confidence) results in a smaller margin, which produces wider bounds.
The function automatically chooses between exact computation for small samples and a fast approximation for large samples.

#pagebreak()
=== Algorithm <sec-alg-signed-rank-margin>

#include "signed-rank-margin-algorithm.typ"

#pagebreak()
=== Tests

#include "signed-rank-margin-tests.typ"
