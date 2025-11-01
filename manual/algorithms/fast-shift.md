## Fast Shift

The $\Shift$ estimator measures the median of all pairwise differences between elements of two samples.
Given samples $\x = (x_1, x_2, \ldots, x_n)$ and $\y = (y_1, y_2, \ldots, y_m)$, this estimator is defined as:

$$
\Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

This definition represents a special case of a more general problem:
  computing arbitrary quantiles of all pairwise differences.
For samples of size $n$ and $m$, the total number of pairwise differences is $n \times m$.
A naive approach would materialize all differences, sort them, and extract the desired quantile.
With $n = m = 10,000$, this creates 100 million values,
  requiring quadratic memory and $O(nm \log(nm))$ time.

The algorithm avoids materializing any pairwise differences by exploiting the sorted structure of the samples.
After sorting both samples to obtain $x_1 \leq x_2 \leq \cdots \leq x_n$ and $y_1 \leq y_2 \leq \cdots \leq y_m$,
  the key insight is that we can count how many pairwise differences fall below any threshold value
  without computing them explicitly.
This counting operation enables binary search over the continuous space of possible difference values,
  iteratively narrowing the search range until convergence to the exact quantile.

The algorithm operates through value-space search rather than index-space selection.
It maintains a search interval $[\text{searchMin}, \text{searchMax}]$ initialized to the range
  of all possible differences: $[x_1 - y_m, x_n - y_1]$.
At each iteration, it selects a candidate value within this interval and counts
  how many pairwise differences are less than or equal to this threshold.
For the median (quantile $p = 0.5$), if fewer than half the differences lie below the threshold,
  the median must be larger; if more than half lie below, the median must be smaller.
Based on this comparison, the algorithm eliminates portions of the search space
  that cannot contain the target quantile.

The counting operation achieves linear complexity through a two-pointer sweep.
For a given threshold $t$, the algorithm counts how many pairs $(i,j)$ satisfy $x_i - y_j \leq t$.
This is equivalent to counting pairs where $y_j \geq x_i - t$.
For each row $i$ in the implicit matrix, the algorithm advances a column pointer
  through the sorted $y$ array while $x_i - y_j > t$, stopping at the first position
  where $x_i - y_j \leq t$.
All subsequent positions in that row satisfy the condition,
  contributing $(m - j)$ pairs to the count for row $i$.
Because both samples are sorted, the column pointer advances monotonically and never backtracks across rows,
  making each counting pass $O(n + m)$ regardless of the total number of differences.

During each counting pass, the algorithm tracks boundary values:
  the largest difference at or below the threshold and the smallest difference above it.
When the count exactly matches the target rank (or the two middle ranks for even-length samples),
  these boundary values provide the exact answer without additional searches.
For Type-7 quantile computation, which interpolates between order statistics,
  the algorithm collects the necessary boundary values in a single pass
  and performs linear interpolation: $(1 - w) \cdot \text{lower} + w \cdot \text{upper}$.

Real datasets often contain discrete or repeated values that can cause search stagnation.
The algorithm detects when the search interval stops shrinking between iterations,
  indicating that multiple pairwise differences share the same value.
When the closest difference below the threshold equals the closest above,
  all remaining candidates are identical and the algorithm terminates immediately.
Otherwise, it uses the boundary values from the counting pass to snap the search interval
  to actual difference values, ensuring reliable convergence even with highly discrete data.

The binary search employs numerically stable midpoint calculations and terminates
  when the search interval collapses to a single value or when boundary tracking confirms convergence.
The algorithm includes iteration limits as a safety mechanism,
  though convergence typically occurs much earlier due to the exponential narrowing of the search space.

The algorithm generalizes naturally to multiple quantiles by computing each one independently.
For $k$ quantiles with samples of size $n$ and $m$, the total complexity becomes $O(k(n + m) \log L)$,
  where $L$ represents the convergence precision.
This is dramatically more efficient than the naive $O(nm \log(nm))$ approach,
  especially when $n$ and $m$ are large but $k$ is small.
The algorithm requires only $O(1)$ additional space beyond the input arrays,
  making it practical for large-scale statistical analysis
  where memory constraints prohibit materializing quadratic structures.

```cs
<!-- INCLUDE cs/Pragmastat/Algorithms/FastShift.cs -->
```
