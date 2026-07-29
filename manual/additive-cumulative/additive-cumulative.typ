#import "/manual/definitions.typ": *

== AdditiveCumulative <sec-additive-cumulative>

$ AdditiveCumulative(z) $

The distribution function of the standard $Additive$ ('Normal') distribution.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$z in (-oo, +oo)$ --- a standardized deviation, in units of spread from the center],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- the share of a standard $Additive$ distribution at or below $z$],
  [*Unit* --- probability, in $[0, 1]$],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Purpose* --- supplies the leading term of the Edgeworth expansions used by both margins],
  [*Used by* --- #link(<sec-pairwise-margin>)[$PairwiseMargin$] and
    #link(<sec-signed-rank-margin>)[$SignedRankMargin$] above their exact-computation limits],
  [*Built from* --- two polynomial chains and #link(<sec-exp-function>)[$ExpFunction$]],
  [*Accuracy* --- worst relative error $9.1 dot 10^(-15)$ over $abs(z) <= 6$],
  [*Note* --- evaluated identically by all seven implementations, to the last bit],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Reflection* #h(2em) $AdditiveCumulative(-z) = 1 - AdditiveCumulative(z)$ to within $6 dot 10^(-17)$ in
    absolute terms. The two sides are not interchangeable: past the center $1 - AdditiveCumulative(z)$
    loses most of its significant digits to cancellation, which is why the tail is computed from
    $"erfc"$ rather than by subtraction],
  [*Bounds* #h(2em) $0 <= AdditiveCumulative(z) <= 1$],
  [*Monotonicity* #h(2em) non-decreasing in $z$ to within two units in the last place; adjacent
    representable arguments invert by at most that much, inside the central chain],
  [*Center* #h(2em) $AdditiveCumulative(0) = 1 \/ 2$ exactly],
)

#v(0.3em)
*Example*

- `AdditiveCumulative(0) = 0.5`
- `AdditiveCumulative(1) = 0.8413447460685429`
- `AdditiveCumulative(-1.959963984540054) = 0.025000000000000012`

#v(0.5em)
This is a supporting function that #link(<sec-pairwise-margin>)[$PairwiseMargin$] and
  #link(<sec-signed-rank-margin>)[$SignedRankMargin$] use internally, so most users do not need to
  call it directly.
No estimator in the toolkit assumes an $Additive$ ('Normal') distribution: the function appears only
  inside the
  approximation both margins fall back to once the exact distribution becomes too expensive to
  compute.

Because the value it returns decides which order statistic a bound reports, its accuracy matters
  more than that modest role suggests.
The notes below give the argument.

#pagebreak()
=== Algorithm <sec-alg-additive-cumulative>

#include "additive-cumulative-algorithm.typ"

#pagebreak()
=== Notes

#include "additive-cumulative-notes.typ"

#pagebreak()
=== Tests

#include "additive-cumulative-tests.typ"

=== References

#include "additive-cumulative-references.typ"
