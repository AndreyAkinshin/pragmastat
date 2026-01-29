#import "/manual/definitions.typ": *

== Breakdown

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

The $Center$ and $Spread$ estimators achieve $29%$ breakdown points,
  providing substantial protection against realistic contamination levels
  while maintaining good precision.
Below is a comparison with traditional estimators.

*Asymptotic breakdown points* for average estimators:

#table(
  columns: 3,
  [*$Mean$*], [*$Median$*], [*$Center$*],
  [0%], [50%], [29%],
)

*Asymptotic breakdown points* for dispersion estimators:

#table(
  columns: 3,
  [*$StdDev$*], [*$MAD$*], [*$Spread$*],
  [0%], [50%], [29%],
)
