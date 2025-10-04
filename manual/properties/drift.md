## Drift

Drift measures estimator precision by quantifying how much estimates scatter across repeated samples.
It is based on $\Spread$ of the estimates, and therefore has a breakdown point of $\approx 29\%$.

Drift is useful when comparing precisions of several estimators.
To simplify the comparison, it is convenient to choose one of the estimators as a baseline.
A table with drift squares normalized by the baseline shows the sample size adjustment factor
  for switching from the baseline to another estimator.
For example, if $\Center$ is the baseline, and the rescaled drift square of $\Median$ is $1.5$,
  this means that $\Median$ would require $1.5$ times more data than $\Center$ to achieve the same precision.
See the "From Statistical Efficiency to Drift" section for details.

**Asymptotic Average estimator drift²** (values are approximated):


|              | $\Mean$  | $\Median$ | $\Center$ |
|--------------|----------|-----------|-----------|
| $\Additive$  | 1.0      | 1.571     | 1.047     |
| $\Multiplic$ | 3.95     | 1.40      | 1.7       |
| $\Exp$       | 1.88     | 1.88      | 1.69      |
| $\Power$     | $\infty$ | 0.9       | 2.1       |
| $\Uniform$   | 0.88     | 2.60      | 0.94      |

Rescaled to $\Center$ (sample size adjustment factors):

|              | $\Mean$  | $\Median$ | $\Center$ |
|--------------|----------|-----------|-----------|
| $\Additive$  | 0.96     | 1.50      | 1.0       |
| $\Multiplic$ | 2.32     | 0.82      | 1.0       |
| $\Exp$       | 1.11     | 1.11      | 1.0       |
| $\Power$     | $\infty$ | 0.43      | 1.0       |
| $\Uniform$   | 0.936    | 2.77      | 1.0       |

<!-- IMG avg-drift-additive -->

<!-- IMG avg-drift-multiplic -->

<!-- IMG avg-drift-exp -->

<!-- IMG avg-drift-power -->

<!-- IMG avg-drift-uniform -->

---

**Asymptotic Dispersion estimator drift²** (values are approximated):

|              | $\StdDev$ | $\MAD$ | $\Spread$ |
|--------------|-----------|--------|-----------|
| $\Additive$  | 0.45      | 1.22   | 0.52      |
| $\Multiplic$ | $\infty$  | 2.26   | 1.81      |
| $\Exp$       | 1.69      | 1.92   | 1.26      |
| $\Power$     | $\infty$  | 3.5    | 4.4       |
| $\Uniform$   | 0.18      | 0.90   | 0.43      |

Rescaled to $\Spread$ (sample size adjustment factors):

|              | $\StdDev$ | $\MAD$ | $\Spread$ |
|--------------|-----------|--------|-----------|
| $\Additive$  | 0.87      | 2.35   | 1.0       |
| $\Multiplic$ | $\infty$  | 1.25   | 1.0       |
| $\Exp$       | 1.34      | 1.52   | 1.0       |
| $\Power$     | $\infty$  | 0.80   | 1.0       |
| $\Uniform$   | 0.42      | 2.09   | 1.0       |

<!-- IMG disp-drift-additive -->

<!-- IMG disp-drift-multiplic -->

<!-- IMG disp-drift-exp -->

<!-- IMG disp-drift-power -->

<!-- IMG disp-drift-uniform -->
