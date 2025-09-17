## Ratio

$$
\Ratio(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
$$

- Measures the median of pairwise ratios between elements of two samples
- Asymptotically, $\Ratio[X, Y]$ is the median of the ratio of random measurements from $X$ and $Y$
- Note: $\Ratio(\x, \y) \neq 1 / \Ratio(\y, \x)$ in general (example: $x=(1, 100)$, $y=(1, 10)$)
- Practical Domain: $x_i, y_j > 0$ or $x_i, y_j < 0$. In practice, exclude values with $|y_j|$ near zero (define a safe $\varepsilon$).
- Unit: relative

$$
\Ratio(\x, \x) = 1
$$

$$
\Ratio(k_x \cdot \x, k_y \cdot \y) = \frac{k_x}{k_y} \cdot \Ratio(\x, \y)
$$

