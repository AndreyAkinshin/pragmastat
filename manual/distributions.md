# Distributions

This section defines the distributions used throughout the manual.

## Additive ('Normal')

$$
\Additive(\mathrm{mean}, \mathrm{stdDev})
$$

- $\mathrm{mean}$: location parameter (center of the distribution), consistent with $\Center$
- $\mathrm{stdDev}$: scale parameter (standard deviation), can be rescaled to $\Spread$

<!-- IMG distribution-additive -->

- **Formation:** the sum of many variables $X_1 + X_2 + \ldots + X_n$ under mild CLT (Central Limit Theorem) conditions (e.g., Lindeberg-Feller).
- **Origin:** historically called 'Normal' or 'Gaussian' distribution after Carl Friedrich Gauss and others.
- **Rename Motivation:** renamed to $\Additive$ to reflect its fundamental formation mechanism
  through addition rather than arbitrary historical attribution.
- **Properties:** symmetric, bell-shaped, characterized by central limit theorem convergence.
- **Applications:** measurement errors, heights and weights in populations, test scores, temperature variations.
- **Characteristics:** symmetric around the mean, light tails, finite variance.
- **Caution:** no perfectly additive distributions exist;
    all real data contain some deviations.
  Traditional estimators like $\Mean$ and $\StdDev$ lack robustness to outliers;
  use them only when strong evidence supports small deviations from additivity with no extreme measurements.

## Multiplic ('LogNormal')

$$
\Multiplic(\mathrm{logMean}, \mathrm{logStdDev})
$$

- $\mathrm{logMean}$: mean of log values (location parameter; $e^{\mathrm{logMean}}$ equals the geometric mean)
- $\mathrm{logStdDev}$: standard deviation of log values (scale parameter; controls multiplicative spread)

<!-- IMG distribution-multiplic -->

- **Formation:** the product of many positive variables $X_1 \cdot X_2 \cdot \ldots \cdot X_n$ with mild conditions (e.g., finite variance of $\log X$).
- **Origin:** historically called 'Log-Normal' or 'Galton' distribution after Francis Galton.
- **Rename Motivation:** renamed to $\Multiplic$ to reflect its fundamental formation mechanism
  through multiplication rather than arbitrary historical attribution.
- **Properties:** logarithm of a $\Multiplic$ ('LogNormal') variable follows an $\Additive$ ('Normal') distribution.
- **Applications:** stock prices, file sizes, reaction times, income distributions, biological growth rates.
- **Caution:** no perfectly multiplic distributions exist;
    all real data contain some deviations.
  Traditional estimators may struggle with the inherent skewness and heavy right tail.

## Exponential

$$
\Exp(\mathrm{rate})
$$

- $\mathrm{rate}$: rate parameter ($\lambda > 0$, controls decay speed; mean = $1/\mathrm{rate}$)

<!-- IMG distribution-exponential -->

- **Formation:** the waiting time between events in a Poisson process.
- **Origin:** naturally arises from memoryless processes where the probability
  of an event occurring is constant over time.
- **Properties:** memoryless property - past events do not affect future probabilities.
- **Applications:** time between failures, waiting times in queues, radioactive decay, customer service times.
- **Characteristics:** always positive, right-skewed with light (exponential) tail.
- **Caution:** extreme skewness makes traditional location estimators like $\Mean$ unreliable;
    robust estimators provide more stable results.

## Power ('Pareto')

$$
\Power(\mathrm{min}, \mathrm{shape})
$$

- $\mathrm{min}$: minimum value (lower bound, $\mathrm{min} > 0$)
- $\mathrm{shape}$: shape parameter ($\alpha > 0$, controls tail heaviness; smaller values = heavier tails)

<!-- IMG distribution-power -->

- **Formation:** follows a power-law relationship where large values are rare but possible.
- **Origin:** historically called 'Pareto' distribution after Vilfredo Pareto's work on wealth distribution.
- **Rename Motivation:** renamed to $\Power$ to emphasize the fundamental power-law behavior
  rather than historical attribution.
- **Properties:** exhibits scale invariance and extremely heavy tails.
- **Applications:** wealth distribution, city population sizes, word frequencies, earthquake magnitudes, website traffic.
- **Characteristics:** infinite variance for many parameter values, extreme outliers common.
- **Caution:** traditional variance-based estimators completely fail;
    robust estimators essential for reliable analysis.

## Uniform

$$
\Uniform(\mathrm{min}, \mathrm{max})
$$

- $\mathrm{min}$: lower bound of the support interval
- $\mathrm{max}$: upper bound of the support interval ($\mathrm{max} > \mathrm{min}$)

<!-- IMG distribution-uniform -->

- **Formation:** all values within a bounded interval have equal probability.
- **Origin:** represents complete uncertainty within known bounds.
- **Properties:** rectangular probability density, finite support with hard boundaries.
- **Applications:** random number generation, round-off errors, arrival times within known intervals.
- **Characteristics:** symmetric, bounded, no tail behavior.
- **Note:** while simple in concept, estimating bounds from samples presents challenges;
    traditional estimators work reasonably well due to symmetry and bounded nature.

