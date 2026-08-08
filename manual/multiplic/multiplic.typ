#import "/manual/definitions.typ": *

== Multiplic ('Log-Normal') <sec-multiplic>

$ Multiplic(plogmean, plogstddev) $

- $plogmean$: mean of log values (location parameter; $e^plogmean$ equals the geometric mean)
- $plogstddev$: standard deviation of log values (scale parameter; controls multiplicative spread)

#image("/img/distribution-multiplic_light.png")

- *Formation:* the product of many positive variables $X_1 dot X_2 dot ... dot X_n$ with mild conditions (e.g., finite variance of $log X$).
- *Origin:* historically called 'Log-Normal' or 'Galton' distribution after Francis Galton.
- *Rename Motivation:* renamed to $Multiplic$ to reflect its formation mechanism through multiplication.
- *Properties:* logarithm of a $Multiplic$ ('LogNormal') variable follows an $Additive$ ('Normal') distribution.
- *Applications:* stock prices, file sizes, reaction times, income distributions, biological growth rates.
- *Caution:* no perfectly multiplic distributions exist in real data;
    all real-world measurements contain deviations.
  Traditional estimators struggle with the inherent skewness and heavy right tail.

All three functions are assembled from the toolkit's own primitives rather than from the platform's, so the
  seven implementations return identical bits, and the `dist-multiplic` suite compares them as raw binary64
  payloads rather than against a tolerance.

On the support $(0, infinity)$, with $z = (LogFunction(v) - plogmean) \/ plogstddev$:

$ Density(v) = ExpFunction(-z^2 \/ 2) / (v dot plogstddev dot sqrt(2 pi)) $
$ Cumulative(v) = AdditiveCumulative(z) $
$ Quantile(c) = ExpFunction(plogmean + plogstddev dot AdditiveQuantile(c)) $

A multiplicative variable is an additive one read through a logarithm, and the three functions say
  so directly: each is its #link(<sec-additive>)[$Additive$] counterpart composed with
  #link(<sec-log-function>)[$LogFunction$] or #link(<sec-exp-function>)[$ExpFunction$].
The support is the positive reals, so $Density$ and $Cumulative$ are zero at and below zero, and
  $Quantile(0) = 0$.

=== Tests

#include "multiplic-tests.typ"
