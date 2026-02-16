#import "/manual/definitions.typ": *

== Multiplic

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
