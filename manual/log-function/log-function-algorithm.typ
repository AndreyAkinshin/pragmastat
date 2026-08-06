#import "/manual/definitions.typ": *

The $LogFunction$ function evaluates $ln x$ using only operations whose results IEEE 754 fixes, so
  that seven independent implementations produce identical bits.

The construction is the classical range reduction.
Every positive double is a significand times a power of two, and the logarithm turns that product
  into a sum:

$ x = m dot 2^k, quad ln x = k ln 2 + ln m $

with $m$ recentered on $[sqrt(2)\/2, sqrt(2))$ so that $m - 1$ stays small at both ends of the
  interval rather than only at one.
Four decisions within that outline determine whether the result is reproducible, and each is
  described below.

*The split is a decomposition, not a rounding*

Separating $x$ into $m$ and $k$ moves no bits: the significand is copied and the exponent is read,
  and both halves multiply back to the input exactly.
That is why the seven implementations may obtain the pair in two different ways without diverging.
Go and Python call the `frexp` their standard library provides, which returns $m$ in $[0.5, 1)$;
  the other five read the exponent field out of the binary64 payload, which yields $m$ in $[1, 2)$.
Each then makes one adjustment to land in the fitted interval, and the two adjustments are
  different only in which end moves: one doubles the low end, the other halves the high one.

Subnormal inputs carry no exponent to read, so they are first scaled by $2^54$, which is exact, and
  the scaling is accounted for in $k$.

*$ln 2$ is split so that the product stays exact*

$ln 2$ is stored as two doubles: a high part $L_"hi"$ carrying 32 significant bits, and a low
  remainder $L_"lo"$ holding what the first one drops.
The technique is due to Cody and Waite (@codywaite1980) and is standard in libraries that evaluate
  elementary functions (@muller2016).
Since $abs(k)$ never exceeds 11 bits, the product $k L_"hi"$ needs at most $32 + 11 = 43$ of the 53
  bits available and is therefore exact.
The check is not a matter of taste: over the whole reachable range of $k$, from $-1076$ to $1024$,
  every one of the 2101 products is representable without rounding.

*The polynomial fits a correction rather than the function*

The argument is transformed once more before fitting.
Writing $f = m - 1$ and

$ s = f \/ (2 + f), quad ln m = ln((1 + s) \/ (1 - s)) $

turns the logarithm into an odd function of a variable no larger than $0.1716$ in magnitude, which
  a short polynomial reproduces far better than $ln(1 + f)$ over the same interval.
What is fitted is not that function but the correction left after its two leading terms:

$ P(z) = (ln((1 + s)\/(1 - s)) - 2 s) \/ s^3, quad z = s^2 $

evaluated as a chain of 7 coefficients by Horner's method.
The leading coefficients come out as $2\/3, 2\/5, 2\/7$, the odd series doubled, which a reader can
  verify by inspection; the tail departs from the series because the fit minimizes the worst error
  rather than matching Taylor.
Over the reduced range the fit itself contributes at most $3.1 dot 10^(-16)$, before any binary64
  rounding enters.

The divisor is $s^3$ and not $2 s^3$, which is why the leading terms are $2\/3, 2\/5, 2\/7$ rather
  than $1\/3, 1\/5, 1\/7$.
The assembly below carries no factor of two to compensate, so fitting the halved quantity produces
  a result wrong in the twelfth digit while every structural test still passes.

*The assembly keeps the two halves apart*

The result is not assembled as $k ln 2 + ln m$.
The two terms differ in magnitude by up to three hundred orders, so adding them directly would
  discard the low bits of the second, and those bits are the accuracy.
Instead, writing $"hfsq" = f^2 \/ 2$ and $r = z P(z)$:

$ ln x = k L_"hi" - (("hfsq" - (s dot ("hfsq" + r) + k L_"lo")) - f) $

Each subtraction is between quantities of comparable size, so no term falls off the end.
When $k = 0$ the two $ln 2$ terms vanish and the shorter form $f - ("hfsq" - s dot ("hfsq" + r))$
  is used, which avoids introducing a zero that would round.
