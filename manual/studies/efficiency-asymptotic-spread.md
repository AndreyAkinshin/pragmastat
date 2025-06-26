## Asymptotic Gaussian Efficiency of the Spread

This study shows that $\Spread$ achieves approximately $86\%$ efficiency relative to the sample standard deviation under normality.

The analysis uses U-statistic theory to derive the asymptotic distribution of $\Spread$ as a scale estimator under Gaussian data.
Consider

$$
X_1, \ldots, X_n \stackrel{\mathrm{iid}}{\sim} \mathcal{N}(\mu, \sigma^2), \qquad n \geq 2,\; \sigma > 0
$$

Since $\Spread(\x) = \Median |X_i - X_j|$ is translation invariant, set $\mu = 0$ without loss of generality and write

$$
R_n = \Spread(\x)
$$

$$
m_0 = \sqrt{2}\,\Phi^{-1}(0.75)\,\sigma \approx 0.954\,\sigma
$$  

Here $m_0$ denotes the population median of $|X_i - X_j|$ when $X_i \sim \mathcal{N}(0, \sigma^2)$.
Letting $D = |X_1 - X_2|$, its density is  

$$
f_D(t) = \frac{1}{\sigma\sqrt{\pi}}\,
        \exp\Bigl(-\frac{t^2}{4\sigma^2}\Bigr), \qquad t \geq 0
$$  

with $f_D(m_0) = \sigma^{-1}\pi^{-1/2}\exp(-\Phi^{-1}(0.75)^2/2)$.
Treating $R_n$ as a degree-two U-quantile and applying asymptotic theory [@sen1963] yields  

$$
\sqrt{n}\,\bigl(R_n - m_0\bigr)
  \xrightarrow{d}
  \mathcal{N}\Bigl(0, \frac{4\,\Var[\psi(X_1)]}{f_D(m_0)^2}\Bigr)
$$  

where

$$
\begin{aligned}
\psi(x) &= \Pr\bigl\{|x - X_2| \leq m_0\bigr\} - \tfrac{1}{2}\\
        &= \Phi\Bigl(\tfrac{x + m_0}{\sigma}\Bigr)
         - \Phi\Bigl(\tfrac{x - m_0}{\sigma}\Bigr)
         - \tfrac{1}{2}
\end{aligned}
$$  

Numerical evaluation of the integral

$$
\Var[\psi(X_1)]
  = \int_{-\infty}^{\infty}\psi(x)^2\,
    \frac{e^{-x^2/(2\sigma^2)}}{\sigma\sqrt{2\pi}}\;dx
$$

yields $\Var[\psi(X_1)] \approx 0.0266$.
Substituting this value and $f_D(m_0)$ into the variance formula gives  

$$
\Var\bigl[\sqrt{n}\,R_n\bigr]
  \approx 0.527\,\sigma^2
$$  

Because $R_n$ is *not* a consistent estimator of $\sigma$, comparisons with the sample standard deviation $\StdDev(\x)$ use the rescaled statistic

$$
\widehat{\sigma}_{\Spread} = \frac{R_n}{m_0}
$$

which *is* consistent.
Dividing the variance above by the constant $m_0^2$ gives  

$$
\Var\bigl[\sqrt{n}\,\widehat{\sigma}_{\Spread}\bigr]
  \approx 0.579\,\sigma^2
$$  

The optimal Gaussian scale estimator $\StdDev(\x)$ has asymptotic variance $\sigma^2/(2n)$, so the *asymptotic Gaussian efficiency* of the (scaled) $\Spread$ is  

$$
\eff_{\mathcal{N},\infty}(\Spread)
  = \frac{\sigma^2/(2n)}{0.579\,\sigma^2/n}
  \approx 0.864
$$  

One needs roughly $1/0.864 \approx 1.16$ times as many observations to match the precision of the sample standard deviation when the data are exactly normal.
In exchange, $\Spread$ inherits a $29\%$ breakdown point from its U-quantile construction, so moderate extra data provide a substantial increase in robustness.

