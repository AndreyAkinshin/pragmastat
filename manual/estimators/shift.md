## Shift

$$
\Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

**Practical Recommendations**

The $\Shift$ provides initial insight into the difference between elements of two samples.
Interpretation: half of $x_i-y_j$ values are smaller than $\Shift(\x, \y)$, the other half are larger.
For samples with small $\RelSpread$, $\Shift(\x, \y)$ approximates pairwise differences $x_i - y_j$.

**Key Facts**

- Measures the median difference between elements of two samples
- Domain: any real numbers
- Equals the *Hodges-Lehmann estimator* for two samples ([@hodges1963])

**Properties**

$$
\Shift(\x + k_x, \y + k_y) = \Shift(\x, \y) + k_x\!-\!k_y
$$

$$
\Shift(k \cdot \x, k \cdot \y) = k \cdot \Shift(\x, \y)
$$

$$
\Shift(\x, \y) = -\Shift(\y, \x)
$$
