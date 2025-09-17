# Introduction

## Desiderata

The toolkit consists of statistical *procedures* —
  practical methods that transform raw measurements into actionable insights and decisions.
When practitioners face real-world problems involving data analysis,
  their success depends on selecting the right procedure for each specific situation.
Convenient and efficient procedures have the following *desired properties*:

- **Usability.**
  Procedures should feel natural to practitioners and minimize opportunities for misuse.
  They should be mathematically elegant yet accessible to readers with standard mathematical backgrounds.
  Implementation should be straightforward across programming languages.
  Like well-designed APIs, these procedures should follow intuitive design principles that reduce cognitive load.
- **Reliability.**
  Procedures should deliver consistent, trustworthy results,
    even in the presence of noise, data corruption, and extreme outliers.
- **Applicability.**
  Procedures should perform well across diverse contexts and sample sizes.
  They should handle the full spectrum of distributions commonly encountered in practice,
    from ideal theoretical models to data that deviates significantly from any assumed distribution.

This manual introduces a unified toolkit that aims to satisfy these properties and provide reliable rule-of-thumb procedures for everyday analytical tasks.

## Primer

Given two numeric samples $\x=(x_1,\ldots,x_n)$ and $\y=(y_1,\ldots,y_m)$, the toolkit provides the following primary procedures:

- $\Center(x)$ — robust average of $\x$
- $\Spread(x)$ — robust dispersion of $\x$
- $\Shift(x,y)$ — robust signed difference ($\x-\y$)
- $\Ratio(x,y)$ — robust ratio ($\x/\y$)
- $\Disparity(x,y)$ — robust effect size ($(\x-\y)$ normalized by average spread)

These procedures are designed to serve as default choices for routine analysis and comparison tasks in engineering contexts.
The toolkit has ready-to-use implementations for Python, TypeScript/JavaScript, R, .NET, Kotlin, Rust, and Go.

## Breaking changes

Statistical practice has evolved through decades of research and teaching,
  creating a system where historical naming conventions became permanently embedded in textbooks and standard practice.
Traditional statistics often names procedures after their discoverers or uses arbitrary symbols
  that reveal nothing about their actual purpose or application context.
This approach forces practitioners to memorize meaningless mappings between historical figures and mathematical concepts.

The result is unnecessary friction for anyone learning or applying statistical methods.
Beginners face an inconsistent landscape of confusing names, fragile defaults,
  and incompatible approaches with little guidance on selection or interpretation.
Modern practitioners would benefit from a more consistent system, which requires some renaming and redefining.
This manual breaks from the traditions, offering a coherent system designed for clarity and practical use.

- Renamed distributions:
  - $\Additive$ (former 'Normal' or 'Gaussian')
  - $\Multiplic$ (former 'Log-Normal' or 'Galton')
  - $\Power$ (former 'Pareto')
- Primary measure of average: $\Center$ (instead of $\Mean$)
- Primary measure of dispersion: $\Spread$ (instead of $\StdDev$)
- Primary measure of effect size: $\Disparity$ (instead of Cohen's $d$)
- Reworked statistical efficiency (see section "Drift")

## Definitions

- $X$, $Y$: random variables, can be treated as generators of random real measurements
  - $X \sim \underline{\operatorname{Distribution}}$ defines a distribution from which this variable comes
- $x_i, y_j$: specific individual measurements
- $\x = (x_1, x_2, \ldots, x_n)$, $\y = (y_1, y_2, \ldots, y_m)$: samples of measurements of given size
  - Samples are non-empty: $n, m \geq 1$
- $x_{(1)}, x_{(2)}, \ldots, x_{(n)}$: sorted measurements of the sample ('order statistics')
- Asymptotic case: the sample size goes to infinity $n, m \to \infty$
  - Can be typically treated as approximation for large samples
- $\operatorname{Estimator}(\x)$: a function that estimates the property of a distribution from given measurements
  - $\operatorname{Estimator}[X]$ shows the true property value of the distribution (asymptotic value)
- $\Median$: an estimator that finds the value splitting the distribution into two equal parts

$$
\Median(\x) = \begin{cases}
x_{((n+1)/2)} & \text{if } n \text{ is odd} \\
\frac{x_{(n/2)} + x_{(n/2+1)}}{2} & \text{if } n \text{ is even}
\end{cases}
$$
