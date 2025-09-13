## Disparity

$$
\Disparity(\x, \y) = \dfrac{\Shift(\x, \y)}{\AvgSpread(\x, \y)}
$$

**Practical Recommendations**

The $\Disparity$ provides scale-invariant insight into the difference between elements of two samples,
  expressed in standardized spread units.

**Key Facts**

- Measures a normalized difference between $\x$ and $\y$ expressed in standardized spread units
- Domain: $\AvgSpread(\x, \y) > 0$ (at least $50\%$ of $|x_i - x_j|$ and $50\%$ of $|y_i - y_j|$ are non-zero)
- Expresses the *effect size*, renamed to "Disparity" for clarity
- Scale-invariant, which makes experimental design more portable

**Comparison**

- Compared to *Cohen's d*: more robust while maintaining efficiency under normality

**Properties**

$$
\Disparity(\x + k, \y + k) = \Disparity(\x, \y)
$$

$$
\Disparity(k\!\cdot\!\x, k\!\cdot\!\y) = \operatorname{sign}(k)\!\cdot\!\Disparity(\x, \y)
$$

$$
\Disparity(\x, \y) = -\Disparity(\y, \x)
$$
