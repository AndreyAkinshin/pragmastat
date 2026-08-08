#import "/manual/definitions.typ": *

== Uniform <sec-uniform>

$ Uniform(pmin, pmax) $

- $pmin$: lower bound of the support interval
- $pmax$: upper bound of the support interval ($pmax > pmin$)

#image("/img/distribution-uniform_light.png")

- *Formation:* all values within a bounded interval have equal probability.
- *Origin:* represents complete uncertainty within known bounds.
- *Properties:* rectangular probability density, finite support with hard boundaries.
- *Applications:* random number generation, round-off errors, arrival times within known intervals.
- *Characteristics:* symmetric, bounded, no tail behavior.
- *Note:* traditional estimators work reasonably well due to symmetry and bounded nature.

All three functions are assembled from the toolkit's own primitives rather than from the platform's, so the
  seven implementations return identical bits, and the `dist-uniform` suite compares them as raw binary64
  payloads rather than against a tolerance.

On the support $[pmin, pmax)$:

$ Density(v) = 1 / (pmax - pmin) $
$ Cumulative(v) = (v - pmin) / (pmax - pmin) $
$ Quantile(c) = pmin + c dot (pmax - pmin) $

This is the one family built from arithmetic alone: no primitive of the toolkit appears on any of
  the three paths, and IEEE 754 already fixes every operation involved.
Outside the interval the family places no mass, so $Density$ and $Cumulative$ are zero below
  $pmin$, $Cumulative$ is one at and above $pmax$, and the two ends of the support answer
  $Quantile(0)$ and $Quantile(1)$.

=== Tests

#include "uniform-tests.typ"
