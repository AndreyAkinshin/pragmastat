## Asymptotic Gaussian Efficiency of the Median

This study shows that the sample median achieves only $2/\pi \approx 63.7\%$ efficiency relative to the sample mean under normality.

The analysis applies classical asymptotic theory for sample quantiles to derive the limiting distribution of the median under Gaussian data.
Consider

$$
X_1, \ldots, X_n \stackrel{\mathrm{iid}}{\sim} \mathcal{N}(\mu, \sigma^2), \qquad n \geq 2,\; \sigma > 0
$$

Since the sample median is translation invariant, set $\mu = 0$ without loss of generality.
Write

$$
M_n = \Median(X_1, \ldots, X_n)
$$

For any continuous distribution with density $f$ positive at its median $\theta$, classical theory ([@sidak1999], [@serfling2009]) gives

$$
\sqrt{n}\,(M_n - \theta) \xrightarrow{d}
\mathcal{N}\Bigl(0, \frac{1}{4f(\theta)^2}\Bigr)
$$  

In the normal case $\theta = 0$ and

$$
f(0) = \frac{1}{\sigma\sqrt{2\pi}}
$$

so the asymptotic variance becomes

$$
\frac{1}{4f(0)^2}
  = \frac{1}{4}\,\bigl(\sigma\sqrt{2\pi}\bigr)^2
  = \frac{\pi\sigma^2}{2}
$$

Hence

$$
\sqrt{n}\,M_n \xrightarrow{d}
\mathcal{N}\Bigl(0, \frac{\pi\sigma^2}{2}\Bigr),
\qquad n \to \infty
$$

The sample mean has asymptotic variance $\sigma^2/n$, so the *asymptotic Gaussian efficiency* of the median is

$$
\eff_{\mathcal{N},\infty}(\Median)
  = \frac{\sigma^2/n}{\pi\sigma^2/(2n)}
  = \frac{2}{\pi}
  \approx 0.637
$$

Thus achieving the same precision as the mean under normality requires roughly $1/0.637 \approx 1.57$ times as many observations when using the median.
This large efficiency loss shows why we prefer the Hodges–Lehmann $\Center$ estimator — which attains about $95.5\%$ efficiency — whenever data are roughly Gaussian but may include outliers.

