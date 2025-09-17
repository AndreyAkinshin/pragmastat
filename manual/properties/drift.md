## Drift

Drift measures estimator precision by quantifying how much estimates scatter across repeated samples.
Unlike traditional statistical efficiency that fails for heavy-tailed distributions,
  drift uses the robust $\Spread$ measure and works reliably across most distribution types.

Drift values directly indicate relative data requirements.
When switching between estimators, the required sample size changes by the square of the drift ratio.
If estimator A has drift 1.5 and estimator B has drift 1.0, then A requires $(1.5)^2 = 2.25$ times
  more data to match B's precision.
See the "From Statistical Efficiency to Drift" section in the methodology chapter for complete details.

**Asymptotic Average estimator drift²** (values are approximated; should be double-checked):


|              | $\Mean$  | $\Median$ | $\Center$ |
|--------------|----------|-----------|-----------|
| $\Additive$  | 1.0      | 1.571     | 1.047     |
| $\Multiplic$ | 3.95     | 1.40      | 1.7       |
| $\Exp$       | 1.88     | 1.88      | 1.69      |
| $\Power$     | $\infty$ | 0.9       | 2.1       |
| $\Uniform$   | 0.88     | 2.60      | 0.94      |

Rescaled to $\Center$ (sample-size factors):

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

**Asymptotic Dispersion estimator drift²** (values are approximated; should be double-checked):

|              | $\StdDev$ | $\MAD$ | $\Spread$ |
|--------------|-----------|--------|-----------|
| $\Additive$  | 0.45      | 1.22   | 0.52      |
| $\Multiplic$ | $\infty$  | 2.26   | 1.81      |
| $\Exp$       | 1.69      | 1.92   | 1.26      |
| $\Power$     | $\infty$  | 3.5    | 4.4       |
| $\Uniform$   | 0.18      | 0.90   | 0.43      |

Rescaled to $\Spread$ (sample-size factors):

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
