## Introduction

This toolkit analyzes random variables $X$ and $Y$.
Each random variable produces random real numbers from its underlying distribution.
Consider a set of $n$ random values $\X$ and a set of $m$ random values $\Y$:

$$
\X = (X_1, X_2, \ldots, X_n)
$$

$$
\Y = (Y_1, Y_2, \ldots, Y_m)
$$

These variables produce *measurements* (the actual observed values).
For random values $\X$, the measurements are $x_1, x_2, \ldots, x_n$.
This collection of measurements forms a *sample* $\x$:

$$
\x = (x_1, x_2, \ldots, x_n)
$$

$$
\y = (y_1, y_2, \ldots, y_m)
$$

The objective is to learn about the original random variables $X$ and $Y$ from these samples.

The *median* represents a fundamental property of any distribution.
This value divides the distribution into two equal parts,
  creating exactly $50\%$ probability of observing a measurement below the median and $50\%$ above it.
The *true* median value $\Median(X)$ remains unknown, but the measurements provide an estimate.

To calculate the median, arrange the sample elements into an ordered sequence $x_{(1)}, \ldots, x_{(n)}$.
This *sorted sample*[^sorted-sample] places $x_{(1)}$ as the smallest value and $x_{(n)}$ as the largest.
The median estimate uses the middle value of this sorted sample.
For even sample sizes, the median equals the average of the two middle values:

[^sorted-sample]: Also known as *order statistics*

$$
\boxed{
\Median(\x) = \begin{cases}
x_{\left((n+1)/2\right)} & \text{if } n \text{ is odd} \\
\frac{x_{(n/2)} + x_{(n/2+1)}}{2} & \text{if } n \text{ is even}
\end{cases}
}
$$

This estimation $\Median(\x)$ approximates the true distribution value $\Median(X)$.

The median represents one example of a *quantile*.
For probability $p \in [0, 1]$,
  the $p^\textrm{th}$ quantile $\Quantile(X, p)$ divides the distribution
  so that the probability of obtaining a measurement below the quantile equals exactly $p$:

$$
\Pr[X \leq \Quantile(X, p)] = p
$$

Linear interpolation between two sample elements produces quantile estimates[^quantile-hf]:

$$
\boxed{
  \begin{array}{c}
  \Quantile(\x, p) = x_{(\lfloor h \rfloor)} + (h - \lfloor h \rfloor) \cdot (x_{(\lceil h \rceil)} - x_{(\lfloor h \rfloor)}) \\
  h = (n-1)p+1
  \end{array}
}
$$

[^quantile-hf]: Definition uses the Hyndman-Fan Type 7 quantile estimator, see [@hyndman1996].

The median equals $\Quantile(\x, 0.5)$ since it divides the distribution into equal halves.
Median estimation produces higher accuracy than other quantile values.
Estimation accuracy decreases as $p$ approaches $0$ or $1$.
Larger sample sizes improve quantile estimation accuracy for any value of $p$.

These quantile estimators form the building blocks for more complex distributional properties.

The following sections examine one-sample and two-sample estimators that provide practical insights about data.
Estimator evaluation focuses on these key properties:

* *Gaussian Efficiency*: performance quality under normal distribution conditions.
  High efficiency produces accurate estimates with fewer measurements under normality.
* *Robustness*: stability when samples contain extreme values (outliers).
  High robustness maintains accuracy even with contaminated data.
