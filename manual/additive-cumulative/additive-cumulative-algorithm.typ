#import "/manual/definitions.typ": *

The $AdditiveCumulative$ function evaluates the distribution function of the standard $Additive$
  ('Normal') distribution through the error function, in the substitution $t = abs(z) \/ sqrt(2)$:

$ AdditiveCumulative(z) = cases(
  (1 + "erf"(t)) \/ 2 & "for " z >= 0,
  "erfc"(t) \/ 2 & "for " z < 0,
) $

Two regimes follow, because no single polynomial covers the whole range.
Near the center the function is smooth and bounded away from zero, so a polynomial reproduces it
  directly.
In the tail it decays like $e^(-t^2)$, and no polynomial of any degree follows exponential decay,
  so the decay is factored out and only the slowly varying remainder is approximated.

*Central regime* ($t < 1 \/ 2$)

The quantity approximated is $"erf"(t) \/ t$ rather than $"erf"(t)$ itself.
The error function is odd, which makes that ratio even, which in turn makes it a function of $t^2$
  alone:

$ "erf"(t) = t dot P_A (s), quad s = t^2 in [0, 1 \/ 4] $

Working in $s$ halves the degree the fit requires, since every odd power vanishes by construction
  rather than through the fit discovering that its coefficient is zero.
$P_A$ carries 12 coefficients and is evaluated by Horner's method in $u = 8 s - 1$, which maps
  $[0, 1\/4]$ onto $[-1, 1]$.

*Tail regime* ($1 \/ 2 <= t <= 4.3$)

The quantity approximated is $"erfc"(t) dot e^(t^2)$, which behaves like $1 \/ (t sqrt(pi))$.
That product is smooth and slowly varying, and therefore suited to a polynomial where $"erfc"$
  itself is not:

$ "erfc"(t) = e^(-t^2) dot P_B (u), quad u = (2 t - 4.8) \/ 3.8 $

$P_B$ carries 27 coefficients on the same normalized variable.
The exponential is #link(<sec-exp-function>)[$ExpFunction$] rather than the platform's, for the
  reason given in that section.

*Beyond the tail* ($t > 4.3$)

The complementary error function is taken as zero, which makes the distribution function return
  exactly $0$ or exactly $1$ past $abs(z) = 6.08$.
The truncated quantity there is $"erfc"(4.3) \/ 2 approx 6 dot 10^(-10)$, so the result is not
  merely imprecise beyond that point: it is wrong by a relative error of $1$, as any approximation
  that stops is.

The cutoff is placed where no estimator reaches rather than where the truncated value becomes
  negligible, and the accuracy claim below is scoped to $abs(z) <= 6$ for the same reason.
Both margins evaluate this function on a standardized statistic whose magnitude stays far inside
  that range for every sample size the Edgeworth branches serve.

*Origin of the coefficients*

Both chains are Chebyshev interpolations (@trefethen2019) computed at 80 decimal digits against a
  reference built from two convergent expansions: the Taylor series for $"erf"$ below $2.5$, and a
  continued fraction for $"erfc"$ above it.
Splitting the range at a transformed $"erfc"$ follows the strategy Cody established for rational
  approximations to the same function (@cody1969); the approximations here are polynomial rather
  than rational.
The two agree to 36 digits where they overlap, which establishes the reference through internal
  consistency rather than through an appeal to an outside implementation.

A script that ships with the toolkit produces the coefficients, so they can be recomputed rather
  than only compared against the source they were transcribed from.

*Accuracy*

The worst relative error is $9.1 dot 10^(-15)$ over $abs(z) <= 6$, evaluated in binary64 against
  the same reference.
The degree of $P_B$ was selected where additional terms stop being visible in binary64: degree 13
  yields $5.9 dot 10^(-8)$, degree 17 yields $1.5 dot 10^(-10)$, degree 21 yields
  $2.7 dot 10^(-13)$, and degree 25 yields $4.0 dot 10^(-16)$.
