## Disparity

$$
\Disparity(\x, \y) = \dfrac{\Shift(\x, \y)}{\AvgSpread(\x, \y)}
$$

- Measures a normalized difference between $\x$ and $\y$ expressed in standardized spread units
- Provides insight into the difference between elements of two samples, expressed in standardized spread units
- Expresses the *effect size*, renamed to "Disparity" for clarity

**Comparison**

- Compared to *Cohen's d*:
  - Gaussian efficiency: good
  - Robustness: tolerates a decent portion of outliers unlike Cohen's d which can be corrupted by a single outlier
- Compared to $\Shift$: scale-invariant
- Compared to $\Ratio$: shift-invariant

**Properties**

- Domain: $\AvgSpread(\x, \y) > 0$ (in each sample, the portion of tied values does not exceed $50\%$)
- Unit: abstract spread unit
- Location-invariant, scale-invariant

$$
\Disparity(\x + k, \y + k) = \Disparity(\x, \y)
$$

$$
\Disparity(k\!\cdot\!\x, k\!\cdot\!\y) = \operatorname{sign}(k)\!\cdot\!\Disparity(\x, \y)
$$

$$
\Disparity(\x, \y) = -\Disparity(\y, \x)
$$

