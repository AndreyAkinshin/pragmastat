#import "/manual/definitions.typ": *

= Distributions

Distributions are parametrized random generators with well-defined statistical properties.
Each distribution describes a family of random variables characterized by specific parameters.

Each family below answers three questions about itself: $Density$ says how tightly probability is
  packed near a value, $Cumulative$ says how much of it lies at or below that value, and $Quantile$
  inverts $Cumulative$.
Traditional statistics calls the first two the probability density function and the cumulative
  distribution function, and abbreviates them PDF and CDF.
Those names appear here once, so that a reader arriving from a textbook can map the vocabulary,
  and nowhere else: the toolkit spells out what a function does rather than abbreviating it.

#v(0.5em)
*Notation*

- $X tilde Additive(0, 1)$ — $X$ is distributed as standard normal
- $op("Estimator")(vx)$ — estimate computed from sample
- $op("Estimator")[X]$ — true value (asymptotic limit)
- $n -> infinity$ — asymptotic case (large sample approximation)

#include "../additive/additive.typ"

#include "../multiplic/multiplic.typ"

#include "../exp/exp.typ"

#include "../power/power.typ"

#include "../uniform/uniform.typ"
