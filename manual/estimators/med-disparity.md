## MedDisparity

$$
\MedDisparity(\x, \y) = \dfrac{\MedShift(\x, \y)}{\MedSpread(\x, \y)}
$$

**Practical Recommendations**

$\MedDisparity$ provides a scale-invariant insight into the absolute difference between elements of two samples,
  expressed in standardized spread units.

**Key Facts**

- Measures a normalized absolute difference between $\x$ and $\y$ expressed in standardized spread units
- Domain: $\MedSpread(\x, \y) > 0$ (at least $50\%$ of $|x_i-x_j|$ and $50\%$ of $|y_i-y_j|$ are non-zeros)
- Expresses the *effect size*, renamed to "Disparity" for clarity
- Scale-invariant, which makes an experiment design more portable

**Comparison**

- Compared to *Cohen's d*: more robust while maintaining efficiency under normality

**Properties**

$$
\MedDisparity(\x + k, \y + k) = \MedDisparity(\x, \y)
$$

$$
\MedDisparity(k\!\cdot\!\x, k\!\cdot\!\y) = \operatorname{sign}(k)\!\cdot\!\MedDisparity(\x, \y)
$$

$$
\MedDisparity(\x, \y) = -\MedDisparity(\y, \x)
$$
