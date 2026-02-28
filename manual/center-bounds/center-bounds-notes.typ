#import "/manual/definitions.typ": *

#let MedianBounds = math.op("MedianBounds")

==== On Bootstrap for Center Bounds

A natural question arises: can bootstrap resampling improve $CenterBounds$ coverage
  for asymmetric distributions where the weak symmetry assumption fails?

The idea is appealing.
The signed-rank approach computes bounds from order statistics of Walsh averages
  using a margin derived from the Wilcoxon distribution,
  which assumes symmetric deviations from the center.
Bootstrap makes no symmetry assumption:
  resample the data with replacement,
  compute #link(<sec-center>)[$Center$] on each resample,
  and take quantiles of the bootstrap distribution as bounds.
This should yield valid bounds regardless of distributional shape.

This manual deliberately does not provide a bootstrap-based alternative to $CenterBounds$.
The reasons are both computational and statistical.

*Computational cost*

$CenterBounds$ computes bounds in $O(n log n)$ time: a single pass through the Walsh averages
  guided by the signed-rank margin.
No resampling, no iteration.

A bootstrap version requires $B$ resamples (typically $B = 10000$ for stable tail quantiles),
  each computing #link(<sec-center>)[$Center$] on the resample.
#link(<sec-center>)[$Center$] itself costs $O(n log n)$ via the fast selection algorithm on the implicit pairwise matrix.
The total cost becomes $O(B dot n log n)$ per call — roughly $10000 times$ slower than the signed-rank approach.

For $n = 5$, each call to #link(<sec-center>)[$Center$] operates on $15$ Walsh averages.
The bootstrap recomputes this $10000$ times.
The computation is not deep — it is merely wasteful.
For $n = 100$, there are $5050$ Walsh averages per resample, and $10000$ resamples
  produce $5 times 10^7$ selection operations per bounds call.
In a simulation study that evaluates bounds across many samples,
  this cost becomes prohibitive.

*Statistical quality*

Bootstrap bounds are _nominal_, not exact.
The percentile method has well-documented undercoverage for small samples:
  requesting high confidence (for example, $misrate = 1e-3$) often yields materially higher actual misrate for $n < 30$.
This is inherent to the bootstrap percentile method — the quantile estimates from $B$ resamples
  are biased toward the sample and underrepresent tail behavior.
Refined methods (BCa, bootstrap-$t$) partially address this
  but add complexity and still provide only asymptotic guarantees.

Meanwhile, $CenterBounds$ provides exact distribution-free coverage under symmetry.
For $n = 5$ requesting $misrate = 0.1$, the signed-rank method delivers exactly $10%$ misrate.
A bootstrap method, requesting the same $10%$, typically delivers $12$–$15%$ misrate.
The exact method is simultaneously faster and more accurate.

*Behavior under asymmetry*

Under asymmetric distributions, the signed-rank margin is no longer calibrated:
  the Wilcoxon distribution assumes symmetric deviations, and asymmetry shifts the actual
  distribution of comparison counts.

However, the coverage degradation is gradual, not catastrophic.
Mild asymmetry produces mild coverage drift.
The bounds remain meaningful — they still bracket the pseudomedian using
  order statistics of the Walsh averages — but the actual misrate differs from the requested value.

This is the same situation as bootstrap, which also provides only approximate coverage.
The practical difference is that the signed-rank approach achieves this approximate coverage
  in $O(n log n)$ time, while bootstrap achieves comparable approximate coverage
  in $O(B dot n log n)$ time.

*Why not both?*

One might argue for providing both methods: the signed-rank approach as default,
  and a bootstrap variant for cases where symmetry is severely violated.

This creates a misleading choice.
If the bootstrap method offered substantially better coverage under asymmetry,
  the complexity would be justified.
But for the distributions practitioners encounter
  ($Multiplic$, $Exp$, and other moderate asymmetries),
  the coverage difference between the two approaches is small relative to the $10000 times$ cost difference.
For extreme asymmetries where the signed-rank coverage genuinely breaks down,
  the sign test provides an alternative foundation for median bounds
  (see #link(<sec-median-bounds-efficiency>)[On Misrate Efficiency of MedianBounds]),
  but its $O(n^(-1\/2))$ efficiency convergence makes it impractical for moderate sample sizes.

The toolkit therefore provides $CenterBounds$ as the single bounds estimator.
The weak symmetry assumption means the method performs well under approximate symmetry
  and degrades gracefully under moderate asymmetry.
There is no useful middle ground that justifies a $10000 times$ computational penalty
  for marginally different approximate coverage.

==== On Misrate Efficiency of MedianBounds <sec-median-bounds-efficiency>

This note analyzes $MedianBounds$, a bounds estimator for the population median
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
This note derives the resulting efficiency loss and its convergence rate.

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
  columns: 5,
  align: (center,) * 5,
  [$alpha$], [$0.1$], [$0.01$], [$0.005$], [$0.001$],
  [$z_alpha$], [$-1.64$], [$-2.58$], [$-2.81$], [$-3.29$],
  [$phi(z_alpha)$], [$0.103$], [$0.015$], [$0.008$], [$0.002$],
  [$C(alpha)$], [$2.06$], [$2.94$], [$3.17$], [$3.45$],
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
