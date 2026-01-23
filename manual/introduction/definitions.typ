#import "/manual/definitions.typ": *

== Definitions

- $X$, $Y$: random variables, can be treated as generators of random real measurements
  - $X tilde "Distribution"$ defines a distribution from which this variable comes
- $x_i, y_j$: specific individual measurements
- $vx = (x_1, x_2, ..., x_n)$, $vy = (y_1, y_2, ..., y_m)$: samples of measurements of a given size
  - Samples are non-empty: $n, m >= 1$
- $x_((1)), x_((2)), ..., x_((n))$: sorted measurements of the sample ('order statistics')
- Asymptotic case: the sample size goes to infinity $n, m -> infinity$
  - Can typically be treated as an approximation for large samples
- $op("Estimator")(vx)$: a function that estimates the property of a distribution from given measurements
  - $op("Estimator")[X]$ shows the true property value of the distribution (asymptotic value)
- $Median$: an estimator that finds the value splitting the distribution into two equal parts

$ Median(vx) = cases(
  x_(((n+1)\/2)) & "if" n "is odd",
  (x_((n\/2)) + x_((n\/2+1))) / 2 & "if" n "is even"
) $
