#import "/manual/definitions.typ": *

== Uniform

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
