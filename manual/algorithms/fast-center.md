## Fast Center

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
This matrix has crucial properties: each row and column are sorted in non-decreasing order,
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

Real data often contain repeated values, which can cause the selection process to stall.
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
<!-- INCLUDE cs/Pragmastat/Algorithms/FastCenter.cs -->
```
