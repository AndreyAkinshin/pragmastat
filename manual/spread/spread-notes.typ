#import "/manual/definitions.typ": *

This section compares the toolkit's robust dispersion estimator against traditional methods
  to demonstrate its advantages across diverse conditions.

*Dispersion Estimators*

*Standard Deviation*:
$ StdDev(vx) = sqrt(1/(n-1) sum_(i=1)^n (x_i - Mean(vx))^2) $

*Median Absolute Deviation* (around the median):
$ MAD(vx) = Median(abs(x_i - Median(vx))) $

*Spread* (Shamos scale estimator):
$ Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j) $

==== Breakdown

Heavy-tailed distributions naturally produce extreme outliers that completely distort traditional estimators.
A single extreme measurement from the $Power$ distribution can make the standard deviation arbitrarily large.
Real-world data can also contain corrupted measurements from instrument failures, recording errors, or transmission problems.
Both natural extremes and data corruption create the same challenge:
  extracting reliable information when some measurements are too influential.

The breakdown point (@huber2009) is the fraction of a sample that can be replaced by arbitrarily large values
  without making an estimator arbitrarily large.
The theoretical maximum is $50%$; no estimator can guarantee reliable results
  when more than half the measurements are extreme or corrupted.

The $Spread$ estimator achieves a $29%$ breakdown point,
  providing substantial protection against realistic contamination levels
  while maintaining good precision.

*Asymptotic breakdown points* for dispersion estimators:

#table(
  columns: 3,
  [*$StdDev$*], [*$MAD$*], [*$Spread$*],
  [0%], [50%], [29%],
)

==== Drift

Drift measures estimator precision by quantifying how much estimates scatter across repeated samples.
It is based on the $Spread$ of estimates and therefore has a breakdown point of approximately $29%$.

Drift is useful for comparing the precision of several estimators.
To simplify the comparison, one of the estimators can be chosen as a baseline.
A table of squared drift values, normalized by the baseline, shows the required sample size adjustment factor
  for switching from the baseline to another estimator.
For example, if $Spread$ is the baseline and the rescaled drift square of $MAD$ is $1.25$,
  this means that $MAD$ requires $1.25$ times more data than $Spread$ to achieve the same precision.
See #link(<sec-efficiency-to-drift>)[From Statistical Efficiency to Drift] for details.

*Squared Asymptotic Drift of Dispersion Estimators* (values are approximated):

#table(
  columns: 4,
  [], [*$StdDev$*], [*$MAD$*], [*$Spread$*],
  [$Additive$], [0.45], [1.22], [0.52],
  [$Multiplic$], [$infinity$], [2.26], [1.81],
  [$Exp$], [1.69], [1.92], [1.26],
  [$Power$], [$infinity$], [3.5], [4.4],
  [$Uniform$], [0.18], [0.90], [0.43],
)

Rescaled to $Spread$ (sample size adjustment factors):

#table(
  columns: 4,
  [], [*$StdDev$*], [*$MAD$*], [*$Spread$*],
  [$Additive$], [0.87], [2.35], [1.0],
  [$Multiplic$], [$infinity$], [1.25], [1.0],
  [$Exp$], [1.34], [1.52], [1.0],
  [$Power$], [$infinity$], [0.80], [1.0],
  [$Uniform$], [0.42], [2.09], [1.0],
)

#image("/img/disp-drift-additive_light.png")

#image("/img/disp-drift-multiplic_light.png")

#image("/img/disp-drift-exp_light.png")

#image("/img/disp-drift-power_light.png")

#image("/img/disp-drift-uniform_light.png")
