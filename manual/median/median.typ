#import "/manual/definitions.typ": *

== Median <sec-median>

$ Median(vx) = cases(
  x_(((n+1)\/2)) & "if" n "is odd",
  (x_((n\/2)) + x_((n\/2+1))) / 2 & "if" n "is even"
) $

The value splitting a sorted sample into two equal parts.

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — 50th percentile, second quartile (Q2)],
  [*Asymptotic* — value where $P(X <= Median) = 0.5$],
  [*Complexity* — $O(n)$],
  [*Domain* — any real numbers],
  [*Unit* — same as measurements],
)

#v(0.5em)
*Notation*

#list(marker: none, tight: true,
  [$x_((1)), ..., x_((n))$ #h(2em) order statistics (sorted sample)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift equivariance* #h(2em) $Median(vx + k) = Median(vx) + k$],
  [*Scale equivariance* #h(2em) $Median(k dot vx) = k dot Median(vx)$],
)

#v(0.3em)
*Example*

- `Median([1, 2, 3, 4, 5]) = 3`
- `Median([1, 2, 3, 4]) = 2.5`

#v(0.5em)
$Median$ provides maximum protection against outliers and corrupted data.
It achieves a 50% breakdown point, meaning that up to half of the data can be arbitrarily bad before the estimate becomes meaningless.
However, this extreme robustness comes at a cost: the median is less precise than #link(<sec-center>)[$Center$] when data is clean.
For most practical applications, #link(<sec-center>)[$Center$] offers a better tradeoff (29% breakdown with 95% efficiency).
Reserve $Median$ for situations with suspected contamination levels above 29% or when the strongest possible robustness guarantee is needed.

#pagebreak()
=== Algorithm <sec-alg-median>

#include "median-algorithm.typ"

#pagebreak()
=== Tests

#include "median-tests.typ"
