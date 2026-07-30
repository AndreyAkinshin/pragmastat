#import "/manual/definitions.typ": *

*What it replaced*

The value used to be computed as $exp$ of a Stirling series for the log-gamma function.
That formulation had two independent problems, and the recurrence removes both.

The first is accuracy.
Measured against exact integer arithmetic over 1985 pairs spanning $4 <= n <= 400$, the Stirling
  form was inexact on $99.8%$ of them and reached a worst relative error of $4.9 dot 10^(-13)$.
The recurrence is inexact on $50.7%$ and reaches $2.2 dot 10^(-15)$, two hundred times closer.
That gap mattered because the quantity is a domain boundary: the specification defines the
  admissible misrate as $misrate >= 2 \/ binom(n+m, n)$, and an approximated boundary accepts
  inputs it should reject.

The second is reproducibility, and it is the reason the change was necessary rather than merely
  desirable.
Stirling reached its answer through a logarithm and an exponential, and every language takes those
  from a different library.
Perturbing them by a single unit in the last place, which is the smallest difference two conforming
  implementations can legitimately have, moved the computed misrate floor on 75579 of 79797 sample
  size pairs.
The recurrence calls nothing, so the same perturbation moves none of them.

#v(0.5em)
*Where the accuracy stops mattering*

Above the threshold the recurrence is not exact, and the manual does not claim otherwise.
What it claims is narrower and is what the estimators need: the seven implementations perform one
  sequence of roundings, so they return one value, and that value is close enough to the true
  boundary that no admissible misrate is misclassified.

The recurrence also overflows earlier than the true value does.
The intermediate $a_(i-1) dot (n - k + i)$ runs up to $n \/ 2$ times above the final result, so the
  central coefficient returns infinity from $n = 1021$ while the exact value stays inside binary64
  until $n = 1030$.
Across that window the misrate floor $2 \/ binom(n+m, n)$ collapses from about $7 dot 10^(-307)$ to
  zero, which only a misrate below $10^(-306)$ could tell apart, and every one of those is dominated
  by the one-sample floor anyway.
Deferring the divide to keep the intermediate small would avoid the window, at the cost of a
  different sequence of roundings; only one sequence is shared, so the window stays.

#v(0.5em)
*The pattern this completes*

Three functions on the deciding path came from somewhere the specification does not fix, and the
  toolkit answered each differently.

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Function*], [*Problem*], [*Answer*],
  table.hline(),
  [#link(<sec-additive-cumulative>)[$AdditiveCumulative$]],
  [an approximation chosen for being identical, not for being close],
  [fit an accurate one, identically],
  [#link(<sec-exp-function>)[$ExpFunction$]],
  [the platform exponential, which no two libraries agree on],
  [write the exponential],
  [$BinomialCoefficient$],
  [an exact integer computed through two library calls],
  [compute it as an integer],
  table.hline(),
)

#v(0.3em)
The third answer is the cheapest and the one to look for first.
It applies whenever the quantity has an exact formulation that the code reached indirectly, and
  it costs nothing: no coefficients to fit, no accuracy to trade, nothing to keep in agreement
  across seven languages except the order of the operations.
The same move later removed the logarithm from #link(<sec-sign-margin>)[$SignMargin$], which had
  been inverting a binomial distribution function in log space for the same reason this function
  had been going through Stirling.
