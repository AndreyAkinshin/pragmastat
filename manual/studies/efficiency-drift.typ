#import "/manual/definitions.typ": *

== From Statistical Efficiency to Drift

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
Drift measures estimator precision using $Spread$ instead of variance,
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

Here $RelSpread$ normalizes by the estimator's typical value for fair comparison.

Drift offers four key advantages:

- For estimators with $sqrt(n)$ convergence rates, drift remains finite and comparable across distributions; for heavier tails, drift may diverge, flagging estimator instability.
- It provides absolute precision measures rather than only pairwise comparisons.
- The robust $Spread$ foundation resists outlier distortion that corrupts variance-based calculations.
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
