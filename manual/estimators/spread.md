## Spread

$$
\Spread(\x) = \underset{i < j}{\Median} |x_i - x_j|
$$

**Practical Recommendations**

$\Spread$ provides an initial insight into the dispersion of the sample values.
Interpretation: half of $|x_i-x_j|$ is smaller than $\Spread(\x)$, the other half is larger.

**Key Facts**

- Measures dispersion (also known as variability or scatter)
- Domain: any real numbers (for $n=1$, it is convenient to use $\Spread(\x) = 0$)
- Equals the *Shamos estimator* ([@shamos1976]), renamed to "Spread" for clarity
- Asymptotic Gaussian efficiency: $\approx 86\%$
- Asymptotic breakdown point: $\approx 29\%$ (matches $\Center$ in robustness)
- Asymptotic expected value for the standard normal distribution: $\approx 0.954$
- Not consistent for the standard deviation under normality

**Comparison**

- Compared to the *standard deviation*: more robust (tolerates almost one-third of outliers)
  and has comparable efficiency under normality; more intuitive without requiring knowledge of normal distributions
- Compared to the *median absolute deviation*: more efficient under normality and requires $\approx 2.35$ times fewer observations for the same precision

**Empirical Rule**

For the standard normal distribution, the asymptotic 68–95–99.7 rule becomes the 66-94-99.6 rule:

- $[\Center(\x) \pm 1 \cdot \Spread(\x)]$ covers $\approx 65.98518\%$ of the distribution
- $[\Center(\x) \pm 2 \cdot \Spread(\x)]$ covers $\approx 94.35758\%$ of the distribution
- $[\Center(\x) \pm 3 \cdot \Spread(\x)]$ covers $\approx 99.57851\%$ of the distribution
- $[\Center(\x) \pm 4 \cdot \Spread(\x)]$ covers $\approx 99.98641\%$ of the distribution
- $[\Center(\x) \pm 5 \cdot \Spread(\x)]$ covers $\approx 99.99982\%$ of the distribution

**Properties**

$$
\Spread(\x + k) = \Spread(\x)
$$

$$
\Spread(k \cdot \x) = |k| \cdot \Spread(\x)
$$

$$
\Spread(x) \geq 0
$$
