#import "/manual/definitions.typ": *

== Fast Spread

The $Spread$ estimator computes the median of all pairwise absolute differences.
Given a sample $x = (x_1, x_2, ..., x_n)$, this estimator is defined as:

$ Spread(vx) = median_(1 <= i < j <= n) abs(x_i - x_j) $

Like $Center$, computing $Spread$ naively requires generating
  all $n(n-1)/2$ pairwise differences, sorting them, and finding the median —
  a quadratic approach that becomes computationally prohibitive for large datasets.

The same structural principles that accelerate $Center$ computation also apply to pairwise differences,
  yielding an exact $O(n log n)$ algorithm.
After sorting the input to obtain $y_1 <= y_2 <= ... <= y_n$,
  all pairwise absolute differences $abs(x_i - x_j)$ with $i < j$ become positive differences $y_j - y_i$.
This allows considering the implicit upper triangular matrix $D$ where $D_(i,j) = y_j - y_i$ for $i < j$.
This matrix has a crucial structural property: for a fixed row $i$, differences increase monotonically,
  while for a fixed column $j$, differences decrease as $i$ increases.
This sorted structure enables linear-time counting of elements below any threshold.

The algorithm applies Monahan's selection strategy, adapted for differences rather than sums.
For each row $i$, it tracks active column indices representing differences still under consideration,
  initially spanning columns $i+1$ through $n$.
It chooses candidate differences from the active set using weighted random row selection,
  maintaining expected logarithmic convergence while avoiding expensive pivot computations.
For any pivot value $p$, the number of differences falling below $p$ is counted using a single sweep;
  the monotonic structure ensures this counting requires only $O(n)$ operations.
While counting, the largest difference below $p$ and the smallest difference at or above $p$ are maintained —
  these boundary values become the exact answer when the target rank is reached.

The algorithm naturally handles both odd and even cases.
For an odd number of differences, it returns the single middle element when the count exactly hits the median rank.
For an even number of differences, it returns the average of the two middle elements;
  boundary tracking during counting provides both values simultaneously.
Unlike approximation methods, this algorithm returns the precise median of all pairwise differences;
  randomness affects only performance, not correctness.

The algorithm includes the same stall-handling mechanisms as the center algorithm.
It tracks whether the count below the pivot changes between iterations;
  when progress stalls due to tied values, it computes the range of remaining active differences
  and pivots to their midrange.
This midrange strategy ensures convergence even with highly discrete data or datasets with many identical values.

Several optimizations make the implementation practical for production use.
A global column pointer that never moves backward during counting exploits the matrix structure
  to avoid redundant comparisons.
Exact boundary values are captured during each counting pass,
  eliminating the need for additional searches when the target rank is reached.
Using only $O(n)$ additional space for row bounds and counters,
  the algorithm achieves $O(n log n)$ time complexity with minimal memory overhead,
  making robust scale estimation practical for large datasets.

#source-include("cs/Pragmastat/Algorithms/FastSpread.cs", "cs")
