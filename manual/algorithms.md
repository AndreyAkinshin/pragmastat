# Algorithms

This chapter describes the core algorithms that power the robust estimators in the toolkit.
Both algorithms solve a fundamental computational challenge: how to efficiently find medians within large collections
  of derived values without materializing the entire collection in memory.

## Fast Center Algorithm

The $\Center$ estimator computes the median of all pairwise averages from a sample.
Given a dataset $x = (x_1, x_2, \ldots, x_n)$, this estimator is defined as:

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

A direct implementation would generate all $\frac{n(n+1)}{2}$ pairwise averages and sort them.
With $n = 10,000$, this creates approximately 50 million values, requiring quadratic memory and $O(n^2 \log n)$ time.

The breakthrough came in 1984 when John Monahan developed an algorithm that reduces expected complexity
  to $O(n \log n)$ while using only linear memory (see [@monahan1984]).
The algorithm exploits the inherent structure in pairwise sums rather than computing them explicitly.
After sorting the input values $x_1 \leq x_2 \leq \cdots \leq x_n$,
  consider the implicit upper triangular matrix $M$ where $M_{i,j} = x_i + x_j$ for $i \leq j$.
This matrix has crucial properties: each row and column is sorted in non-decreasing order,
  enabling efficient median selection without materializing the quadratic structure.

Rather than sorting all pairwise sums, the algorithm uses a selection approach similar to quickselect.
The process maintains search bounds for each matrix row and iteratively narrows the search space.
For each row $i$, the algorithm tracks active column indices from $i+1$ to $n$,
  defining which pairwise sums remain candidates for the median.
It selects a candidate sum as a pivot using randomized selection from active matrix elements,
  then counts how many pairwise sums fall below the pivot.
Because both rows and columns are sorted, this counting takes only $O(n)$ time
  using a two-pointer sweep from the matrix's upper-right corner.

The median corresponds to rank $k = \lfloor \frac{N+1}{2} \rfloor$ where $N = \frac{n(n+1)}{2}$.
If fewer than $k$ sums lie below the pivot, the median must be larger;
  if more than $k$ sums lie below the pivot, the median must be smaller.
Based on this comparison, the algorithm eliminates portions of each row that cannot contain the median,
  shrinking the active search space while preserving the true median.

Real data often contains repeated values, which can cause the selection process to stall.
When the algorithm detects no progress between iterations, it switches to a midrange strategy:
  find the smallest and largest pairwise sums still in the search space,
  then use their average as the next pivot.
If the minimum equals the maximum, all remaining candidates are identical and the algorithm terminates.
This tie-breaking mechanism ensures reliable convergence with discrete or duplicated data.

The algorithm achieves $O(n \log n)$ time complexity through linear partitioning
  (each pivot evaluation requires only $O(n)$ operations) and logarithmic iterations
  (randomized pivot selection leads to expected $O(\log n)$ iterations, similar to quickselect).
The algorithm maintains only row bounds and counters, using $O(n)$ additional space
  regardless of the number of pairwise sums.
This matches the complexity of sorting a single array while avoiding the quadratic explosion
  of materializing all pairwise combinations.

```cs
<!-- INCLUDE dotnet/Pragmastat/Algorithms/FastCenterAlgorithm.cs -->
```

## Fast Spread Algorithm

The $\Spread$ estimator computes the median of all pairwise absolute differences.
Given a sample $x = (x_1, x_2, \ldots, x_n)$, this estimator is defined as:

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

Like $\Center$, computing $\Spread$ naively requires generating
  all $\frac{n(n-1)}{2}$ pairwise differences, sorting them, and finding the median —
  a quadratic approach that becomes computationally prohibitive for large datasets.

The same structural principles that accelerate $\Center$ computation apply to pairwise differences,
  yielding an exact $O(n \log n)$ algorithm.
After sorting the input to obtain $y_1 \leq y_2 \leq \cdots \leq y_n$,
  all pairwise absolute differences $|x_i - x_j|$ with $i < j$ become positive differences $y_j - y_i$.
Consider the implicit upper triangular matrix $D$ where $D_{i,j} = y_j - y_i$ for $i < j$.
This matrix inherits crucial structural properties: for fixed row $i$, differences increase monotonically,
  while for fixed column $j$, differences decrease as $i$ increases.
The sorted structure enables linear-time counting of elements below any threshold.

The algorithm applies Monahan's selection strategy adapted for differences rather than sums.
For each row $i$, it tracks active column indices representing differences still under consideration,
  initially spanning columns $i+1$ through $n$.
The algorithm chooses candidate differences from the active set using weighted random row selection,
  maintaining expected logarithmic convergence while avoiding expensive pivot computations.
For any pivot value $p$, it counts how many differences fall below $p$ using a single sweep,
  with the monotonic structure ensuring this counting requires only $O(n)$ operations.
While counting, the algorithm maintains the largest difference below $p$ and smallest difference at or above $p$ —
  these boundary values become the exact answer when the target rank is reached.

The algorithm handles both odd and even cases naturally.
For an odd number of differences, it returns the single middle element when the count exactly hits the median rank.
For an even number of differences, it returns the average of the two middle elements,
  with boundary tracking during counting providing both values simultaneously.
Unlike approximation methods, this algorithm returns the precise median of all pairwise differences,
  with randomness affecting only performance, not correctness.

The algorithm includes the same stall-handling mechanisms as the center algorithm.
It tracks whether the count below the pivot changes between iterations,
  and when progress stalls due to tied values, it computes the range of remaining active differences
  and pivots to their midrange.
This midrange strategy ensures convergence even with highly discrete data or datasets containing many identical values.

Several optimizations make the algorithm practical for production use.
A global column pointer that never moves backward during counting exploits the matrix structure
  to avoid redundant comparisons.
The algorithm captures exact boundary values during each counting pass,
  eliminating the need for additional searches when the target rank is reached.
Using only $O(n)$ additional space for row bounds and counters,
  independent of the quadratic number of pairwise differences,
  the algorithm achieves $O(n \log n)$ time complexity with minimal memory overhead,
  making robust scale estimation practical for large datasets.

```cs
<!-- INCLUDE dotnet/Pragmastat/Algorithms/FastSpreadAlgorithm.cs -->
```