#import "/manual/definitions.typ": *

== Power <sec-power>

$ Power(pmin, pshape) $

- $pmin$: minimum value (lower bound, $pmin > 0$)
- $pshape$: shape parameter ($alpha > 0$, controls tail heaviness; smaller values = heavier tails)

#image("/img/distribution-power_light.png")

- *Formation:* follows a power-law relationship where large values are rare but possible.
- *Origin:* historically called 'Pareto' distribution after Vilfredo Pareto's work on wealth distribution.
- *Rename Motivation:* renamed to $Power$ to reflect its connection with power-law.
- *Properties:* exhibits scale invariance and extremely heavy tails.
- *Applications:* wealth distribution, city population sizes, word frequencies, earthquake magnitudes, website traffic.
- *Characteristics:* infinite variance for many parameter values; extreme outliers are common.
- *Caution:* traditional variance-based estimators completely fail;
    robust estimators are essential for reliable analysis.

All three functions are assembled from the toolkit's own primitives rather than from the platform's, so the
  seven implementations return identical bits, and the `dist-power` suite compares them as raw binary64
  payloads rather than against a tolerance.

On the support $[pmin, infinity)$:

$ Density(v) = (pshape dot pmin^pshape) / v^(pshape + 1) $
$ Cumulative(v) = 1 - (pmin \/ v)^pshape $
$ Quantile(c) = pmin \/ (1 - c)^(1 \/ pshape) $

Every one of the three raises a number to a real power, and the toolkit has no primitive for that,
  deliberately: a general power would be a fourth transcendental to fit, verify and keep identical
  across seven implementations, reached from nowhere else.
It is expressed through the two primitives that already exist instead, as
  $x^a = ExpFunction(a dot LogFunction(x))$:

$ Density(v) = pshape / v dot ExpFunction(pshape dot (LogFunction(pmin) - LogFunction(v))) $
$ Cumulative(v) = 1 - ExpFunction(pshape dot (LogFunction(pmin) - LogFunction(v))) $
$ Quantile(c) = pmin dot ExpFunction((- LogFunction(1 - c)) / pshape) $

This family is the reason #link(<sec-log-function>)[$LogFunction$] had to exist before the
  distributions could be described at all: without a reproducible logarithm there is no
  reproducible real power.
Below $pmin$ the family places no mass, so $Cumulative$ is zero there and $Quantile(0) = pmin$,
  while $Density(pmin) = pshape \/ pmin$ because the support is closed at that end.

=== Tests

#include "power-tests.typ"
