## Asymptotic Gaussian Efficiency of the Center

This study shows that $\Center$ achieves $3/\pi \approx 95.5\%$ efficiency relative to the sample mean under normality.
This high efficiency allows $\Center$ to handle outliers while maintaining near-optimal performance on normal data.

The analysis uses U-statistic theory to derive the asymptotic distribution of $\Center$ under Gaussian data.
Let

$$
X_1, \ldots, X_n \stackrel{\mathrm{iid}}{\sim} \mathcal{N}(\mu, \sigma^2), \qquad n \geq 2,\; \sigma > 0
$$

$\Center(\x)$ has translation invariance, so setting $\mu = 0$ preserves generality.
The Walsh averages are

$$
W_{ij} = \frac{X_i + X_j}{2}, \qquad 1 \leq i \leq j \leq n
$$

Since $X_i + X_j \sim \mathcal{N}(0, 2\sigma^2)$, it follows that

$$
W_{ij} \sim \mathcal{N}\bigl(0, \sigma^2/2\bigr), \qquad 
f_W(0) = \frac{1}{\sigma\sqrt{\pi}}
$$

The estimator $\Center(\x)$ equals the sample median of the $\binom{n+1}{2}$ Walsh averages.
As a U-quantile of degree two, it satisfies

$$
0 = \frac{2}{n(n+1)}\sum_{1 \leq i \leq j \leq n}
     \Bigl\{\mathbf{1}\{W_{ij} \leq \Center(\x)\} - \tfrac{1}{2}\Bigr\}
$$

U-quantile theory [@sen1963] provides the linear expansion

$$
\sqrt{n}\,\Center(\x) = \frac{2}{f_W(0)}
\frac{1}{\sqrt{n}}\sum_{i=1}^{n}\psi(X_i) + o_{\Pr}(1)
$$

where

$$
\psi(x) = \Pr\Bigl\{\tfrac{x+X_2}{2} \leq 0\Bigr\} - \tfrac{1}{2}
  = \tfrac{1}{2} - \Phi\Bigl(\tfrac{x}{\sigma}\Bigr)
$$  

and $\Phi$ is the standard normal cumulative distribution function.
Since $X_1/\sigma \sim \mathcal{N}(0, 1)$,

$$
\Var\bigl[\psi(X_1)\bigr] = \int_{-\infty}^{\infty} \Bigl(\tfrac{1}{2} - \Phi(u)\Bigr)^2\varphi(u)\,du
  = \frac{1}{12}
$$  

with $\varphi$ the standard normal probability density function.
Substituting $\Var[\psi(X_1)]$ and $f_W(0)$ yields

$$
\Var\bigl[\sqrt{n}\,\Center(\x)\bigr]
  = \frac{4 \cdot \frac{1}{12}}{\bigl(1/(\sigma\sqrt{\pi})\bigr)^2}
  = \frac{\pi\sigma^2}{3}
$$  

so

$$
\sqrt{n}\,\Center(\x) \xrightarrow{d}
\mathcal{N}\Bigl(0, \tfrac{\pi\sigma^2}{3}\Bigr),
\qquad n \to \infty
$$

The sample mean has asymptotic variance $\sigma^2/n$, hence the *asymptotic Gaussian efficiency* of $\Center$ is

$$
\eff_{\mathcal{N},\infty}(\Center)
  = \frac{\sigma^2/n}{\pi\sigma^2/(3n)}
  = \frac{3}{\pi}
  \approx 0.954\,930
$$  

Thus matching the mean’s precision under normality requires only $1/0.955\approx1.05$ times as many observations, a negligible price for the Center’s $29\%$ breakdown point and much stronger resistance to outliers.

