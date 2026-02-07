#import "/manual/definitions.typ": *

#let MedianBounds = math.op("MedianBounds")

== On Misrate Efficiency of MedianBounds

This study analyzes $MedianBounds$, a bounds estimator for the population median
  based on the sign test,
  and explains why pragmastat omits it in favor of $CenterBounds$.

*Definition*

$ MedianBounds(vx, misrate) = [x_((k)), x_((n-k+1))] $

where $k$ is the largest integer satisfying $2 dot Pr(B <= k-1) <= misrate$
  and $B tilde "Binomial"(n, 0.5)$.
The interval brackets the population median using order statistics,
  with $misrate$ controlling the probability that the true median falls outside the bounds.

$MedianBounds$ requires no symmetry assumption — only weak continuity —
  making it applicable to arbitrarily skewed distributions.
This is its principal advantage over $CenterBounds$, which assumes weak symmetry.

*Sign test foundation*

The method is equivalent to inverting the sign test.
Under weak continuity, each observation independently falls above or below the true median
  with probability $1\/2$.
The number of observations below the median follows $"Binomial"(n, 0.5)$,
  and the order statistics $x_((k))$ and $x_((n-k+1))$ form a confidence interval
  whose coverage is determined exactly by the binomial CDF.

Because the binomial CDF is a step function,
  the achievable misrate values form a discrete set.
The algorithm rounds down to the nearest achievable level,
  inevitably wasting part of the requested misrate budget.
This study derives the resulting efficiency loss and its convergence rate.

*Achievable misrate levels*

The achievable misrate values for sample size $n$ are:

$ m_k = 2 dot Pr(B <= k), quad k = 0, 1, 2, dots $

The algorithm selects the largest $k$ satisfying $m_k <= misrate$.
The _efficiency_ $eta = m_k \/ misrate$ measures how much of the requested budget is used.
Efficiency $eta = 1$ means the bounds are as tight as the misrate allows;
  $eta = 0.5$ means half the budget is wasted, producing unnecessarily wide bounds.

*Spacing between consecutive levels*

The gap between consecutive achievable misrates is:

$ Delta m_k = m_(k+1) - m_k = 2 dot Pr(B = k+1) = 2 dot binom(n, k+1) \/ 2^n $

For a target misrate $alpha$, the relevant index $k$ satisfies $m_k approx alpha$.
By the normal approximation to the binomial, $B approx cal(N)(n\/2, n\/4)$,
  the binomial CDF near this index changes by approximately:

$ Delta m approx (4 phi(z_alpha)) / sqrt(n) $

where $z_alpha = Phi^(-1)(alpha\/2)$ is the corresponding standard normal quantile
  and $phi$ is the standard normal density.
This spacing governs how coarsely the achievable misrates are distributed near the target.

*Expected efficiency*

The requested misrate $alpha$ falls at a uniformly random position within a gap of width $Delta m$.
On average, the algorithm wastes $Delta m \/ 2$, giving expected efficiency:

$ EE[eta] approx 1 - (Delta m) / (2 alpha) = 1 - (2 phi(z_alpha)) / (alpha sqrt(n)) $

Define the misrate-dependent constant:

$ C(alpha) = (2 phi(Phi^(-1)(alpha\/2))) / alpha $

Then the expected efficiency has the form:

$ EE[eta] approx 1 - C(alpha) / sqrt(n) $

The convergence rate is $O(n^(-1\/2))$: efficiency improves as the square root of sample size.

*Values of $C(alpha)$*

The constant $C(alpha)$ increases for smaller misrates,
  meaning tighter error tolerances require proportionally larger samples for efficient bounds:

#table(
  columns: 6,
  align: (center,) * 6,
  [$alpha$], [$0.1$], [$0.05$], [$0.01$], [$0.005$], [$0.001$],
  [$z_alpha$], [$-1.64$], [$-1.96$], [$-2.58$], [$-2.81$], [$-3.29$],
  [$phi(z_alpha)$], [$0.103$], [$0.058$], [$0.015$], [$0.008$], [$0.002$],
  [$C(alpha)$], [$2.06$], [$2.33$], [$2.94$], [$3.17$], [$3.45$],
)

For $alpha = 0.1$ and $n = 50$: $EE[eta] approx 1 - 2.06 \/ sqrt(50) approx 0.71$.
Achieving $90%$ efficiency on average requires $n > (C(alpha) \/ 0.1)^2$.
For $alpha = 0.1$ this gives $n > 424$; for $alpha = 0.001$ this gives $n > 1190$.

*Comparison with CenterBounds*

$CenterBounds$ uses the signed-rank statistic $W$ with range $[0, n(n+1)\/2]$.
Under the null hypothesis, $W$ has variance $sigma^2 = n(n+1)(2n+1)\/24 approx n^3\/12$.
The CDF spacing at the relevant quantile is:

$ Delta m_W approx (2 sqrt(12) dot phi(z_alpha)) / n^(3\/2) $

The expected efficiency for $CenterBounds$ is therefore:

$ EE[eta_W] approx 1 - (sqrt(12) dot phi(z_alpha)) / (alpha dot n^(3\/2)) $

This converges at rate $O(n^(-3\/2))$ — three polynomial orders faster than $MedianBounds$.
The difference arises because the signed-rank distribution has $n(n+1)\/2$ discrete levels
  compared to the binomial's $n$ levels,
  providing fundamentally finer resolution.

*Why pragmastat omits MedianBounds*

The efficiency loss of $MedianBounds$ is not an implementation artifact.
It reflects a structural limitation of the sign test:
  using only the signs of $(X_i - theta)$ discards magnitude information,
  leaving only $n$ binary observations to determine coverage.
The signed-rank test used by $CenterBounds$ exploits both signs and ranks,
  producing $n(n+1)\/2$ comparison outcomes and correspondingly finer misrate resolution.

For applications requiring tight misrate control on the median,
  large samples ($n > 500$) are needed to ensure efficient use of the misrate budget.
For smaller samples, the bounds remain valid but conservative:
  the actual misrate is guaranteed to not exceed the requested value,
  even though it may be substantially below it.

$CenterBounds$ with its $O(n^(-3\/2))$ convergence achieves near-continuous misrate control
  even for moderate $n$, at the cost of requiring weak symmetry.
For the distributions practitioners typically encounter,
  this tradeoff favors $CenterBounds$ as the single bounds estimator in the toolkit.
When symmetry is severely violated, the coverage drift of $CenterBounds$ is gradual —
  mild asymmetry produces mild drift — making it a robust default
  without the efficiency penalty inherent to the sign test approach.
