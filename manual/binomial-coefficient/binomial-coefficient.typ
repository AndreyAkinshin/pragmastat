#import "/manual/definitions.typ": *

== BinomialCoefficient <sec-binomial-coefficient>

$ BinomialCoefficient(n, k) $

The number of $k$-subsets of an $n$-set, computed so that all seven implementations return the
  same bits rather than the most accurate value each language could reach.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$n in NN$ --- the size of the set],
  [$k in NN$, $0 <= k <= n$ --- the size of the subset],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- $binom(n, k)$],
  [*Unit* --- dimensionless],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Purpose* --- supplies the domain boundary on $misrate$ and normalizes an exact distribution,
    both of which decide an integer],
  [*Used by* --- the admissible misrate for two-sample bounds, which is $2 \/ binom(n+m, n)$, and
    the normalizing total of #link(<sec-pairwise-margin>)[$PairwiseMargin$]'s exact distribution],
  [*Built from* --- 64-bit integer arithmetic below $n + k = 62$, and a multiplicative recurrence
    in binary64 at or above it],
  [*Accuracy* --- exact below the threshold; within $2.3 dot 10^(-15)$ relative above it],
  [*Note* --- the threshold is a cross-language contract, not an implementation limit],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Symmetry* #h(2em) $BinomialCoefficient(n, k) = BinomialCoefficient(n, n - k)$, bit for bit],
  [*Edges* #h(2em) $BinomialCoefficient(n, 0) = BinomialCoefficient(n, n) = 1$ exactly],
  [*Domain* #h(2em) zero for $k < 0$ and for $k > n$],
  [*Overflow* #h(2em) $+oo$ once the recurrence leaves binary64, from $n = 1021$ at $k = n \/ 2$],
)

#v(0.3em)
*Example*

- `BinomialCoefficient(10, 3) = 120`
- `BinomialCoefficient(56, 28) = 7648690600760440` (the largest central value binary64 holds exactly)
- `BinomialCoefficient(62, 31) = 465428353255261250` (three units in the last place above the true
  value, and identically so in all seven)

#v(0.5em)
*Why the toolkit computes it*

This is the third of three answers the toolkit gives to one problem, and the only one that removes
  the library call instead of replacing it.

#link(<sec-additive-cumulative>)[$AdditiveCumulative$] and
  #link(<sec-exp-function>)[$ExpFunction$] exist because a value on the deciding path came from a
  function IEEE 754 leaves unfixed, and the answer there was to write the function.
Here the answer was to notice that no such function is needed.
$binom(n, k)$ is an integer, and an integer has no approximation error to disagree about; the
  earlier formulation reached it through $exp$ of a Stirling series, which had both a library call
  and an approximation, and needed neither.

#pagebreak()
=== Algorithm <sec-alg-binomial-coefficient>

#include "binomial-coefficient-algorithm.typ"

#pagebreak()
=== Notes

#include "binomial-coefficient-notes.typ"

#pagebreak()
=== Tests

#include "binomial-coefficient-tests.typ"

=== References

#include "binomial-coefficient-references.typ"

