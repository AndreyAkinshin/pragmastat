## MedRatio

$$
\MedRatio(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
$$

**Practical Recommendations**

$\MedRatio$ provides an initial insight into the ratio between elements of two samples expressed as a multiplicative factor.
It answers "how many times larger is $\x$ compared to $\y$?"
E.g., $\MedRatio = 2.0$ means that for $50\%$ of pairs $(x_i, y_j)$, $x_i$ is at least twice as large as $y_j$.

$\MedRatio$ functions as a division operator: $\MedRatio(\x, \y)$ computes the typical ratio $\x / \y$.
Results convert to percentage differences as $(\MedRatio - 1) \times 100\%$.

**Key Facts**

- Measures the median ratio between elements of two samples
- Domain: $y_j > 0$
- Second sample $y$ is always the baseline
- In general, $\MedRatio(\x, \y) \neq 1 / \MedRatio(\y, \x)$ (e.g., $x=(1, 100)$, $y=(1, 10)$)

**Properties**

$$
\MedRatio(k_x \cdot \x, k_y \cdot \y) = \frac{k_x}{k_y} \cdot \MedRatio(\x, \y)
$$

