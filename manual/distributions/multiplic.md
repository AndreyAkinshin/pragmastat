## Multiplic ('LogNormal')

$$
\Multiplic(\mathrm{logMean}, \mathrm{logStdDev})
$$

- $\mathrm{logMean}$: mean of log values (location parameter; $e^{\mathrm{logMean}}$ equals the geometric mean)
- $\mathrm{logStdDev}$: standard deviation of log values (scale parameter; controls multiplicative spread)

<!-- IMG distribution-multiplic -->

- **Formation:** the product of many positive variables $X_1 \cdot X_2 \cdot \ldots \cdot X_n$ with mild conditions (e.g., finite variance of $\log X$).
- **Origin:** historically called 'Log-Normal' or 'Galton' distribution after Francis Galton.
- **Rename Motivation:** renamed to $\Multiplic$ to reflect its formation mechanism through multiplication.
- **Properties:** logarithm of a $\Multiplic$ ('LogNormal') variable follows an $\Additive$ ('Normal') distribution.
- **Applications:** stock prices, file sizes, reaction times, income distributions, biological growth rates.
- **Caution:** no perfectly multiplic distributions exist in real data;
    all real-world measurements contain some deviations.
  Traditional estimators may struggle with the inherent skewness and heavy right tail.
