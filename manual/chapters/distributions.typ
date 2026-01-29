#import "/manual/definitions.typ": *

= Distributions

Distributions are parametrized random generators with well-defined statistical properties.
Each distribution describes a family of random variables characterized by specific parameters.

#v(0.5em)
*Notation*

- $X tilde Additive(0, 1)$ — $X$ is distributed as standard normal
- $op("Estimator")(vx)$ — estimate computed from sample
- $op("Estimator")[X]$ — true value (asymptotic limit)
- $n -> infinity$ — asymptotic case (large sample approximation)

#include "../distributions/additive.typ"

#include "../distributions/multiplic.typ"

#include "../distributions/exp.typ"

#include "../distributions/power.typ"

#include "../distributions/uniform.typ"
