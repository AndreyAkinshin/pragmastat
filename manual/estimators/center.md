## Center

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

- Measures central tendency (the average value)
- Provides initial insight into the magnitude of sample values
- When $\RelSpread$ is small, the entire sample can be approximated by the $\Center$ value
- Equals the *Hodges-Lehmann estimator* ([@hodges1963], [@sen1963]), renamed to "Center" for clarity
- Called *pseudomedian* in some texts because it is consistent with the median for symmetric distributions

**Comparison**

- Compared to the *mean*:
  - Robustness: tolerates almost one-third of outliers unlike the mean that can be corrupted by a single outlier
  - Efficiency: requires $4..10\%$ more measurements for the same precision
- Compared to the *median*:
  - Robustness: lower than the median, which tolerates half of the samples corrupted
      (although, such extreme robustness rarely provides practical advantage)
  - Efficiency: requires $1.5$ times fewer measurements for the same precision

**Properties**

- Domain: any real numbers
- Unit: the same as measurements
- Asymptotic Gaussian efficiency: $\approx 96\%$
- Finite-sample Gaussian efficiency $> 91\%$
- Asymptotic breakdown point: $\approx 29\%$ (matches $\Spread$)
- Location-dependent, scale-dependent

$$
\Center(\x + k) = \Center(\x) + k
$$

$$
\Center(k \cdot \x) = k \cdot \Center(\x)
$$
