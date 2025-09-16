## Ratio

$$
\Ratio(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
$$

- Measures the median ratio between elements of two samples as a multiplicative factor
- Second sample $y$ is always the baseline
- In general, $\Ratio(\x, \y) \neq 1 / \Ratio(\y, \x)$ (example: $x=(1, 100)$, $y=(1, 10)$)
- Results convert to percentage differences as $(\Ratio - 1) \times 100\%$
- Interpretation example: $\Ratio = 2.0$ means that for $50\%$ of pairs $(x_i, y_j)$, $x_i$ is at least twice as large as $y_j$

**Properties**

- Domain: $y_j > 0$
- Unit: relative (dimensionless)
- Location-dependent, scale-invariant

$$
\Ratio(k_x \cdot \x, k_y \cdot \y) = \frac{k_x}{k_y} \cdot \Ratio(\x, \y)
$$

