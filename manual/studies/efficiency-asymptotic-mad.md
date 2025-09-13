## Asymptotic Gaussian Efficiency of the Median Absolute Deviation

This study shows that the median absolute deviation (MAD) achieves only about $37\%$ efficiency
  relative to the sample standard deviation under normality.

The analysis derives the asymptotic distribution of the MAD under Gaussian data using classical theory for sample medians.
Consider

$$
X_1, \ldots, X_n \stackrel{\mathrm{iid}}{\sim} \mathcal{N}(\mu, \sigma^2), \qquad n \geq 2,\; \sigma > 0
$$  

Since both the sample median and the MAD are translation invariant, set $\mu = 0$ without loss of generality and write

$$
\MAD_n = \Median\Bigl(|X_i - \Median(\x)| : i = 1, \ldots, n\Bigr)
$$

For a standard normal distribution, the population MAD equals

$$
m_0 = \Phi^{-1}(0.75)\,\sigma = c_0\,\sigma
$$

$$
c_0 = \Phi^{-1}(0.75) \approx 0.674
$$

Dividing the empirical MAD by this constant gives a consistent scale estimator:

$$
\widehat{\sigma}_{\MAD} = \frac{\MAD_n}{c_0}
$$

To find the large-sample variance, observe that $Y_i = |X_i|$ has density

$$
g(y) = \frac{2}{\sigma\sqrt{2\pi}}
           \exp\Bigl(-\frac{y^2}{2\sigma^2}\Bigr), \qquad y \geq 0
$$  

whose median is $m_0$.
A classical result for sample medians of independent draws with continuous positive density at the
  median ([@sidak1999], [@serfling2009]) states  

$$
\sqrt{n}\bigl(\Median(Y_1, \ldots, Y_n) - m_0\bigr)
  \xrightarrow{d}
  \mathcal{N}\Bigl(0, \tfrac{1}{4g(m_0)^2}\Bigr)
$$

Since

$$
g(m_0) = \frac{2}{\sigma\sqrt{2\pi}}\,
             \exp\Bigl(-\frac{c_0^2}{2}\Bigr)
$$

the asymptotic variance of $\sqrt{n}\,\MAD_n$ is

$$
\Var\bigl[\sqrt{n}\,\MAD_n\bigr]
  = \frac{1}{4g(m_0)^2}
  = \frac{\pi\sigma^2\exp(c_0^2)}{8}
$$

Scaling by $1/c_0$ gives the variance of the consistent estimator:

$$
\Var\bigl[\sqrt{n}\,\widehat{\sigma}_{\MAD}\bigr]
  = \frac{\pi\sigma^2\exp(c_0^2)}{8\,c_0^2}
$$

The optimal Gaussian scale estimator is the sample standard deviation.
It has asymptotic variance of $\sigma^2/(2n)$.
The *asymptotic Gaussian efficiency* of the (scaled) MAD is therefore  

$$
\eff_{\mathcal{N},\infty}(\MAD)
  = \frac{\sigma^2/(2n)}{\pi\sigma^2\exp(c_0^2)/(8c_0^2n)}
  = \frac{4c_0^2}{\pi\,\exp(c_0^2)}
$$  

This gives the result:

$$
\eff_{\mathcal{N},\infty}(\MAD) \approx 0.367\,523
$$

Achieving the same precision as the sample standard deviation under normality
  requires $1/0.368 \approx 2.7$ times as many observations when using the MAD.
