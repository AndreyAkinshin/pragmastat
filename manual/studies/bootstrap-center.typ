#import "/manual/definitions.typ": *

=== On Bootstrap for Center Bounds

A natural question arises: can bootstrap resampling improve $CenterBounds$ coverage
  for asymmetric distributions where the weak symmetry assumption fails?

The idea is appealing.
The signed-rank approach computes bounds from order statistics of Walsh averages
  using a margin derived from the Wilcoxon distribution,
  which assumes symmetric deviations from the center.
Bootstrap makes no symmetry assumption:
  resample the data with replacement,
  compute $Center$ on each resample,
  and take quantiles of the bootstrap distribution as bounds.
This should yield valid bounds regardless of distributional shape.

This manual deliberately does not provide a bootstrap-based alternative to $CenterBounds$.
The reasons are both computational and statistical.

*Computational cost*

$CenterBounds$ computes bounds in $O(n log n)$ time: a single pass through the Walsh averages
  guided by the signed-rank margin.
No resampling, no iteration.

A bootstrap version requires $B$ resamples (typically $B = 10000$ for stable tail quantiles),
  each computing $Center$ on the resample.
$Center$ itself costs $O(n log n)$ via the fast selection algorithm on the implicit pairwise matrix.
The total cost becomes $O(B dot n log n)$ per call — roughly $10000 times$ slower than the signed-rank approach.

For $n = 5$, each call to $Center$ operates on $15$ Walsh averages.
The bootstrap recomputes this $10000$ times.
The computation is not deep — it is merely wasteful.
For $n = 100$, there are $5050$ Walsh averages per resample, and $10000$ resamples
  produce $5 times 10^7$ selection operations per bounds call.
In a simulation study that evaluates bounds across many samples,
  this cost becomes prohibitive.

*Statistical quality*

Bootstrap bounds are _nominal_, not exact.
The percentile method has well-documented undercoverage for small samples:
  requesting 95% confidence ($misrate = 0.05$) typically yields 85–92% actual coverage for $n < 30$.
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
  (see the study on misrate efficiency of MedianBounds),
  but its $O(n^(-1\/2))$ efficiency convergence makes it impractical for moderate sample sizes.

The toolkit therefore provides $CenterBounds$ as the single bounds estimator.
The weak symmetry assumption means the method performs well under approximate symmetry
  and degrades gracefully under moderate asymmetry.
There is no useful middle ground that justifies a $10000 times$ computational penalty
  for marginally different approximate coverage.
