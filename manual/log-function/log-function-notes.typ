#import "/manual/definitions.typ": *

*Why the toolkit supplies its own logarithm*

IEEE 754 fixes the result of every arithmetic operation and of the square root (@ieee2019).
For the logarithm and the other transcendental functions the standard recommends correct rounding
  rather than requiring it, which leaves conforming implementations free to differ.
Two libraries can both satisfy the standard and still return neighboring values for the same
  argument, and the standard offers no way to prefer one over the other.

This is not a remark about exotic inputs.
On one machine, asking two of the runtimes this toolkit targets for the logarithm of $3$ returns
  two different doubles:

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Source*], [$ln 3$], [*Payload*],
  table.hline(),
  [System library, through R and Python], [1.0986122886681098], [`3ff193ea7aad030b`],
  [V8, through TypeScript], [1.0986122886681096], [`3ff193ea7aad030a`],
  table.hline(),
)

#v(0.5em)
For most numerical work this freedom costs nothing, since a last-bit difference in a logarithm
  propagates into a last-bit difference in the answer.
$Ratio$ is not such a case.
It is $exp(Shift(log vx, log vy))$: the log-transformed samples go into a shift, the shift takes a
  median of all pairwise differences, and a median returns one of those differences rather than
  computing a new number.
Moving one input by a single bit can change which difference is the middle one, and the distance
  between two neighboring pairwise differences is set by the data rather than by the arithmetic.
So the error does not stay the size of the perturbation that caused it.

#v(0.5em)
*What that was costing, measured*

The conformance experiment recomputes every estimator with each call to the logarithm, the
  exponential, the cosine and the general power returning the neighboring representable value,
  which is the smallest difference two conforming libraries may legitimately have.

Before this function existed, that perturbation moved $Ratio$ on every one of the 180 probed
  inputs and $RatioBounds$ on 2280 of 2364.
Both are now built from this logarithm and from #link(<sec-exp-function>)[$ExpFunction$], and the
  same perturbation moves neither on any input, which is what let both suites move from the
  approximate conformance class to the exact one.

#v(0.5em)
*Reproducible rather than closest*

The goal is that seven implementations agree, not that the answer is the best available double.
Those are different targets, and where they part company this function follows the first.

$ln 3$ is again the example.
The correctly rounded double is the one the system library returns, $0.41$ units in the last place
  from the true value; this function returns the other one, $0.59$ units away, agreeing with V8
  rather than with glibc.
Both are faithful, meaning within one unit in the last place of the true logarithm, and the
  toolkit claims faithfulness rather than correct rounding.
Across the shared fixture, $98.22%$ of results are correctly rounded and the worst case is $0.63$
  units, measured against a 60-digit reference rather than against another implementation.

A correctly rounded logarithm is achievable and is the subject of a literature of its own
  (@ziv1991, @muller2016), at a cost in evaluation time and in table size that a statistical
  toolkit has no reason to pay.
What it cannot buy is the property that matters here: two implementations that are both correctly
  rounded do agree, but only as long as both remain so on every input, which no runtime promises
  and none of the seven guarantees across versions.
