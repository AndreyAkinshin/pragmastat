#import "/manual/definitions.typ": *

$AdditiveQuantile$ inverts #link(<sec-additive-cumulative>)[$AdditiveCumulative$] using only
  operations whose results IEEE 754 fixes, plus the toolkit's own logarithm, so that seven
  independent implementations produce identical bits.

*Why the domain is split*

The function runs to $-oo$ as $p$ approaches $0$ and to $+oo$ as it approaches $1$.
No polynomial and no ratio of polynomials does that, so a single approximant over $(0, 1)$ is not
  merely inaccurate but the wrong shape.
The domain is therefore cut into a band around the middle, where the function is smooth and
  bounded, and two tails, where a change of variable removes the growth before anything is fitted:

$ q = p - 1\/2, quad r = 0.180625 - q^2, quad w = sqrt(-ln(min(p, 1-p))) $

Inside the band the answer is $q dot R_1(r)$.
The factor of $q$ is what makes the branch odd: the approximant never has to represent the sign,
  and $p = 1\/2$ returns exactly zero because it is multiplied by $q = 0$ rather than by
  cancelling two nearly equal numbers.
Outside it the answer is $plus.minus R_2(w)$ or $plus.minus R_3(w)$, the sign taken from $q$ and
  the magnitude from the smaller of $p$ and $1 - p$.
Working from the smaller side keeps the argument away from $1$, where the subtraction would throw
  away the digits the answer depends on.

*Why the approximants are rational*

Both of the obvious simplifications were tried and rejected by measurement, and the numbers are
  reproduced by `tests/oracles/fit_additive_quantile.py`:

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Attempt*], [*Reaches*], [*Verdict*],
  table.hline(),
  [Polynomial over the central band, degree 12],
  [$1.6 dot 10^(-7)$],
  [Coefficients run to $10^8$ with alternating signs: the power basis reporting that it is the
    wrong basis],
  [One rational fit across the whole tail, $(8, 8)$],
  [$3.0 dot 10^(-10)$],
  [The band spans $w$ from $1.6$ to $27$ and no approximant of a transcribable length is uniform
    over it],
  [Three rational branches],
  [below $1.1 dot 10^(-16)$ each],
  [Shipped],
  table.hline(),
)

#v(0.3em)
The tail therefore splits again at $w = 5$, which is where the near branch stops being
  comfortable.
The three fits reach $7.0 dot 10^(-17)$, $2.2 dot 10^(-17)$ and $4.6 dot 10^(-18)$, all below the
  binary64 relative rounding floor of $1.1 dot 10^(-16)$, so the approximation is no longer what
  limits the result.

*What the assembled function measures*

Worst error $7.7$ units in the last place over a sweep of $320000$ arguments across the whole open
  interval, with a mean of $1.03$.
The gap between that and the fits is arithmetic rather than approximation: two Horner chains of
  nine to eleven terms and one division, evaluated in binary64.
That is measured, not inferred.
At the argument where the assembled function is $7.7$ units away, the same coefficients and the
  same $w$ evaluated in $60$-digit arithmetic land $0.05$ units from the true value, so every
  remaining unit enters through the intermediate roundings.
The error is spread evenly for the same reason, $6.3$ to $7.7$ units in the last place on each of
  the three branches, which is what accumulation looks like and what one weak fit does not.

A sweep is the only instrument that reports this number.
The stored fixture is four regular grids and reaches $4.4$, because a grid samples a branch evenly
  and a worst case does not live at evenly spaced arguments.
Any figure here that a reader wants to check has to come from a sweep of comparable size.

A Newton step on $AdditiveCumulative$ was tried as a refinement and made the result worse rather
  than better, measured on the arguments of the day.
That function carries about $10^(-15)$ of relative error and its accuracy is scoped to
  $abs(z) <= 6$, so refining a $7 dot 10^(-17)$ fit against it converges toward the less accurate
  of the two.
Taking the step only in the lower tail, where no cancellation occurs, does not rescue it either:
  the cutoff at $abs(z) = 6$ is exactly where the refinement would have been needed.

#source-include("cs/Pragmastat/Functions/AdditiveQuantile.cs", "cs")
