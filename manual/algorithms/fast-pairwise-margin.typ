#import "/manual/definitions.typ": *

== Fast PairwiseMargin

The $PairwiseMargin$ function determines how many extreme pairwise differences to exclude
  when constructing bounds around $Shift(vx, vy)$.
Given samples $vx = (x_1, ..., x_n)$ and $vy = (y_1, ..., y_m)$,
  the $ShiftBounds$ estimator computes all $n m$ pairwise differences $z_(i j) = x_i - y_j$ and sorts them.
The bounds select specific order statistics from this sorted sequence:
  $[z_((k_"left")), z_((k_"right"))]$.
The challenge lies in determining which order statistics produce bounds
  that contain the true shift $Shift[X, Y]$ with probability $1 - misrate$.

Random sampling creates natural variation in pairwise differences.
Even when populations have identical distributions, sampling variation produces both positive and negative differences.
The margin function quantifies this sampling variability:
  it specifies how many extreme pairwise differences could occur by chance with probability $misrate$.
For symmetric bounds, this margin splits evenly between the tails,
  giving $k_"left" = floor(PairwiseMargin(n, m, misrate) / 2) + 1$
  and $k_"right" = n m - floor(PairwiseMargin(n, m, misrate) / 2)$.

Computing the margin requires understanding the distribution of pairwise comparisons.
Each pairwise difference corresponds to a comparison: $x_i - y_j > 0$ exactly when $x_i > y_j$.
This connection motivates the dominance function:

$ "Dominance"(vx, vy) = sum_(i=1)^n sum_(j=1)^m bb(1)(x_i > y_j) $

The dominance function counts how many pairwise comparisons favor $vx$ over $vy$.
Both $Shift$ and $"Dominance"$ operate on the same collection of $n m$ pairwise differences.
The $Shift$ estimator examines difference values, returning the median as a location estimate.
The $"Dominance"$ function examines difference signs,
  counting how many comparisons produce positive differences.
While $Shift$ provides the estimate itself,
  $"Dominance"$ determines which order statistics form reliable bounds around that estimate.

When populations have equivalent distributions, $"Dominance"$ concentrates near $n m \/ 2$ by symmetry.
The distribution of $"Dominance"$ across all possible sample orderings determines reliable bounds.
If $"Dominance"$ deviates from $n m \/ 2$ by at least $k\/2$ with probability $misrate$,
  then the interval excluding the $k$ most extreme pairwise differences
  contains zero with probability $1 - misrate$.
Translation invariance extends this relationship to arbitrary shifts:
  the margin computed from the comparison distribution applies regardless of the true shift value.

Two computational approaches provide the distribution of $"Dominance"$:
  exact calculation for small samples and approximation for large samples.

*Exact method*

Small sample sizes allow exact computation without approximation.
The exact approach exploits a fundamental symmetry: under equivalent populations,
  all $binom(n+m, n)$ orderings of the combined measurements occur with equal probability.
This symmetry enables direct calculation of how many orderings produce each comparison count.

Direct computation faces a combinatorial challenge.
Enumerating all orderings to count comparison outcomes requires substantial memory and computation time.
For samples beyond a few dozen measurements, naive implementation becomes impractical.

Löffler's recurrence relation (@loeffler1982) resolves this through algebraic structure.
The recurrence exploits cycle properties in the comparison distribution,
  reducing memory requirements while maintaining exact calculation.
The algorithm builds cumulative probabilities sequentially
  until reaching the threshold corresponding to the desired error rate.
This approach extends practical exact computation to combined sample sizes of several hundred.

Define $p_(n,m)(c)$ as the number of orderings producing exactly $c$ comparisons favoring $vx$.
The probability mass function becomes:

$ Pr("Dominance" = c) = p_(n,m)(c) / binom(n+m, n) $

A direct recurrence follows from considering the largest measurement.
The rightmost element comes from either $vx$ (contributing $m$ comparisons)
  or $vy$ (contributing zero):

$ p_(n,m)(c) = p_(n-1,m)(c - m) + p_(n,m-1)(c) $

with base cases $p_(n,0)(0) = 1$ and $p_(0,m)(0) = 1$.

Direct implementation requires $O(n dot m dot n m)$ time and $O(n m)$ memory.
An alternative recurrence (@loeffler1982) exploits cycle structure:

$ p_(n,m)(c) = 1/c sum_(i=0)^(c-1) p_(n,m)(i) dot sigma_(n,m)(c - i) $

where $sigma_(n,m)(d)$ captures structural properties through divisors:

$ sigma_(n,m)(d) = sum_(k|d) epsilon_k dot k, quad
epsilon_k = cases(
  1 & "if" 1 <= k <= n,
  -1 & "if" m+1 <= k <= m+n,
  0 & "otherwise"
) $

This reduces memory to $O(n m)$ and enables efficient computation through $c = n m$.

The algorithm computes cumulative probabilities $Pr("Dominance" <= c)$ sequentially
  until the threshold $misrate\/2$ is exceeded.
By symmetry, the lower and upper thresholds determine the total margin $PairwiseMargin = 2c$.

The sequential computation proceeds incrementally.
Starting from $u = 0$ with base probability $p_(n,m)(0) = 1$,
  the algorithm computes $p_(n,m)(1)$, then $p_(n,m)(2)$, and so on,
  accumulating the cumulative distribution function with each step.
The loop terminates as soon as $Pr("Dominance" <= u)$ reaches $misrate\/2$,
  returning the threshold value $u$ without computing further probabilities.

This sequential approach performs particularly well for small misrates.
For $misrate = 10^(-6)$, the threshold $u$ typically remains small even with large sample sizes,
  requiring only a few iterations regardless of whether $n$ and $m$ equal 50 or 200.
The algorithm computes only the extreme tail probabilities needed to reach the threshold,
  never touching the vast majority of probability mass concentrated near $n m \/ 2$.
This efficiency advantage grows as misrates decrease:
  stricter bounds require fewer computed values,
  making exact calculation particularly attractive for high-confidence applications.

*Approximate method*

Large samples make exact computation impractical.
The dominance count $"Dominance"$ concentrates near $n m \/ 2$ with variance $n m(n+m+1)\/12$.
A basic $Additive$ (Normal) approximation suffices asymptotically:

$ "Dominance" approx Additive(n m \/ 2, sqrt(n m(n+m+1)\/12)) $

This approximation underestimates tail probabilities for moderate sample sizes.
The $Additive$ (Normal) approximation provides a convenient baseline
  but fails to capture the true distribution shape in the tails,
  producing mis-calibrated probabilities that become problematic for small error rates.

The Edgeworth expansion refines this approximation through moment-based corrections (@fix1955).
The expansion starts with the $Additive$ (Normal) cumulative distribution as a baseline,
  then adds correction terms that account for the distribution's asymmetry (skewness) and tail weight (kurtosis).
These corrections use Hermite polynomials to adjust the baseline curve
  where the $Additive$ (Normal) approximation deviates most from the true distribution.
The first few correction terms typically achieve the practical balance between accuracy and computational cost,
  substantially improving tail probability estimates compared to the basic approximation.

The standardized comparison count:

$ z = (c - n m\/2) / sqrt(n m(n+m+1)\/12) $

produces the approximated cumulative distribution:

$ Pr("Dominance" <= c) approx Phi(z) + e_3 phi^((3))(z) + e_5 phi^((5))(z) + e_7 phi^((7))(z) $

where $Phi$ denotes the standard $Additive$ (Normal) CDF.

The correction coefficients depend on standardized moments:

$ e_3 = 1/24 (mu_4 / mu_2^2 - 3), quad
e_5 = 1/720 (mu_6 / mu_2^3 - 15 mu_4 / mu_2^2 + 30), quad
e_7 = 35/40320 (mu_4 / mu_2^2 - 3)^2 $

The moments $mu_2$, $mu_4$, $mu_6$ are computed from sample sizes:

$ mu_2 = (n m(n+m+1)) / 12 $

$ mu_4 = (n m(n+m+1)) / 240 (5 n m(n+m) - 2(n^2 + m^2) + 3 n m - 2(n+m)) $

$ mu_6 = (n m(n+m+1)) / 4032 (35 n^2 m^2(n^2 + m^2) + 70 n^3 m^3 - 42 n m(n^3 + m^3) \
  - 14 n^2 m^2(n + m) + 16(n^4 + m^4) - 52 n m(n^2 + m^2) \
  - 43 n^2 m^2 + 32(n^3 + m^3) + 14 n m(n + m) \
  + 8(n^2 + m^2) + 16 n m - 8(n + m)) $

The correction terms use Hermite polynomials:

$ phi^((k))(z) = -phi(z) H_k(z) $

$ H_3(z) = z^3 - 3z, quad
H_5(z) = z^5 - 10z^3 + 15z, quad
H_7(z) = z^7 - 21z^5 + 105z^3 - 105z $

Binary search locates the threshold value efficiently.
The algorithm maintains a search interval $[a, b]$ initialized to $[0, n m]$.
Each iteration computes the midpoint $c = (a + b)\/2$ and evaluates the Edgeworth CDF at $c$.
If $Pr("Dominance" <= c) < misrate\/2$, the threshold lies above $c$ and the search continues with $a = c$.
If $Pr("Dominance" <= c) >= misrate\/2$, the threshold lies below $c$ and the search continues with $b = c$.
The loop terminates when $a$ and $b$ become adjacent, requiring $O(log(n m))$ CDF evaluations.

This binary search exhibits uniform performance across misrate values.
Whether computing bounds for $misrate = 10^(-6)$ or $misrate = 0.05$,
  the algorithm performs the same number of iterations determined solely by the sample sizes.
Each CDF evaluation costs constant time regardless of the threshold location,
  making the approximate method particularly efficient for large samples where exact computation becomes impractical.
The logarithmic scaling ensures that doubling the sample size adds only one additional iteration,
  enabling practical computation for samples in the thousands or tens of thousands.

The toolkit selects between exact and approximate computation based on combined sample size:
  exact method for $n + m <= 400$, approximate method for $n + m > 400$.
The exact method guarantees correctness but scales as $O(n m)$ memory and $O((n m)^2)$ time.
For $n = m = 200$, this requires 40,000 memory locations.
The approximate method achieves 1% accuracy with $O(log(n m))$ constant-time evaluations.
For $n = m = 10000$, the approximate method completes in milliseconds versus minutes for exact computation.

Both methods handle discrete data.
Repeated measurements produce tied pairwise differences,
  creating plateaus in the sorted sequence.
The exact method counts orderings without assuming continuity.
The approximate method's moment-based corrections capture the actual distribution shape
  regardless of discreteness.

*Minimum reasonable misrate*

The $misrate$ parameter controls how many extreme pairwise differences the bounds exclude.
Lower misrate produces narrower bounds with higher confidence but requires excluding fewer extreme values.
However, sample size limits how small misrate can meaningfully become.

Consider the most extreme configuration:
  all measurements from $vx$ exceed all measurements from $vy$, giving $x_1, ..., x_n > y_1, ..., y_m$.
Under equivalent populations, this arrangement occurs purely by chance.
The probability equals the chance of having all $n$ elements from $vx$
  occupy the top $n$ positions among $n+m$ total measurements:

$ misrate_min = 1 / binom(n+m, n) = (n! dot m!) / (n+m)! $

This represents the minimum probability of the most extreme ordering under random sampling.
Setting $misrate < misrate_min$ makes bounds construction problematic.
The exact distribution of $"Dominance"$ cannot support misrates smaller than the probability
  of its most extreme realization.
Attempting to construct bounds with $misrate < misrate_min$ forces the algorithm
  to exclude zero pairwise differences from the tails, making $PairwiseMargin = 0$.
The resulting bounds span all $n m$ pairwise differences,
  returning $[z_((1)), z_((n m))]$ regardless of the desired confidence level.
These bounds convey no useful information beyond the range of observed pairwise differences.

For small samples, $misrate_min$ can exceed commonly used values.
With $n = m = 6$, the minimum misrate equals $1\/binom(12, 6) approx 0.00108$,
  making the typical choice of $misrate = 10^(-3)$ impossible.
With $n = m = 4$, the minimum becomes $1\/binom(8, 4) approx 0.0143$,
  exceeding even $misrate = 0.01$.

The table below shows $misrate_min$ for small sample sizes:

#table(
  columns: 11,
  align: (left, right, right, right, right, right, right, right, right, right, right),
  stroke: none,
  table.hline(),
  [], [1], [2], [3], [4], [5], [6], [7], [8], [9], [10],
  table.hline(),
  [1], [0.500000], [0.333333], [0.250000], [0.200000], [0.166667], [0.142857], [0.125000], [0.111111], [0.100000], [0.090909],
  [2], [0.333333], [0.166667], [0.100000], [0.066667], [0.047619], [0.035714], [0.027778], [0.022222], [0.018182], [0.015152],
  [3], [0.250000], [0.100000], [0.050000], [0.028571], [0.017857], [0.011905], [0.008333], [0.006061], [0.004545], [0.003497],
  [4], [0.200000], [0.066667], [0.028571], [0.014286], [0.007937], [0.004762], [0.003030], [0.002020], [0.001399], [0.000999],
  [5], [0.166667], [0.047619], [0.017857], [0.007937], [0.003968], [0.002165], [0.001263], [0.000777], [0.000500], [0.000333],
  [6], [0.142857], [0.035714], [0.011905], [0.004762], [0.002165], [0.001082], [0.000583], [0.000333], [0.000200], [0.000125],
  [7], [0.125000], [0.027778], [0.008333], [0.003030], [0.001263], [0.000583], [0.000291], [0.000155], [0.000087], [0.000051],
  [8], [0.111111], [0.022222], [0.006061], [0.002020], [0.000777], [0.000333], [0.000155], [0.000078], [0.000041], [0.000023],
  [9], [0.100000], [0.018182], [0.004545], [0.001399], [0.000500], [0.000200], [0.000087], [0.000041], [0.000021], [0.000011],
  [10], [0.090909], [0.015152], [0.003497], [0.000999], [0.000333], [0.000125], [0.000051], [0.000023], [0.000011], [0.000005],
  table.hline(),
)

For meaningful bounds construction, choose $misrate > misrate_min$.
This ensures the margin function excludes at least some extreme pairwise differences,
  producing bounds narrower than the full range.
When working with small samples, verify that the desired misrate exceeds $misrate_min$
  for the given sample sizes.
With moderate sample sizes ($n, m >= 15$), $misrate_min$ drops below $10^(-8)$,
  making standard choices like $misrate = 10^(-6)$ feasible.

#source-include("cs/Pragmastat/Functions/PairwiseMargin.cs", "cs")
