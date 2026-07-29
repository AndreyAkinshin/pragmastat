#import "/manual/definitions.typ": *

$ ExpFunction(y) $

The $ExpFunction$ test suite contains 4 test cases holding 1032 arguments in total
  (401 working band + 201 Edgeworth band + 401 finite range + 29 boundaries).
All seven implementations compare the results by binary64 payload rather than by tolerance, since
  a tolerance would accept precisely the divergence this suite exists to detect.

*Working band* — 401 points over $[-18.5, 0]$:

- The interval $e^(-t^2)$ reaches for the $t in [1\/2, 4.3]$ that
  #link(<sec-additive-cumulative>)[$AdditiveCumulative$] uses in its tail regime.

*Edgeworth band* — 201 points over $[-40, 0]$:

- Covers the density term of both margins' expansions.

*Finite range* — 401 points over $[-745, 709]$:

- Reaches from $8.2 dot 10^307$ down through the subnormal range to the smallest denormal, so 11
  of the 401 results are subnormal and the last one is the smallest positive double there is.
  The grid stops short of the overflow cutoff on purpose: the largest finite result, $1.79 dot
  10^308$ at $y = 709.78$, is in the boundaries file, where the values that decide behavior belong.

*Boundaries* — 29 points at the values where the construction changes behavior:

- The reduction endpoints at $plus.minus (ln 2) \/ 2$ and $plus.minus 3 (ln 2) \/ 2$
- The underflow cutoff with its neighbors on both sides
- The first denormals, and both signed zeros

Arguments above $709.78$ are absent because their result is infinite and JSON provides no way to
  express one, and neither NaN nor the two infinities can appear as arguments for the same reason.
Each implementation keeps its own unit test for those, where a language can name its own infinity.

*Why this suite exists separately*

Every other suite reaches this function through a margin, which exercises it wherever an Edgeworth
  expansion happens to look and nowhere else.
That leaves the one function on the decision path that IEEE 754 does not fix without direct
  coverage.

Expected values transcribed by hand from another implementation would not serve: such a copy goes
  stale without announcing itself whenever the function changes.
The fixture is generated alongside the others, and the build regenerates it and requires a
  byte-identical result, so it cannot drift from the code that produces it.

*Properties checked beyond the payloads*

*Exactness of the scaling primitive*.
The two implementations that lack a scaling function use exponentiation, whose exactness is a
  property of the platform rather than of the language.
Both compare it against a construction that cannot be wrong, across every exponent the reduction
  can reach, deriving that range from the cutoffs so that widening a cutoff widens the test.

*Monotonicity*.
Both margins binary-search an expansion under the assumption that it is monotone in the index,
  which rests on this function being monotone.
A single inversion would allow the search to return either neighbor, and the two neighbors are
  different confidence bounds.
The sweep walks adjacent representable arguments rather than a grid, since an inversion between
  two neighbors is exactly what the search can hit and a grid steps over it; 180000 of them across
  nine bands produce none.
One port runs it: all seven return identical bits on every argument, which this suite is what
  pins, so a property measured in one holds in all.
The distribution function above does not have this property to the same strength, and the
  expansion the pairwise search bisects inverts by up to one unit in the last place.

*That the fixture is actually compared*.
A fixture loader that finds no files reports success.
Each of the seven pins the argument count, and each was checked by breaking a scratch copy of the
  fixture and confirming the suite fails: a one-unit change in each file, a deleted file, and a
  truncated argument list.
