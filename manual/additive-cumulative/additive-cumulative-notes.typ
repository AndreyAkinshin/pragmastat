#import "/manual/definitions.typ": *

*Why accuracy matters for this function*

A distribution function accurate to seven digits satisfies most uses it is put to.
The Edgeworth branches of the two margins are not among them, because of what happens to the value
  after it is computed.

Each margin compares the value against a misrate, the comparison decides an integer index, and that
  index selects an order statistic out of the sorted sample.
No intermediate quantity absorbs a small error.
Either the comparison yields the same index and the answer is identical to the last bit, or it
  yields a different one and the reported bound moves to a different observation, separated by a
  distance the data sets rather than the arithmetic.

This property determines how the function is built.
The requirement is not accuracy in the abstract but accuracy sufficient to select the correct
  integer, together with agreement close enough that all seven implementations select the same one.

#v(0.5em)
*The earlier approximation*

The function was previously ACM Algorithm 209 (@ibbetson1963), contributed by Ibbetson in 1963 and
  built on polynomial approximations credited in the published note to A. M. Murray of Aberdeen
  University.
That approximation is short and requires no library calls, which is why it was chosen.

Its accuracy was assessed twice in print soon after publication (@pike1964, @hill1967), the second
  time alongside six other algorithms for the same function.
Measured here against a reference accurate to 36 digits, its worst relative error reaches
  $4.5 dot 10^(-7)$ near the center, where the margins spend most of their evaluations.
That gap reaches the returned value rather than remaining beneath it.
Recomputing the margins over the Edgeworth region with an accurate distribution function changes
  13 of 2961 of them, so on roughly $0.4%$ of those inputs all seven implementations agreed on an
  integer that was not the correct one.

Agreement across implementations does not imply accuracy.
A shared approximation moves all seven the same way, so comparing them against each other cannot
  detect an error inside the approximation itself; only a reference can.

#v(0.5em)
*What replaced it*

The construction described in the algorithm section reaches a worst relative error of
  $9.1 dot 10^(-15)$ over $abs(z) <= 6$, closer by a factor of $5 dot 10^7$ over the central region.
Both approximations stop somewhere in the tail and return exactly zero beyond it, the older one at
  $abs(z) = 6$ and this one at $abs(z) = 6.08$; the difference between them is accuracy across the
  range the estimators use, not the presence or absence of a cutoff.

The 13 margins it changes are byte-identical to those produced by an independent implementation of
  the complementary error function taken from a platform library.
Two implementations sharing no coefficients and no structure, agreeing on which 13 margins change
  and on what each changes to, establishes the result in a way that comparing the new function
  against its own reference cannot: that comparison would show only that the fit converged.

#v(0.5em)
*The cost of accuracy in the tail*

Reproducing the tail requires an exponential, and the earlier approximation avoided one precisely
  because it required no library calls at all.
Introducing the exponential introduces a dependency on a function IEEE 754 leaves unspecified,
  which is the subject of the #link(<sec-exp-function>)[$ExpFunction$] section and the reason that
  section exists.

The tail regime is not what introduces that dependency.
The density term beside the distribution function in both expansions is itself an exponential, so
  the path from sample to order statistic passes through a library call either way.
#link(<sec-conformance-classes>)[Conformance Classes] describes how that path is measured.

#v(0.5em)
*What the construction deliberately omits*

The function is not correctly rounded, and no attempt is made to make it so.
Correct rounding would require carrying extended precision through both polynomial chains in seven
  languages, two of which provide no inexpensive way to do it.

The accuracy delivered clears the requirement by a wide margin.
The quantity being approximated is itself an approximation to a discrete distribution, and the
  truncation error of the Edgeworth expansion exceeds $9.1 dot 10^(-15)$ by orders of magnitude.
Accuracy beyond that point improves no answer the toolkit returns.
