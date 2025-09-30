## Disparity ('robust effect size')

$$
\Disparity(\x, \y) = \dfrac{\Shift(\x, \y)}{\AvgSpread(\x, \y)}
$$

- Measures a normalized $\Shift$ between $\x$ and $\y$ expressed in spread units
- Expresses the 'effect size', renamed to $\Disparity$ for clarity
- Pragmatic alternative to Cohen's d (note: exact estimates differ due to robust construction)
- Domain: $\AvgSpread(\x, \y) > 0$
- Unit: spread unit

$$
\Disparity(\x + k, \y + k) = \Disparity(\x, \y)
$$

$$
\Disparity(k\!\cdot\!\x, k\!\cdot\!\y) = \operatorname{sign}(k)\!\cdot\!\Disparity(\x, \y)
$$

$$
\Disparity(\x, \y) = -\Disparity(\y, \x)
$$

