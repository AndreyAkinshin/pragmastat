#import "/manual/definitions.typ": *

*What the accuracy is measured against*

The reference is `tests/oracles/normal_quantile.py`, which evaluates $AdditiveQuantile$ to eighty
  significant digits by Newton iteration on a high-precision $AdditiveCumulative$.

That reference had to be repaired before it could serve as one.
It computed the cumulative as $1\/2 + 1\/2 dot "erf"(x\/sqrt(2))$, which is accurate in the body
  and worthless in the tail: $"erf"$ approaches $-1$ there, adding it to $1$ cancels every
  significant digit, and $AdditiveCumulative(-21)$ came back as a flat zero with nothing for
  Newton to descend.
It failed outright past $p = 10^(-150)$.
The tail now goes through the Laplace continued fraction, which computes the tail mass directly and
  never forms a difference of nearly equal numbers, so the reference reaches the smallest positive
  double at a relative accuracy of $10^(-77)$.

The defect had gone unnoticed because the only consumer was a grid of 395 points in the body of the
  distribution, where the old path is correct.
A reference is only as trustworthy as the region anyone has asked it about.

*The symmetry that holds, and the one that does not*

The central branch is exactly odd about $1\/2$: it consumes $q = p - 1\/2$ and multiplies the
  approximant by it, so the two sides of the center are the same computation with one sign.

The tails are not, and the manual states so rather than implying otherwise.
They work from $min(p, 1 - p)$, and that subtraction is where the symmetry is lost: $1 - 0.95$ is
  $0.050000000000000044$ rather than $0.05$, so the two sides are never handed the same number.
The test suites assert oddness only on the central branch, and only at offsets that survive the
  round trip $1\/2 + q - 1\/2 = q$.
The band edge itself fails that round trip, since $0.5 + 0.425 - 0.5$ returns
  $0.42500000000000004$ and lands its point in the tail branch while its mirror stays central.

*Reach*

The far branch is the only one that sees the extreme tail, and it is exercised there: the smallest
  positive double is the last argument a caller can pass, and its quantile is a finite
  $-38.467...$ rather than an infinity or a saturated bound.
The upper side has no counterpart at that scale, because $1 - p$ stops resolving long before $p$
  does: any argument past $1 - 2^(-53)$ is simply $1$.
