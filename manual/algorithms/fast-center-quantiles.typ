#import "/manual/definitions.typ": *

== Fast CenterQuantiles <sec-fast-center-quantiles>

The $Center$ estimator computes the median of all pairwise averages $(x_i + x_j)/2$ for $i <= j$.
For $CenterBounds$, we need not just the median but specific order statistics:
the $k$-th smallest pairwise average for bounds computation.
Given a sample of size $n$, there are $N = n(n+1)/2$ such pairwise averages.

A naive approach would materialize all $N$ pairwise averages, sort them, and extract the desired quantile.
With $n = 10000$, this creates approximately 50 million values,
  requiring quadratic memory and $O(N log N)$ time.
The fast algorithm avoids materializing the pairs entirely.

The algorithm exploits the sorted structure of the implicit pairwise average matrix.
After sorting the input to obtain $x_1 <= x_2 <= ... <= x_n$,
  the pairwise averages form a symmetric matrix where both rows and columns are sorted.
Instead of searching through indices, the algorithm searches through values.
It maintains a search interval $["lo", "hi"]$, initialized to $[x_1, x_n]$
  (the range of all possible pairwise averages).
At each iteration, it asks: "How many pairwise averages are at most this threshold?"
Based on the count, it narrows the search interval.
For finding the $k$-th smallest pairwise average:
if $"count" >= k$, the target is at or below the threshold and the search continues in the lower half;
if $"count" < k$, the target is above the threshold and the search continues in the upper half.

The key operation is counting pairwise averages at or below a threshold $t$.
For each $i$, we need to count how many $j >= i$ satisfy $(x_i + x_j)/2 <= t$,
  equivalently $x_j <= 2t - x_i$.
Because the array is sorted, a single pass through the array suffices.
For each $i$, a pointer $j$ tracks the largest index where $x_j <= 2t - x_i$.
As $i$ increases, $x_i$ increases, so the threshold $2t - x_i$ decreases,
  meaning $j$ can only decrease (or stay the same).
This two-pointer technique ensures each element is visited at most twice,
  making the counting operation $O(n)$ regardless of the threshold value.

Binary search converges to an approximate value, but bounds require exact pairwise averages.
After the search converges, the algorithm identifies candidate exact values
  near the approximate threshold, then selects the one at the correct rank.
The candidate generation examines positions near the boundary:
for each row $i$, it finds the $j$ values where the pairwise average crosses the threshold,
collecting the actual pairwise average values.
Sorting these candidates and verifying ranks ensures the exact quantile is returned.

$CenterBounds$ needs two quantiles: $w_((k_"left"))$ and $w_((k_"right"))$.
The algorithm computes each independently using the same technique.
For symmetric bounds around the center, the lower and upper ranks are
$k_"left" = floor(SignedRankMargin / 2) + 1$ and $k_"right" = N - floor(SignedRankMargin / 2)$.

The algorithm achieves $O(n log n)$ time complexity for sorting,
  plus $O(n log R)$ for binary search where $R$ is the value range precision.
Memory usage is $O(n)$ for the sorted array plus $O(n)$ for candidate generation.
This is dramatically more efficient than the naive $O(n^2 log n^2)$ approach.
For $n = 10000$, the fast algorithm completes in milliseconds
  versus minutes for the naive approach.

The algorithm uses relative tolerance for convergence:
$"hi" - "lo" <= 10^(-14) dot max(1, abs("lo"), abs("hi"))$.
The floor of 1 prevents degenerate tolerance when bounds are near zero.
This ensures stable behavior across different scales of input data.
For candidate generation near the threshold, small tolerances prevent
missing exact values due to floating-point imprecision.

#source-include("cs/Pragmastat/Algorithms/FastCenterQuantiles.cs", "cs")
