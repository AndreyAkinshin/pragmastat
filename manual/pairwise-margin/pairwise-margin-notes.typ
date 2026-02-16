#import "/manual/definitions.typ": *

The Mann-Whitney $U$ test (also known as the Wilcoxon rank-sum test)
  ranks among the most widely used non-parametric statistical tests,
  testing whether two independent samples come from the same distribution.
Under $Additive$ (Normal) conditions, it achieves nearly the same precision as the Student's $t$-test,
  while maintaining reliability under diverse distributional conditions where the $t$-test fails.

The test operates by comparing all pairs of measurements between the two samples.
Given samples $vx = (x_1, ..., x_n)$ and $vy = (y_1, ..., y_m)$,
  the Mann-Whitney $U$ statistic counts how many pairs satisfy $x_i > y_j$:

$ U = sum_(i=1)^n sum_(j=1)^m bb(1)(x_i > y_j) $

If the samples come from the same distribution, $U$ should be near $n m\/2$
  (roughly half the pairs favor $vx$, half favor $vy$).
Large deviations from $n m\/2$ suggest the distributions differ.

The test answers: "Could this $U$ value arise by chance if the samples were truly equivalent?"
The $p$-value quantifies this probability.
If $p < 0.05$, traditional practice declares the difference "statistically significant".

This approach creates several problems for practitioners:

- *Binary thinking*.
  The test produces a yes/no answer: reject or fail to reject the null hypothesis.
  Practitioners typically want to know the magnitude of difference, not just whether one exists.
- *Arbitrary thresholds*.
  The 0.05 threshold has no universal justification,
  yet it dominates practice and creates a false dichotomy between $p = 0.049$ and $p = 0.051$.
- *Hypothesis-centric framework*.
  The test assumes a null hypothesis of "no difference"
  and evaluates evidence against it.
  Real questions rarely concern exact equality;
  practitioners want to know "how different?" rather than "different or not?"
- *Inverted logic*.
  The natural question is "what shifts are consistent with my data?"
  The test answers "is this specific shift (zero) consistent with my data?"

The toolkit inverts this framework.
Instead of testing whether a hypothesized shift is plausible,
  we compute which shifts are plausible given the data.
This inversion transforms hypothesis testing into bounds estimation.

The mathematical foundation remains the same.
The distribution of pairwise comparisons under random sampling determines
  which order statistics of pairwise differences form reliable bounds.
The Mann-Whitney $U$ statistic measures pairwise comparisons ($x_i > y_j$).
The $Shift$ estimator uses pairwise differences ($x_i - y_j$).
These quantities are mathematically related:
  a pairwise difference $x_i - y_j$ is positive exactly when $x_i > y_j$.
The toolkit renames this comparison count from $U$ to $"Dominance"(vx, vy)$,
  clarifying its purpose: measuring how often one sample dominates the other in pairwise comparisons.

The distribution of $"Dominance"$ determines which order statistics form reliable bounds.
Define the margin function:

$ PairwiseMargin(n, m, misrate) = "number of pairwise differences to exclude from bounds" $

This function computes how many extreme pairwise differences
  could occur by chance with probability $misrate$,
  based on the distribution of pairwise comparisons.

The $PairwiseMargin$ function requires knowing the distribution of pairwise comparisons under sampling.
Two computational approaches exist:

- *Exact computation* (Löffler's algorithm, 1982).
  Uses a recurrence relation to compute the exact distribution
  of pairwise comparisons for small samples.
  Practical for combined sample sizes up to several hundred.
- *Approximation* (Edgeworth expansion, 1955).
  Refines the normal approximation with correction terms
  based on higher moments of the distribution.
  Provides accurate results for large samples where exact computation becomes impractical.

The toolkit automatically selects the appropriate method based on sample sizes,
  ensuring both accuracy and computational efficiency.

This approach naturally complements $Center$ and $Spread$:

- $Center(vx)$ uses the median of pairwise averages $(x_i + x_j)\/2$
- $Spread(vx)$ uses the median of pairwise differences $abs(x_i - x_j)$
- $Shift(vx, vy)$ uses the median of pairwise differences $x_i - y_j$
- $ShiftBounds(vx, vy, misrate)$ uses order statistics of the same pairwise differences

All procedures build on pairwise operations.
This structural consistency reflects the mathematical unity underlying robust statistics:
  pairwise operations provide natural robustness
  while maintaining computational feasibility and statistical efficiency.

The inversion from hypothesis testing to bounds estimation
  represents a philosophical shift in statistical practice.
Traditional methods ask "should I believe this specific hypothesis?"
Pragmatic methods ask "what should I believe, given this data?"
Bounds provide actionable answers:
  they tell practitioners which values are plausible,
  enabling informed decisions without arbitrary significance thresholds.

Traditional Mann-Whitney implementations apply tie correction when samples contain repeated values.
This correction modifies variance calculations to account for tied observations,
  changing $p$-values and confidence intervals in ways that depend on measurement precision.
The toolkit deliberately omits tie correction.
Continuous distributions produce theoretically distinct values;
  observed ties result from finite measurement precision and digital representation.
When measurements appear identical, this reflects rounding of underlying continuous variation,
  not true equality in the measured quantity.
Treating ties as artifacts of discretization rather than distributional features
  simplifies computation while maintaining accuracy.
The exact and approximate methods compute comparison distributions
  without requiring adjustments for tied values,
  eliminating a source of complexity and potential inconsistency in statistical practice.

*Historical Development*

The mathematical foundations emerged through decades of refinement.
Mann and Whitney (1947) established the distribution of pairwise comparisons under random sampling,
  creating the theoretical basis for comparing samples through rank-based methods.
Their work demonstrated that comparison counts follow predictable patterns
  regardless of the underlying population distributions.

The original computational approaches suffered from severe limitations.
Mann and Whitney proposed a slow exact method requiring exponential resources
  and a normal approximation that proved grossly inaccurate for practical use.
The approximation works reasonably in distribution centers
  but fails catastrophically in the tails where practitioners most need accuracy.
For moderate sample sizes, approximation errors can exceed factors of $10^11$.

Fix and Hodges (1955) addressed the approximation problem through higher-order corrections.
Their expansion adds terms based on the distribution's actual moments
  rather than assuming perfect normality.
This refinement reduces tail probability errors from orders of magnitude to roughly 1%,
  making approximation practical for large samples where exact computation becomes infeasible.

Löffler (1982) solved the exact computation problem through algorithmic innovation.
The naive recurrence requires quadratic memory—
  infeasible for samples beyond a few dozen measurements.
Löffler discovered a reformulation that reduces memory to linear scale,
  making exact computation practical for combined sample sizes up to several hundred.

Despite these advances, most statistical software continues using the 1947 approximation.
The computational literature contains the solutions,
  but software implementations lag decades behind theoretical developments.
This toolkit implements both the exact method for small samples
  and the refined approximation for large samples,
  automatically selecting the appropriate approach based on sample sizes.

The shift from hypothesis testing to bounds estimation requires no new mathematics.
The same comparison distributions that enable hypothesis tests
  also determine which order statistics form reliable bounds.
Traditional applications ask "is zero plausible?" and answer yes or no.
This toolkit asks "which values are plausible?" and answers with an interval.
The perspective inverts while the mathematical foundation remains identical.
