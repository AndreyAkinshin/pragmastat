## Power ('Pareto')

$$
\Power(\mathrm{min}, \mathrm{shape})
$$

- $\mathrm{min}$: minimum value (lower bound, $\mathrm{min} > 0$)
- $\mathrm{shape}$: shape parameter ($\alpha > 0$, controls tail heaviness; smaller values = heavier tails)

<!-- IMG distribution-power -->

- **Formation:** follows a power-law relationship where large values are rare but possible.
- **Origin:** historically called 'Pareto' distribution after Vilfredo Pareto's work on wealth distribution.
- **Rename Motivation:** renamed to $\Power$ to reflect its connection with power-law.
- **Properties:** exhibits scale invariance and extremely heavy tails.
- **Applications:** wealth distribution, city population sizes, word frequencies, earthquake magnitudes, website traffic.
- **Characteristics:** infinite variance for many parameter values; extreme outliers are common.
- **Caution:** traditional variance-based estimators completely fail;
    robust estimators are essential for reliable analysis.
