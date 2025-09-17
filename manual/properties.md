# Summary Estimator Properties

This section compares the toolkit's robust estimators against traditional statistical methods
  to demonstrate their advantages and universally good properties.
While traditional estimators often work well under ideal conditions,
  the toolkit estimators maintain reliable performance across diverse real-world scenarios.

Average Estimators:

**Mean** (arithmetic average):
$$
\Mean(\x) = \frac{1}{n} \sum_{i=1}^{n} x_i
$$

**Median**:
$$
\Median(\x) = \begin{cases}
x_{((n+1)/2)} & \text{if } n \text{ is odd} \\
\frac{x_{(n/2)} + x_{(n/2+1)}}{2} & \text{if } n \text{ is even}
\end{cases}
$$

**Center** (Hodges-Lehmann estimator):
$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

Dispersion Estimators:

**Standard Deviation**:
$$
\StdDev(\x) = \sqrt{\frac{1}{n-1} \sum_{i=1}^{n} (x_i - \Mean(\x))^2}
$$

**Median Absolute Deviation** (around the median):
$$
\MAD(\x) = \Median(|x_i - \Median(\x)|)
$$

**Spread** (Shamos scale estimator):
$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

<!-- INCLUDE manual/properties/breakdown.md -->

<!-- INCLUDE manual/properties/drift.md -->

<!-- INCLUDE manual/properties/invariance.md -->
