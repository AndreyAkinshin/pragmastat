#import "/manual/definitions.typ": *

The $Ratio$ estimator measures the typical multiplicative relationship between elements of two samples.
Given samples $vx = (x_1, x_2, ..., x_n)$ and $vy = (y_1, y_2, ..., y_m)$ with all positive values, this estimator is defined as:

$ Ratio(vx, vy) = exp(median_(i,j)(log x_i - log y_j)) = exp(Shift(log vx, log vy)) $

A naive approach would compute all $n times m$ ratios, sort them, and extract the median.
With $n = m = 10000$, this creates 100 million values,
requiring quadratic memory and $O(n m log(n m))$ time.

The presented algorithm avoids this cost by exploiting the multiplicative-additive duality.
Taking the logarithm of a ratio yields a difference:

$ log(x_i / y_j) = log(x_i) - log(y_j) $

This transforms the problem of finding the median of pairwise ratios
into finding the median of pairwise differences in log-space.

The algorithm operates in three steps:

+ *Log-transform* — Apply $log$ to each element of both samples.
  If $vx$ was sorted, $log vx$ remains sorted (log is monotonically increasing for positive values).

+ *Delegate to Shift* — Use the #link(<sec-alg-shift>)[Shift] algorithm to compute
  the desired quantile of pairwise differences in log-space.
  This leverages the $O((m + n) log L)$ complexity of the Shift algorithm.

+ *Exp-transform* — Apply $exp$ to convert the result back to ratio-space.
  If the log-space median is $d$, then $exp(d) = exp(log(x_i) - log(y_j)) = x_i / y_j$ is the original ratio.

The total complexity is $O((m + n) log L)$ per quantile,
where $L$ represents the convergence precision in the log-space binary search.
This is dramatically more efficient than the naive $O(n m log(n m))$ approach.

Memory usage is $O(m + n)$ for storing the log-transformed samples,
compared to $O(n m)$ for materializing all pairwise ratios.

#source-include("cs/Pragmastat/Algorithms/FastRatio.cs", "cs")
