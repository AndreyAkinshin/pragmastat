# Introduction

## Primer

Given two numeric samples $\x = (x_1, \ldots, x_n)$ and $\y = (y_1, \ldots, y_m)$, the toolkit provides the following primary procedures:

$\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left((x_i + x_j)/2 \right)$ — robust average of $\x$

For $\x = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Center(\x) &= 4 \\
\Center(\x + 10) &= 14 \\
\Center(3\x) &= 12
\end{aligned}
$$

$\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|$ — robust dispersion of $\x$

For $\x = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Spread(\x) &= 4 \\
\Spread(\x + 10) &= 4 \\
\Spread(2\x) &= 8
\end{aligned}
$$

$\RelSpread(\x) = \Spread(\x) / \left| \Center(\x) \right|$ — robust relative dispersion of $\x$

For $\x = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\RelSpread(\x) &= 1 \\
\RelSpread(5\x) &= 1
\end{aligned}
$$

$\Shift(\x, \y) = \underset{1 \leq i \leq n, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)$ — robust signed difference ($\x-\y$)

For $\x = (0, 2, 4, 6, 8)$ and $\y = (10, 12, 14, 16, 18)$:

$$
\begin{aligned}
\Shift(\x, \y) &= -10 \\
\Shift(\x, \x) &= 0 \\
\Shift(\x + 7, \y + 3) &= -6 \\
\Shift(2\x, 2\y) &= -20 \\
\Shift(\y, \x) &= 10
\end{aligned}
$$

$\Ratio(\x, \y) = \underset{1 \leq i \leq n, 1 \leq j \leq m}{\Median} \left( x_i / y_j \right)$ — robust ratio ($\x/\y$)

For $\x = (1, 2, 4, 8, 16)$ and $\y = (2, 4, 8, 16, 32)$:

$$
\begin{aligned}
\Ratio(\x, \y) &= 0.5 \\
\Ratio(\x, \x) &= 1 \\
\Ratio(2\x, 5\y) &= 0.2
\end{aligned}
$$

$\AvgSpread(\x, \y) = (n\Spread(\x) + m\Spread(\y)) / (n + m)$ — robust average spread of $\x$ and $\y$

For $\x = (0, 3, 6, 9, 12)$ and $\y = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Spread(\x) &= 6 \\
\Spread(\y) &= 4 \\
\AvgSpread(\x, \y) &= 5 \\
\AvgSpread(\x, \x) &= 6 \\
\AvgSpread(2\x, 3\x) &= 15 \\
\AvgSpread(\y, \x) &= 5 \\
\AvgSpread(2\x, 2\y) &= 10
\end{aligned}
$$

$\Disparity(\x, \y) = \Shift(\x, \y) / \AvgSpread(\x, \y)$ — robust effect size between $\x$ and $\y$

For $\x = (0, 3, 6, 9, 12)$ and $\y = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Shift(\x, \y) &= 2 \\
\AvgSpread(\x, \y) &= 5 \\
\Disparity(\x, \y) &= 0.4 \\
\Disparity(\x + 5, \y + 5) &= 0.4 \\
\Disparity(2\x, 2\y) &= 0.4 \\
\Disparity(\y, \x) &= -0.4
\end{aligned}
$$

These procedures are designed to serve as default choices for routine analysis and comparison tasks in engineering contexts.
The toolkit has ready-to-use implementations for Python, TypeScript/JavaScript, R, C#, Kotlin, Rust, and Go.

## Breaking changes

Statistical practice has evolved through decades of research and teaching,
  creating a system where historical naming conventions became embedded in textbooks and standard practice.
Traditional statistics often names procedures after their discoverers or uses arbitrary symbols
  that reveal nothing about their actual purpose or application context.
This approach forces practitioners to memorize meaningless mappings between historical figures and mathematical concepts.

The result is unnecessary friction for anyone learning or applying statistical methods.
Beginners face an inconsistent landscape of confusing names, fragile defaults,
  and incompatible approaches with little guidance on selection or interpretation.
Modern practitioners would benefit from a more consistent system, which requires some renaming and redefining.
This manual offers a coherent system designed for clarity and practical use, breaking from tradition.
The following concepts were adopted from traditional textbooks via renaming or reworking:

- Estimators
  - Average: $\Center$ (former 'Hodges-Lehmann location estimator')
  - Dispersion: $\Spread$ (former 'Shamos scale estimator')
  - Effect Size: $\Disparity$ (a robust alternative to 'Cohen's $d$')
- Estimator properties
  - Precision: $\Drift$ (a robust alternative to statistical efficiency)
- Distributions
  - $\Additive$ (former 'Normal' or 'Gaussian')
  - $\Multiplic$ (former 'Log-Normal' or 'Galton')
  - $\Power$ (former 'Pareto')

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
