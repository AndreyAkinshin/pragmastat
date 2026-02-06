#import "/manual/definitions.typ": *

== Fast SignedRankMargin <sec-fast-signed-rank-margin>

The $SignedRankMargin$ function determines how many extreme pairwise averages to exclude
  when constructing bounds around $Center(vx)$.
Given a sample $vx = (x_1, ..., x_n)$,
  the $CenterBounds$ estimator computes all $N = n(n+1)/2$ pairwise averages $w_(i j) = (x_i + x_j) / 2$ for $i <= j$ and sorts them.
The bounds select specific order statistics from this sorted sequence:
  $[w_((k_"left")), w_((k_"right"))]$.
The challenge lies in determining which order statistics produce bounds
  that contain the true center with probability $1 - misrate$.

The margin function is the one-sample analog of $PairwiseMargin$.
While $PairwiseMargin$ uses the Mann-Whitney distribution for two-sample comparisons,
  $SignedRankMargin$ uses the Wilcoxon signed-rank distribution for one-sample inference.
Under the weak symmetry assumption, the signed-rank statistic has a known distribution
  that enables exact computation of bounds coverage.

For symmetric distributions, consider the signs of deviations from the center.
The Wilcoxon signed-rank statistic $W$ sums the ranks of positive deviations:

$ W = sum_(i=1)^n R_i dot bb(1)(x_i > theta) $

where $R_i$ is the rank of $abs(x_i - theta)$ among all $abs(x_j - theta)$,
and $theta$ is the true center.
Under symmetry, each deviation is equally likely to be positive or negative,
giving $W$ a discrete distribution over $[0, n(n+1)/2]$.

The connection to pairwise averages is fundamental:
the $k$-th order statistic of sorted pairwise averages corresponds to
a specific threshold of the signed-rank statistic.
By computing the distribution of $W$, we determine which order statistics
provide bounds with the desired coverage.

Two computational approaches provide the distribution of $W$:
  exact calculation for small samples and approximation for large samples.

*Exact method*

Small sample sizes allow exact computation without approximation.
The Wilcoxon signed-rank distribution has $2^n$ equally likely outcomes under symmetry,
corresponding to all possible sign patterns for deviations from the center.

Dynamic programming builds the probability mass function efficiently.
Define $p(w)$ as the number of sign patterns producing signed-rank statistic equal to $w$.
The recurrence considers whether to include rank $i$ in the positive sum:

$ p_i(w) = p_(i-1)(w - i) + p_(i-1)(w) $

with base case $p_0(0) = 1$.
This builds the distribution incrementally, rank by rank.

The algorithm computes cumulative probabilities $Pr(W <= w)$ sequentially
  until the threshold $misrate\/2$ is exceeded.
For symmetric two-tailed bounds, the margin becomes $SignedRankMargin = 2w$.
Memory is $O(n^2)$ for storing the probability array,
  and time is $O(n^3)$ for the full computation.

The sequential computation performs well for small misrates.
For $misrate = 10^(-6)$, the threshold $w$ typically remains small,
  requiring only iterations through the lower tail regardless of sample size.

*Approximate method*

Large samples make exact computation impractical.
For $n > 63$, the Wilcoxon distribution is approximated using an Edgeworth expansion.

Under symmetry, the signed-rank statistic $W$ has:

$ EE[W] = n(n+1) / 4, quad Var(W) = n(n+1)(2n+1) / 24 $

The basic normal approximation uses these moments directly,
but underestimates tail probabilities for moderate sample sizes.

The Edgeworth expansion refines this through moment-based corrections.
The fourth central moment of $W$ is:

$ mu_4 = (9n^5 + 45n^4 + 65n^3 + 15n^2 - 14n) / 480 $

This enables kurtosis correction:

$ e_3 = (mu_4 - 3 sigma^4) / (24 sigma^4) $

The approximated CDF becomes:

$ Pr(W <= w) approx Phi(z) + e_3 phi(z)(z^3 - 3z) $

where $z = (w - mu + 0.5) / sigma$ includes a continuity correction.

Binary search locates the threshold efficiently.
Each CDF evaluation costs $O(1)$, and $O(log N)$ evaluations suffice.
The approximate method completes in constant time regardless of sample size.

The toolkit uses exact computation for $n <= 63$ and approximation for $n > 63$.
At $n = 63$, the exact method requires arrays of size $2,016$ ($= 63 times 64 / 2$),
which remains practical on modern hardware.
The transition at $n = 63$ is determined by the requirement that $2^n$ fits in a 64-bit integer.
The approximation achieves sub-1% accuracy for $n > 100$,
making the transition smooth.

*Minimum achievable misrate*

The $misrate$ parameter controls how many extreme pairwise averages the bounds exclude.
However, sample size limits how small misrate can meaningfully become.

The most extreme configuration occurs when all signs are positive (or all negative):
$W = n(n+1)/2$ or $W = 0$.
Under symmetry, this extreme occurs with probability:

$ misrate_min = 2 / 2^n = 2^(1-n) $

Setting $misrate < misrate_min$ makes bounds construction problematic.
Pragmastat rejects such requests with a `domain` error.

The table below shows $misrate_min$ for small sample sizes:

#table(
  columns: 6,
  align: (center, center, center, center, center, center),
  stroke: none,
  table.hline(),
  [*$n$*], [*2*], [*3*], [*5*], [*7*], [*10*],
  table.hline(),
  [$misrate_min$], [0.5], [0.25], [0.0625], [0.0156], [0.00195],
  [max conf], [50%], [75%], [93.75%], [98.4%], [99.8%],
  table.hline(),
)

For meaningful bounds construction, choose $misrate > misrate_min$.
With $n >= 10$, standard choices like $misrate = 10^(-3)$ become feasible.
With $n >= 20$, even $misrate = 10^(-6)$ is achievable.

#source-include("cs/Pragmastat/Functions/SignedRankMargin.cs", "cs")
