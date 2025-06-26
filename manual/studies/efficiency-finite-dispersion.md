## Finite-Sample Efficiency of Dispersion Estimators

This study presents finite-sample efficiency values for $\Spread$ and demonstrates its consistent superiority over
  the *standard deviation* $\StdDev$ and the *median absolute deviation* $\MAD$ across small and medium sample sizes.

The previous studies established asymptotic efficiency values for dispersion estimators under the Gaussian distribution.
These asymptotic values are:

- $\StdDev$: $100\%$ (the most efficient estimator under normality)
- $\Spread$: $\approx 86.4\%$
- $\MAD$: $\approx 36.8\%$

The asymptotic values provide excellent approximations for large samples but may not accurately reflect performance
  with limited data.
Understanding actual efficiency at practical sample sizes guides estimator selection from the toolkit.

**Efficiency Measurement for Dispersion Estimators**

Dispersion estimators face a variance-bias trade-off: different estimators have different expected values
  under the same distribution.
To compare their variability fairly, the simulation normalizes each estimator by its expected value.
This normalization eliminates bias differences and focuses purely on variance comparison.

The procedure follows these steps:

1. **Generate samples**: Draw $m$ independent samples of size $n$ from the standard normal distribution
2. **Calculate estimators**: Compute $\StdDev$, $\Spread$, and $\MAD$ for each sample
3. **Calculate normalized variance**: for each set of estimations, calculate the variance and divide by the mean value to align estimator biases
4. **Compute efficiency**: Take the variance ratio relative to $\StdDev$

This approach ensures fair comparison by measuring how much each estimator varies around its own expected value.
The simulation uses $m = 10^6$ iterations to achieve sufficient precision.

**Results and Analysis**

The simulation covers sample sizes $n \in \{3, \ldots, 100\}$ with efficiency computed relative to standard deviation.

The below figure shows the Gaussian efficiency curves based on $10^6$ Monte Carlo iterations
  (dotted lines show asymptotic values):

<!-- IMG efficiency-dispersion -->

**Key Observations**

1. **Superior efficiency**: $\Spread$ consistently outperforms $\MAD$ by a factor of two across all sample sizes.
   At $n = 20$, $\Spread$ achieves $81.4\%$ efficiency while $\MAD$ reaches only $39.0\%$.

2. **Practical implications**: For typical sample sizes ($n = 20$ to $50$), $\Spread$ maintains $81-84\%$ efficiency.
   This $16-19\%$ penalty compared to $\StdDev$ represents a reasonable trade-off for robustness.

3. **Small sample behavior**: $\Spread$ shows a characteristic dip for $n = 5-8$ (dropping to $74\%$) before recovering.
   Even at its worst, $\Spread$ remains twice as efficient as $\MAD$.

$\Spread$ from the toolkit provides a substantial efficiency advantage
  over $\MAD$ while maintaining reasonable performance compared to the non-robust standard deviation.
This efficiency advantage, combined with its robustness properties, makes $\Spread$ the preferred choice
  for dispersion estimation in practical applications.
