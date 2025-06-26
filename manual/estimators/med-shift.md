## MedShift

$$
\MedShift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

**Practical Recommendations**

$\MedShift$ provides an initial insight into the absolute difference between elements of two samples.
Interpretation: half of $x_i-y_j$ is smaller than $\MedShift(\x, \y)$, the other half is larger.
For samples with small $\Volatility$, $\MedShift(\x, \y)$ approximates pairwise differences $x_i - y_j$.

**Key Facts**

- Measures the median absolute difference between elements of two samples
- Domain: any real numbers
- Equals the *Hodges-Lehmann estimator* for two samples ([@hodges1963])

**Properties**

$$
\MedShift(\x + k_x, \y + k_y) = \MedShift(\x, \y) + k_x\!-\!k_y
$$

$$
\MedShift(k \cdot \x, k \cdot \y) = k \cdot \MedShift(\x, \y)
$$

$$
\MedShift(\x, \y) = - \MedShift(\y, \x)
$$
