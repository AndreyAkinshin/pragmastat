#import "/manual/definitions.typ": *

== Exp

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
