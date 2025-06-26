## Center

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

**Practical Recommendations**

$\Center$ provides an initial insight into the magnitude of sample values.
When $\Volatility$ is small, the whole sample can be approximated by the $\Center$ value.

**Key Facts**

- Measures central tendency (the average value)
- Domain: any real numbers
- Equals the *Hodges-Lehmann estimator* ([@hodges1963], [@sen1963]), renamed to "Center" for clarity
- Called *pseudomedian* in some texts because it is consistent with the median for symmetric distributions
- Asymptotic Gaussian efficiency: $\approx 96\%$; finite-sample Gaussian efficiency $> 91\%$
- Asymptotic breakdown point: $\approx 29\%$

**Comparison**

- Compared to the *mean*: more robust (tolerates almost one-third of outliers)
  and has comparable efficiency under normality
- Compared to the *median*: more efficient under normality and requires $1.5$ times fewer observations for the same precision

**Properties**

$$
\Center(\x + k) = \Center(\x) + k
$$

$$
\Center(k \cdot \x) = k \cdot \Center(\x)
$$
