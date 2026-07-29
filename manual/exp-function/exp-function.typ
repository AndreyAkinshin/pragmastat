#import "/manual/definitions.typ": *

== ExpFunction <sec-exp-function>

$ ExpFunction(y) $

The exponential function, supplied by the toolkit rather than by the platform.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$y in (-oo, +oo)$ --- a point on the real line],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- $e^y$],
  [*Unit* --- dimensionless],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Purpose* --- removes the last library call from the path that decides an order statistic],
  [*Used by* --- #link(<sec-additive-cumulative>)[$AdditiveCumulative$] in its tail regime, and the density term of
    both margins' Edgeworth expansions],
  [*Built from* --- a range reduction and one polynomial, using only operations IEEE 754 pins],
  [*Accuracy* --- never more than one unit in the last place from the correctly rounded result;
    worst relative error $1.4 dot 10^(-16)$ under a search of the range where the result is normal],
  [*Note* --- all seven implementations return identical bits on every argument],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Monotonicity* #h(2em) non-decreasing in $y$, with no inversions on any tested band],
  [*Positivity* #h(2em) $ExpFunction(y) > 0$ for every $y$ above the underflow cutoff],
  [*Identity* #h(2em) $ExpFunction(0) = 1$ exactly],
  [*Cutoffs* #h(2em) $+oo$ above $y = 709.79$, zero below $y = -745.2$, NaN preserved],
)

#v(0.3em)
*Example*

- `ExpFunction(0) = 1`
- `ExpFunction(1) = 2.7182818284590455`
- `ExpFunction(-18.5) = 9.237449661970594e-09`

#v(0.5em)
A statistical toolkit does not normally supply its own exponential.
This one does because IEEE 754 fixes the result of every arithmetic operation and of the square
  root while fixing nothing about the exponential, so two conforming libraries may return
  neighboring values for the same argument.

That freedom is ordinarily harmless.
Here the value is compared against a misrate and the comparison selects an order statistic, so a
  last-bit difference produces a different confidence interval from identical inputs.

#pagebreak()
=== Algorithm <sec-alg-exp-function>

#include "exp-function-algorithm.typ"

#pagebreak()
=== Notes

#include "exp-function-notes.typ"

#pagebreak()
=== Tests

#include "exp-function-tests.typ"

=== References

#include "exp-function-references.typ"
