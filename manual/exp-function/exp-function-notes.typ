#import "/manual/definitions.typ": *

*Why the toolkit supplies its own exponential*

IEEE 754 fixes the result of every arithmetic operation and of the square root (@ieee2019).
For the exponential and the other transcendental functions the standard recommends correct
  rounding rather than requiring it, which leaves conforming implementations free to differ.
Two libraries can both satisfy the standard and still return neighboring values for the same
  argument, and the standard offers no way to prefer one over the other.

For most numerical work this freedom costs nothing, since a last-bit difference in an exponential
  propagates into a last-bit difference in the answer.
The margins are not such a case.
Each of them compares the value against a misrate, the comparison decides an integer index, and
  that index selects an order statistic out of the sorted sample.
A one-bit difference either changes nothing or changes which observation is reported, and the gap
  between adjacent observations is set by the data rather than by the arithmetic.

The effect appears on inputs that sit exactly on a decision boundary.
On one such input the margin evaluates to $2830$ under two of the seven implementations and to
  $2832$ under the remaining five.
Every intermediate quantity agrees to the last bit until the distribution function, where two
  implementations differ by one unit in the last place.
The split follows which exponential each language runtime calls, and nothing about the statistics.

#v(0.5em)
*What each language would otherwise call*

Seven languages do not mean seven exponentials.
Four of them reach the same system library, two descend from a single 1993 source, and one carries
  its own implementation, so the ports fall into three groups rather than seven.
On the crossover described above, the four system-library ports and Kotlin returned $2832$ while
  TypeScript and Go returned $2830$, which is the three groups resolving into two answers.

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Language*], [*Exponential it would call*], [*Consequence*],
  table.hline(),
  [C\#], [System library through the runtime], [Identical to Python, R, Rust],
  [Python], [System library through `math.exp`], [Identical to C\#, R, Rust],
  [R], [System library through `exp`], [Identical to C\#, Python, Rust],
  [Rust], [System library through `f64::exp`], [Identical to C\#, Python, R],
  [Kotlin], [Vendor library, or fdlibm by architecture], [Differs from itself across machines],
  [TypeScript], [fdlibm, until the runtime replaced it], [Differs from the four above],
  [Go], [Its own implementation, in Go], [Differs from every other port],
  table.hline(),
)

Measured over 200001 arguments in the band these estimators reach, the four system-library ports
  agree with each other on every argument, and the 1993 lineage agrees with itself on every
  argument.
Between those two groups the results differ on $10.0%$ of arguments, and the Go implementation
  differs from the system library on $13.6%$ and from the 1993 lineage on $16.1%$.
Every disagreement is one unit in the last place, or two where Go is involved.

Three properties of this landscape each defeat a plausible alternative to writing the function.

Naming one implementation does not narrow the target.
The four ports that share a system library do agree exactly, yet that library ships one algorithm
  compiled twice, with and without fused multiply-add, and selects between the two at load time
  according to processor features.
The variants disagree on $0.06%$ of arguments within a single installation, so even "the system
  library on this machine" names two functions rather than one.

A managed runtime is not one implementation either.
The Kotlin port's exponential substitutes a vendor routine on one processor architecture and falls
  back to the runtime's own copy of the 1993 reference code on another, so the same program returns
  different bits on different machines.
Its two exponentials, both reachable from the same program, disagree on $9.9%$ of arguments.

Pinning a version does not hold the target still.
The TypeScript runtime replaced its exponential outright during the preparation of this manual,
  moving from the 1993 lineage to a correctly rounded implementation, so the port's results would
  have changed with an upgrade that no line of this toolkit requested.

#v(0.5em)
*Accuracy relative to the alternatives*

Writing a function does not make it better than the ones written by specialists.
Each row below reports the worst relative error over the band these estimators reach, measured
  against a sixty-digit reference, together with the share of arguments where the result is the
  correctly rounded double.

#table(
  columns: 3,
  align: (left, right, right),
  stroke: none,
  table.hline(),
  [*Implementation*], [*Worst relative error*], [*Correctly rounded*],
  table.hline(),
  [Table-based system library], [$1.11 dot 10^(-16)$], [$99.9%$],
  [Table-based vendor library], [$1.11 dot 10^(-16)$], [$99.7%$],
  [$ExpFunction$], [$1.30 dot 10^(-16)$], [$90.0%$],
  [1993 reference code], [$1.30 dot 10^(-16)$], [$90.0%$],
  [One language runtime], [$2.55 dot 10^(-16)$], [$86.4%$],
  table.hline(),
)

The function places third of five, matching the long-standing reference implementation that two of
  the runtimes descend from and exceeding one of the runtimes it replaces.

Two qualifications bound what those figures mean.
The tabulated errors are maxima over a uniform grid; searching the same range adversarially raises
  this function's worst relative error to $1.37 dot 10^(-16)$, and the same search would raise the
  other rows similarly.
More importantly, relative error stops describing anything once the result becomes subnormal, below
  about $y = -708$: there the correctly rounded result itself carries a relative error approaching
  $1$, because binary64 has no bits left to carry it.
The margins reach that region whenever the standardized statistic exceeds $37.6$ in magnitude, at
  which point the density term is $10^(-308)$ and contributes nothing to the sum it enters.

The bound that does hold everywhere is stated in units of the last place rather than as a ratio.
Every implementation in the table, this one included, returns one of the two doubles bracketing the
  true value, so the entire spread across the five spans a single bit.
For this construction the bound follows from the arithmetic rather than from sampling: the scaling
  step is exact for every reachable exponent, so the only rounding after the polynomial assembly is
  the final subtraction, whose measured envelope stays below half an ulp.

What this construction offers is not accuracy but agreement: it lies within one bit of every
  alternative and returns identical bits in all seven languages.
An estimator that returns the most accurate of seven different observations serves the reader
  worse than one that returns the same observation everywhere.

#v(0.5em)
*Constructions considered and set aside*

The 1993 reference code (@ng1993) reaches its accuracy through the rational form
  $1 + r + r R \/ (2 - R)$,
  which compensates for a polynomial of lower degree.
Applied on top of the degree-11 chain used here, the division changes no measured quantity, so the
  simpler form is kept.

Library implementations written since 2018 add a table of 128 entries, which reduces the argument
  by a further factor of $128$ and reaches the $99.9%$ row above.
That construction requires 263 constants where this one requires 15, transcribed into seven
  languages, with each transcription an opportunity to mistype a digit.
Two prototypes of it built during this work produced errors an order of magnitude larger than the
  polynomial they were meant to improve.
A function reimplemented seven times should be one that is difficult to get wrong, which the
  table is not.

A correctly rounded implementation would deliver identical results and half-an-ulp accuracy
  together, and is the strongest alternative.
Reaching it requires evaluating at progressively higher precision until the rounding is decided
  (@ziv1991), which is why such implementations carry a multi-precision fallback path.
It requires either fused multiply-add or emulated extended precision in every language, and two of
  the seven provide neither cheaply, which is a substantial cost for a value whose purpose is to be
  compared against a misrate.

#v(0.5em)
*Precedent*

Software that requires identical results across platforms arrives at this solution regularly.
Java specifies its $"StrictMath"$ exponential by naming the 1993 implementation that it must
  reproduce (@oracle2025), which makes the algorithm rather than the accuracy the published
  contract.
A game studio documented the same conclusion in 2014 after recorded sessions failed to replay on
  another operating system, and reported that the difficulty was never precision but obtaining the
  same, possibly imprecise, outcome everywhere.
Other projects avoid the question entirely by running their simulations in fixed-point arithmetic
  with tabulated transcendental functions.
