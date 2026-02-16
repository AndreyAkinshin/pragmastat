#import "/manual/definitions.typ": *

This section compares the toolkit's robust average estimator against traditional methods
  to demonstrate its advantages across diverse conditions.

*Average Estimators*

*Mean* (arithmetic average):
$ Mean(vx) = 1/n sum_(i=1)^n x_i $

*Median*:
$ Median(vx) = cases(
  x_(((n+1)\/2)) & "if" n "is odd",
  (x_((n\/2)) + x_((n\/2+1))) / 2 & "if" n "is even"
) $

*Center* (Hodges-Lehmann estimator):
$ Center(vx) = attach(Median, b: 1 <= i <= j <= n) ((x_i + x_j) / 2) $

==== Breakdown

Heavy-tailed distributions naturally produce extreme outliers that completely distort traditional estimators.
A single extreme measurement from the $Power$ distribution can make the sample mean arbitrarily large.
Real-world data can also contain corrupted measurements from instrument failures, recording errors, or transmission problems.
Both natural extremes and data corruption create the same challenge:
  extracting reliable information when some measurements are too influential.

The breakdown point (@huber2009) is the fraction of a sample that can be replaced by arbitrarily large values
  without making an estimator arbitrarily large.
The theoretical maximum is $50%$; no estimator can guarantee reliable results
  when more than half the measurements are extreme or corrupted.
In such cases, summary estimators are not applicable, and a more sophisticated approach is needed.

A $50%$ breakdown point is rarely needed in practice, as more conservative values also cover practical needs.
Additionally, a high breakdown point corresponds to low precision
  (information is lost by neglecting part of the data).
The optimal practical breakdown point should be between
  $0%$ (no robustness) and $50%$ (low precision).

The $Center$ estimator achieves a $29%$ breakdown point,
  providing substantial protection against realistic contamination levels
  while maintaining good precision.

*Asymptotic breakdown points* for average estimators:

#table(
  columns: 3,
  [*$Mean$*], [*$Median$*], [*$Center$*],
  [0%], [50%], [29%],
)

==== Drift

Drift measures estimator precision by quantifying how much estimates scatter across repeated samples.
It is based on the $Spread$ of estimates and therefore has a breakdown point of approximately $29%$.

Drift is useful for comparing the precision of several estimators.
To simplify the comparison, one of the estimators can be chosen as a baseline.
A table of squared drift values, normalized by the baseline, shows the required sample size adjustment factor
  for switching from the baseline to another estimator.
For example, if $Center$ is the baseline and the rescaled drift square of $Median$ is $1.5$,
  this means that $Median$ requires $1.5$ times more data than $Center$ to achieve the same precision.
See #link(<sec-efficiency-to-drift>)[From Statistical Efficiency to Drift] for details.

*Squared Asymptotic Drift of Average Estimators* (values are approximated):

#table(
  columns: 4,
  [], [*$Mean$*], [*$Median$*], [*$Center$*],
  [$Additive$], [1.0], [1.571], [1.047],
  [$Multiplic$], [3.95], [1.40], [1.7],
  [$Exp$], [1.88], [1.88], [1.69],
  [$Power$], [$infinity$], [0.9], [2.1],
  [$Uniform$], [0.88], [2.60], [0.94],
)

Rescaled to $Center$ (sample size adjustment factors):

#table(
  columns: 4,
  [], [*$Mean$*], [*$Median$*], [*$Center$*],
  [$Additive$], [0.96], [1.50], [1.0],
  [$Multiplic$], [2.32], [0.82], [1.0],
  [$Exp$], [1.11], [1.11], [1.0],
  [$Power$], [$infinity$], [0.43], [1.0],
  [$Uniform$], [0.936], [2.77], [1.0],
)

#image("/img/avg-drift-additive_light.png")

#image("/img/avg-drift-multiplic_light.png")

#image("/img/avg-drift-exp_light.png")

#image("/img/avg-drift-power_light.png")

#image("/img/avg-drift-uniform_light.png")
