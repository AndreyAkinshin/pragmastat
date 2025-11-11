## Definitions

- $X$, $Y$: random variables, can be treated as generators of random real measurements
  - $X \sim \underline{\operatorname{Distribution}}$ defines a distribution from which this variable comes
- $x_i, y_j$: specific individual measurements
- $\x = (x_1, x_2, \ldots, x_n)$, $\y = (y_1, y_2, \ldots, y_m)$: samples of measurements of a given size
  - Samples are non-empty: $n, m \geq 1$
- $x_{(1)}, x_{(2)}, \ldots, x_{(n)}$: sorted measurements of the sample ('order statistics')
- Asymptotic case: the sample size goes to infinity $n, m \to \infty$
  - Can typically be treated as an approximation for large samples
- $\operatorname{Estimator}(\x)$: a function that estimates the property of a distribution from given measurements
  - $\operatorname{Estimator}[X]$ shows the true property value of the distribution (asymptotic value)
- $\Median$: an estimator that finds the value splitting the distribution into two equal parts

$$
\Median(\x) = \begin{cases}
x_{((n+1)/2)} & \text{if } n \text{ is odd} \\
\frac{x_{(n/2)} + x_{(n/2+1)}}{2} & \text{if } n \text{ is even}
\end{cases}
$$
