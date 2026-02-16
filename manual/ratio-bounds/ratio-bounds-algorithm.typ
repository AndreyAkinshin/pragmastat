#import "/manual/definitions.typ": *

The $RatioBounds$ estimator uses the same log-exp transformation as
  #link(<sec-alg-ratio>)[Ratio], delegating to #link(<sec-alg-shift-bounds>)[ShiftBounds] in log-space:

$ RatioBounds(vx, vy, misrate) = exp(ShiftBounds(log vx, log vy, misrate)) $

The algorithm operates in three steps:

+ *Log-transform* --- Apply $log$ to each element of both samples.
  Positivity is required so that the logarithm is defined.

+ *Delegate to ShiftBounds* --- Compute $[a, b] = ShiftBounds(log vx, log vy, misrate)$.
  This provides distribution-free bounds on the shift in log-space.

+ *Exp-transform* --- Return $[e^a, e^b]$, converting the additive bounds back to multiplicative bounds.

Because $log$ and $exp$ are monotone, the coverage guarantee of $ShiftBounds$ transfers directly:
  the probability that the true ratio falls outside $[e^a, e^b]$ equals the probability
  that the true log-shift falls outside $[a, b]$, which is at most $misrate$.
