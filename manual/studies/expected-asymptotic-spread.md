## Asymptotic Gaussian Expected Value of the Spread

This study establishes that $\Spread$ has expected value $\sqrt{2}\,\Phi^{-1}(0.75) \approx 0.954$ under standard normal data as sample size increases.

The key insight is that pairwise absolute differences $|X_i - X_j|$ from normal data converge to a known distribution.
Since $\Spread$ takes the median of these differences, its asymptotic expectation equals the population median of $|X_1 - X_2|$ where $X_1, X_2 \stackrel{\mathrm{iid}}{\sim} \mathcal{N}(0, 1)$.

Consider $X_1, \ldots, X_n \stackrel{\mathrm{iid}}{\sim} \mathcal{N}(0, 1)$.
For any fixed $i \neq j$, the difference $X_i - X_j$ has mean 0 and variance

$$
\Var[X_i - X_j] = \Var[X_i] + \Var[X_j] = 1 + 1 = 2
$$

because $X_i$ and $X_j$ are independent. Therefore

$$
X_i - X_j \sim \mathcal{N}(0, 2)
$$

Define $W = (X_i - X_j)/\sqrt{2} \sim \mathcal{N}(0, 1)$ and $D = |X_i - X_j| = \sqrt{2}\,|W|$.
The cumulative distribution function of $D$ for $t \geq 0$ is

$$
\Pr[D \leq t]
  = \Pr\bigl[|W| \leq t/\sqrt{2}\bigr]
  = 2\Phi\bigl(t/\sqrt{2}\bigr) - 1
$$

where $\Phi$ denotes the standard normal cumulative distribution function.
The population median $m$ of $D$ satisfies

$$
2\Phi\bigl(m/\sqrt{2}\bigr) - 1 = \tfrac{1}{2}
$$

Solving for $m$ yields

$$
m = \sqrt{2}\,\Phi^{-1}(0.75)
$$

The multiset of gaps $\{|X_i - X_j| : i < j\}$ forms a bounded-kernel U-statistic of degree 2.
U-statistic consistency results imply that its empirical distribution converges almost surely to the law of $D$.
The sample median of pairwise differences thus converges in probability to $m$.
Convergence in probability plus uniform integrability yields convergence of expectations:

$$
\lim_{n \to \infty} \E\bigl[\Spread(X_1, \ldots, X_n)\bigr]
  = \sqrt{2}\,\Phi^{-1}(0.75)
$$

or

$$
\lim_{n \to \infty} \E\bigl[\Spread(X_1, \ldots, X_n)\bigr]
  \approx 0.953\,873
$$
