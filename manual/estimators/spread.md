## Spread

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

- Measures dispersion (variability, scatter)
- Provides insight into the median absolute difference between two random measurements
- Interpretation: half of $|x_i-x_j|$ values are smaller than $\Spread(\x)$, the other half are larger
- Corner case: for $n=1$, use $\Spread(\x) = 0$
- Equals the *Shamos scale estimator* ([@shamos1976]), renamed to "Spread" for clarity

**Comparison**

- Compared to the *standard deviation*:
  - Robustness: the spread tolerates almost one-third of outliers unlike the standard deviation that can be corrupted by a single outlier
  - Efficiency: good (asymptotic relative efficiency of the spread to the standard deviation is of $\approx 86\%$)
  - Usability: More intuitive without requiring knowledge of normal distributions
  - Consistency: no consistency under normality without multiplication by a constant
- Compared to the *median absolute deviation*:
  - Robustness: median tolerates half of the samples corrupted, which is more than one-third for the spread,
     but it does not provide practical advantage
  - Efficiency: requires $\approx 2.35$ times fewer observations for the same precision

$\Spread$ can be converted to an unbiased consistent estimator for another dispersion measure by multiplication by a constant

**Properties**

- Domain: any real numbers
- Unit: the same as measurements
- Location-invariant, scale-dependent
- Asymptotic Gaussian efficiency: $\approx 86\%$
- Asymptotic breakdown point: $\approx 29\%$ (matches $\Center$ in robustness)
- Asymptotic expected value for the standard normal distribution: $\approx 0.954$
- Coverage for the normal distribution:
  - $[\Center(\x) \pm 1 \cdot \Spread(\x)]$ covers $\approx 65.98518\%$ of the normal distribution
  - $[\Center(\x) \pm 2 \cdot \Spread(\x)]$ covers $\approx 94.35758\%$ of the normal distribution
  - $[\Center(\x) \pm 3 \cdot \Spread(\x)]$ covers $\approx 99.57851\%$ of the normal distribution
  - $[\Center(\x) \pm 4 \cdot \Spread(\x)]$ covers $\approx 99.98641\%$ of the normal distribution
  - $[\Center(\x) \pm 5 \cdot \Spread(\x)]$ covers $\approx 99.99982\%$ of the normal distribution

$$
\Spread(\x + k) = \Spread(\x)
$$

$$
\Spread(k \cdot \x) = |k| \cdot \Spread(\x)
$$

$$
\Spread(\x) \geq 0
$$
