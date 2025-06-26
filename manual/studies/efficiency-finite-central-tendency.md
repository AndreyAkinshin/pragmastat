## Finite-Sample Efficiency of Central Tendency Estimators

This study presents finite-sample efficiency values for $\Center$ and shows how it performs better than $\Median$
  across small and medium sample sizes.

The previous studies established asymptotic efficiency values — the limiting behavior
  as sample size approaches infinity.
For the Gaussian distribution, these asymptotic values are:

- $\Mean$: $100\%$ (the most efficient estimator under normality)
- $\Center$: $3/\pi \approx 95.5\%$
- $\Median$: $2/\pi \approx 63.7\%$

Asymptotic theory provides excellent approximations for large samples but may be inaccurate for small $n$.
Finite-sample efficiency captures the actual precision when working with limited data.

**Efficiency and Sample Size Requirements**

Efficiency quantifies the variance ratio between estimators.
For two estimators $T_1$ and $T_2$ applied to the same distribution:

$$
\eff(T_1 \text{ relative to } T_2) = \frac{\Var[T_2]}{\Var[T_1]}
$$

This ratio directly translates to sample size requirements.
An estimator with $80\%$ efficiency needs $100/80 = 1.25$ times as many observations
  to achieve the same precision as the reference estimator.
The $\Median$ with its $63.7\%$ asymptotic efficiency requires roughly $1.57$ times more data
  than the $\Mean$ under normality.

**Monte Carlo Estimation of Finite-Sample Efficiency**

Numerical simulation provides exact efficiency values for any sample size.
The procedure follows these steps:

1. **Generate samples**: Draw $m$ independent samples of size $n$ from the standard normal distribution
2. **Calculate estimators**: Compute both estimators for each sample
3. **Measure dispersion**: Calculate the sample variance of the $m$ estimator values
4. **Compute efficiency**: Take the variance ratio

The simulation must balance computational cost against precision.
Larger $m$ reduces Monte Carlo error but increases computation time.
For efficiency estimation, $m = 10^6$ iterations achieve enough precision.

**Finite-Sample Results**

The simulation reveals how efficiency evolves from small to moderate sample sizes.

The figure below shows the Gaussian efficiency curves for $n \in \{3, \ldots, 100\}$ based on $m = 10^6$ Monte Carlo iterations
  (dotted lines show asymptotic values):

<!-- IMG efficiency-central-tendency -->

**Key Observations**

1. **Small sample behavior**: For $n = 3$, $\Center$ achieves $97.9\%$ efficiency while $\Median$ drops to $74.3\%$.
   Even at extreme small samples, $\Center$ maintains above $91\%$ efficiency throughout.

2. **Convergence patterns**: The $\Center$ estimator reaches $94\%$ efficiency by $n = 15$
   and stabilizes above $95\%$ for $n \geq 50$.
   The $\Median$ oscillates between $63.5\%$ and $68\%$ across all sample sizes,
   never approaching the efficiency of $\Center$.

3. **Practical advantage**: For typical applications ($n = 20$ to $50$), $\Center$ maintains $94-95\%$ efficiency.
   This represents only a $5-6\%$ penalty compared to $\Mean$, while $\Median$ suffers a $35-37\%$ penalty.

The toolkit's $\Center$ estimator performs much better than $\Median$
  while sacrificing minimal efficiency compared to the non-robust $\Mean$.
