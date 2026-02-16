#import "/manual/definitions.typ": *

== From Statistical Efficiency to Drift <sec-efficiency-to-drift>

Statistical efficiency measures estimator precision (@serfling2009).
When multiple estimators target the same quantity, efficiency determines which provides more reliable results.

Efficiency measures how tightly estimates cluster around the true value across repeated samples.
For an estimator $T$ applied to samples from distribution $X$,
  absolute efficiency is defined relative to the optimal estimator $T^*$:

$ "Efficiency"(T, X) = "Var"[T^*(X_1, ..., X_n)] / "Var"[T(X_1, ..., X_n)] $

Relative efficiency compares two estimators by taking the ratio of their variances:

$ "RelativeEfficiency"(T_1, T_2, X) = "Var"[T_2(X_1, ..., X_n)] / "Var"[T_1(X_1, ..., X_n)] $

Under $Additive$ (Normal) distributions, this approach works well.
The sample mean achieves optimal efficiency, while the median operates at roughly 64% efficiency.

However, this variance-based definition creates four critical limitations:

- Absolute efficiency requires knowing the optimal estimator, which is difficult to determine.
  For many distributions, deriving the minimum-variance unbiased estimator requires complex mathematical analysis.
  Without this reference point, absolute efficiency cannot be computed.
- Relative efficiency only compares estimator pairs, preventing systematic evaluation.
  This limits understanding of how multiple estimators perform relative to each other.
  Practitioners cannot rank estimators comprehensively or evaluate individual performance in isolation.
- The approach depends on variance calculations that break down when variance becomes infinite
  or when distributions have heavy tails.
  Many real-world distributions, such as those with power-law tails, exhibit infinite variance.
  When the variance is undefined, efficiency comparisons become impossible.
- Variance is not robust to outliers, which can corrupt efficiency calculations.
  A single extreme observation can greatly inflate variance estimates.
  This sensitivity can make efficient estimators look inefficient and vice versa.

The $Drift$ concept provides a robust alternative.
Drift measures estimator precision using #link(<sec-spread>)[$Spread$] instead of variance,
  providing reliable comparisons across a wide range of distributions.

For an average estimator $T$, random variable $X$, and sample size $n$:

$ AvgDrift(T, X, n) = (sqrt(n) dot Spread[T(X_1, ..., X_n)]) / Spread[X] $

This formula measures estimator variability compared to data variability.
$Spread[T(X_1, ..., X_n)]$ captures the median absolute difference between estimates across repeated samples.
Multiplying by $sqrt(n)$ removes sample size dependency, making drift values comparable across different sample sizes.
Dividing by $Spread[X]$ creates a scale-free measure that provides consistent drift values
  across different distribution parameters and measurement units.

Dispersion estimators use a parallel formulation:

$ DispDrift(T, X, n) = sqrt(n) dot RelSpread[T(X_1, ..., X_n)] $

Here $RelSpread$ (where $RelSpread[Y] = Spread[Y] / abs(Center[Y])$) normalizes by the estimator's typical value for fair comparison.

Drift offers four key advantages:

- For estimators with $sqrt(n)$ convergence rates, drift remains finite and comparable across distributions; for heavier tails, drift may diverge, flagging estimator instability.
- It provides absolute precision measures rather than only pairwise comparisons.
- The robust #link(<sec-spread>)[$Spread$] foundation resists outlier distortion that corrupts variance-based calculations.
- The $sqrt(n)$ normalization makes drift values comparable across different sample sizes,
  enabling direct comparison of estimator performance regardless of sample size.

Under $Additive$ (Normal) conditions, drift matches traditional efficiency.
The sample mean achieves drift near 1.0; the median achieves drift around 1.25.
This consistency validates drift as a proper generalization of efficiency
  that extends to realistic data conditions where traditional efficiency fails.

When switching from one estimator to another while maintaining the same precision,
  the required sample size adjustment follows:

$ n_"new" = n_"original" dot Drift^2(T_2, X) / Drift^2(T_1, X) $

This applies when estimator $T_1$ has lower drift than $T_2$.

The ratio of squared drifts determines the data requirement change.
If $T_2$ has drift 1.5 times higher than $T_1$, then $T_2$ requires $(1.5)^2 = 2.25$ times more data
  to match $T_1$'s precision.
Conversely, switching to a more precise estimator allows smaller sample sizes.

For asymptotic analysis, $Drift(T, X)$ denotes the limiting value as $n -> infinity$.
With a baseline estimator, rescaled drift values enable direct comparisons:

$ Drift_"baseline"(T, X) = Drift(T, X) / Drift(T_"baseline", X) $

The standard drift definition assumes $sqrt(n)$ convergence rates typical under $Additive$ (Normal) conditions.
For broader applicability, drift generalizes to:

$ AvgDrift(T, X, n) = (n^"instability" dot Spread[T(X_1, ..., X_n)]) / Spread[X] $

$ DispDrift(T, X, n) = n^"instability" dot RelSpread[T(X_1, ..., X_n)] $

The instability parameter adapts to estimator convergence rates.
The toolkit uses $"instability" = 1\/2$ throughout because this choice provides natural intuition
  and mental representation for the $Additive$ (Normal) distribution.
Rather than introduce additional complexity through variable instability parameters,
  the fixed $sqrt(n)$ scaling offers practical convenience while maintaining theoretical rigor
  for the distribution classes most common in applications.

== From Confidence Level to Misrate

Traditional statistics expresses uncertainty through confidence levels:
  "95% confidence interval", "99% confidence", "99.9% confidence".
This convention emerged from early statistical practice
  when tables printed confidence intervals for common levels like 90%, 95%, and 99%.

The confidence level approach creates practical problems:

- *Cognitive difficulty with high confidence*.
  Distinguishing between 99.999% and 99.9999% confidence requires mental effort.
  The difference matters — one represents a 1-in-100,000 error rate, the other 1-in-1,000,000 —
  but the representation obscures this distinction.
- *Asymmetric scale*.
  The confidence level scale compresses near 100%, where most practical values cluster.
  Moving from 90% to 95% represents a 2× change in error rate,
  while moving from 99% to 99.9% represents a 10× change, despite similar visual spacing.
- *Indirect interpretation*.
  Practitioners care about error rates, not success rates.
  "What's the chance I'm wrong?" matters more than "What's the chance I'm right?"
  Confidence level forces mental subtraction to answer the natural question.
- *Unclear defaults*.
  Traditional practice offers no clear default confidence level.
  Different fields use different conventions (95%, 99%, 99.9%),
  creating inconsistency and requiring arbitrary choices.

The $misrate$ parameter provides a more natural representation.
Misrate expresses the probability that computed bounds fail to contain the true value:

$ misrate = 1 - "confidence level" $

This simple inversion provides several advantages:

- *Direct interpretation*.
  $misrate = 0.01$ means "1% chance of error" or "wrong 1 time in 100".
  $misrate = 10^(-6)$ means "wrong 1 time in a million".
  No mental arithmetic required.
- *Linear scale for practical values*.
  $misrate = 0.1$ (10%), $misrate = 0.01$ (1%), $misrate = 0.001$ (0.1%)
  form a natural sequence.
  Scientific notation handles extreme values cleanly: $10^(-3)$, $10^(-6)$, $10^(-9)$.
- *Clear comparisons*.
  $10^(-5)$ versus $10^(-6)$ immediately shows a 10× difference in error tolerance.
  99.999% versus 99.9999% confidence obscures this same relationship.
- *Pragmatic default*.
  The toolkit recommends $misrate = 10^(-3)$ (one-in-a-thousand error rate)
  as a reasonable default for everyday analysis.
  For critical decisions where errors are costly, use $misrate = 10^(-6)$ (one-in-a-million).

The terminology shift from "confidence level" to "misrate"
  parallels other clarifying renames in this toolkit.
Just as $Additive$ better describes the distribution's formation than 'Normal',
  and #link(<sec-center>)[$Center$] better describes the estimator's purpose than 'Hodges-Lehmann',
  $misrate$ better describes the quantity practitioners actually reason about:
  the probability of error.

Traditional confidence intervals become "bounds" in this framework,
  eliminating statistical jargon in favor of descriptive terminology.
#link(<sec-shift-bounds>)[$ShiftBounds(vx, vy, misrate)$] clearly indicates:
  it provides bounds on the shift, with a specified error rate.
No background in classical statistics required to understand the concept.

== Invariance

Invariance properties determine how estimators respond to data transformations.
These properties are crucial for analysis design and interpretation:

- *Location-invariant* estimators are invariant to additive shifts: $T(vx+k)=T(vx)$
- *Scale-invariant* estimators are invariant to positive rescaling: $T(k dot vx)=T(vx)$ for $k>0$
- *Equivariant* estimators change predictably with transformations, maintaining relative relationships

Choosing estimators with appropriate invariance properties ensures that results remain
  meaningful across different measurement scales, units, and data transformations.
For example, when comparing datasets collected with different instruments or protocols,
  location-invariant estimators eliminate the need for data centering,
  while scale-invariant estimators eliminate the need for normalization.

*Location-invariance*: An estimator $T$ is location-invariant if adding a constant to the measurements leaves the result unchanged:

$ T(vx + k) = T(vx) $

$ T(vx + k, vy + k) = T(vx, vy) $

*Location-equivariance*: An estimator $T$ is location-equivariant if it shifts with the data:

$ T(vx + k) = T(vx) + k $

$ T(vx + k_1, vy + k_2) = T(vx, vy) + f(k_1, k_2) $

*Scale-invariance*: An estimator $T$ is scale-invariant if multiplying by a positive constant leaves the result unchanged:

$ T(k dot vx) = T(vx) quad "for" k > 0 $

$ T(k dot vx, k dot vy) = T(vx, vy) quad "for" k > 0 $

*Scale-equivariance*: An estimator $T$ is scale-equivariant if it scales proportionally with the data:

$ T(k dot vx) = k dot T(vx) "or" abs(k) dot T(vx) quad "for" k != 0 $

$ T(k dot vx, k dot vy) = k dot T(vx, vy) "or" abs(k) dot T(vx, vy) quad "for" k != 0 $

#table(
  columns: 3,
  [], [*Location*], [*Scale*],
  [#link(<sec-center>)[Center]], [Equivariant], [Equivariant],
  [#link(<sec-spread>)[Spread]], [Invariant], [Equivariant],
  [#link(<sec-shift>)[Shift]], [Invariant], [Equivariant],
  [#link(<sec-ratio>)[Ratio]], [–], [Invariant],
  [#link(<sec-disparity>)[Disparity]], [Invariant], [Invariant],
)
