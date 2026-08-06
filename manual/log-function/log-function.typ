#import "/manual/definitions.typ": *

== LogFunction <sec-log-function>

$ LogFunction(x) $

The natural logarithm, supplied by the toolkit rather than by the platform.

#v(0.3em)
*Input*

#list(marker: none, tight: true,
  [$x in [0, +oo)$ --- a non-negative real number],
)

#v(0.3em)
*Output*

#list(marker: none, tight: true,
  [*Value* --- $ln x$],
  [*Unit* --- dimensionless],
)

#v(0.3em)
*Notes*

#list(marker: none, tight: true,
  [*Purpose* --- removes the last library call from the path that decides which pairwise element a
    median selects],
  [*Used by* --- #link(<sec-ratio>)[$Ratio$] and #link(<sec-ratio-bounds>)[$RatioBounds$], which
    log-transform both samples before the shift],
  [*Built from* --- a range reduction and one polynomial, using only operations IEEE 754 pins],
  [*Accuracy* --- never more than one unit in the last place from the correctly rounded result;
    98.22% of the shared fixture is correctly rounded, and the worst case over it is 0.63 units],
  [*Note* --- all seven implementations return identical bits on every argument],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Monotonicity* #h(2em) non-decreasing in $x$, with no inversions on any tested band],
  [*Identity* #h(2em) $LogFunction(1) = 0$ exactly, and the zero is positive],
  [*Inverse* #h(2em) $ExpFunction(LogFunction(x)) approx x$, to within the amplification $abs(ln x)$
    applies to a half-unit argument],
  [*Edges* #h(2em) $-oo$ at zero, NaN below it, $+oo$ preserved, NaN preserved],
)

#v(0.3em)
*Example*

- `LogFunction(1) = 0`
- `LogFunction(2) = 0.6931471805599453`
- `LogFunction(10) = 2.302585092994046`

#v(0.5em)
The reason is the one given for #link(<sec-exp-function>)[$ExpFunction$], one step sharper.

IEEE 754 fixes the result of every arithmetic operation and of the square root while fixing nothing
  about the logarithm, so two conforming libraries may return neighboring values for the same
  argument.
$Ratio$ is $exp(Shift(log vx, log vy))$: the logarithm feeds a median, and a median selects an
  element out of the set of pairwise differences.
A last-bit difference in the logarithm therefore does not move the answer slightly.
It changes which element is selected, and the gap between two neighboring pairwise elements is
  whatever the data says it is.

#pagebreak()
=== Algorithm <sec-alg-log-function>

#include "log-function-algorithm.typ"

#pagebreak()
=== Notes

#include "log-function-notes.typ"

#pagebreak()
=== Tests

#include "log-function-tests.typ"

=== References

#include "log-function-references.typ"
