#import "/manual/definitions.typ": *

== AdditiveQuantile <sec-additive-quantile>

$ AdditiveQuantile(p) $

The inverse standard normal CDF, supplied by the toolkit rather than by the platform.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$p in (0, 1)$ --- a probability],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- the $v$ with $AdditiveCumulative(v) = p$],
  [*Unit* --- dimensionless],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Purpose* --- gives the parametric models a normal quantile every language agrees on],
  [*Used by* --- nothing yet. $Additive$ and $Multiplic$ take it up together, because a sampling
    stream cannot change in one language ahead of the others],
  [*Built from* --- three rational approximants, arithmetic, a square root and
    #link(<sec-log-function>)[$LogFunction$]; nothing the standard leaves free],
  [*Accuracy* --- worst measured error $7.7$ units in the last place over $320000$ arguments,
    $6.3$ to $7.7$ on each of the three branches rather than concentrated in one; the test suite
    holds it under $16$],
  [*Note* --- all seven implementations return identical bits on every argument],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Center* #h(2em) $AdditiveQuantile(1\/2) = 0$ exactly, with a positive sign],
  [*Oddness* #h(2em) the central branch satisfies
    $AdditiveQuantile(1\/2 + q) = -AdditiveQuantile(1\/2 - q)$ exactly],
  [*Monotonicity* #h(2em) increasing in $p$, with no inversions on any tested argument],
  [*Cutoffs* #h(2em) $-oo$ at zero and below, $+oo$ at one and above, NaN preserved],
  [*Reach* #h(2em) finite at the smallest positive double, where it returns $-38.467...$],
)

#v(0.3em)
*Example*

- `AdditiveQuantile(0.5) = 0`
- `AdditiveQuantile(0.975) = 1.959963984540054`
- `AdditiveQuantile(1e-300) = -37.047096299361193`

#v(0.5em)
A quantile is a value a caller compares and reports, and IEEE 754 fixes nothing about the function
  that produces it.
Two conforming libraries may return neighboring values for the same probability, which is the same
  freedom that made the #link(<sec-exp-function>)[$ExpFunction$] and the
  #link(<sec-log-function>)[$LogFunction$] the toolkit's own.

The tails are what make this one harder than those two.
The function runs to infinity at both ends of its domain, so no single approximant covers
  $(0, 1)$, and the answer is to split the domain and to approximate by a ratio of polynomials
  rather than by one polynomial.
Both of those were established here by measurement rather than inherited: the numbers are under
  Algorithm.

#pagebreak()
=== Algorithm <sec-alg-additive-quantile>

#include "additive-quantile-algorithm.typ"

#pagebreak()
=== Notes

#include "additive-quantile-notes.typ"

#pagebreak()
=== Tests

#include "additive-quantile-tests.typ"

=== References

#include "additive-quantile-references.typ"
