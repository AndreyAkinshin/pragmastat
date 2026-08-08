#import "/manual/definitions.typ": *

== Exp <sec-exp>

$ Exp(prate) $

- $prate$: rate parameter ($lambda > 0$, controls decay speed; mean = $1\/prate$)

#image("/img/distribution-exponential_light.png")

- *Formation:* the waiting time between events in a Poisson process.
- *Origin:* naturally arises from memoryless processes where the probability
  of an event occurring is constant over time.
- *Properties:* memoryless (past events do not affect future probabilities).
- *Applications:* time between failures, waiting times in queues, radioactive decay, customer service times.
- *Characteristics:* always positive, right-skewed with a light (exponential) tail.
- *Caution:* extreme skewness makes traditional location estimators like $Mean$ unreliable;
    robust estimators provide more stable results.

All three functions are assembled from the toolkit's own primitives rather than from the platform's, so the
  seven implementations return identical bits, and the `dist-exp` suite compares them as raw binary64
  payloads rather than against a tolerance.

On the support $[0, infinity)$:

$ Density(v) = prate dot ExpFunction(-prate dot v) $
$ Cumulative(v) = 1 - ExpFunction(-prate dot v) $
$ Quantile(c) = (- LogFunction(1 - c)) / prate $

The density and the cumulative reach #link(<sec-exp-function>)[$ExpFunction$], the quantile reaches
  #link(<sec-log-function>)[$LogFunction$], and nothing on either path reaches the platform.
Below zero the family places no mass, so $Density$ and $Cumulative$ are zero there and
  $Quantile(0) = 0$.

=== Tests

#include "exp-tests.typ"
