## One-Sample Summary

Consider a sample $\x$ of $n$ real numbers: $\x = (x_1, x_2, \ldots, x_n)$.
The toolkit provides four estimators to summarize key properties of the data and provide insights into the data's primary characteristics:

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

$$
\Volatility(\x) = \frac{\Spread(\x)}{\left| \Center(\x) \right|}
$$

$$
\Precision(\x) = \frac{2 \cdot \Spread(\x)}{\sqrt{n}}
$$

One-sample summary statistics work best for unimodal distributions and distributions with low dispersion.

$\Center(\x)$[^center] estimates the central (average) value of the distribution.
For normal distributions, it matches both the mean and the median.
It outperforms traditional estimators in practical use.
Compared to $\Mean$, $\Center$ is much more robust (tolerates almost one-third of outliers).
Compared to $\Median$, $\Center$ is much more efficient and requires $1.5$ times fewer observations to achieve the
  same precision.

[^center]: Also known as the *Hodges--Lehmann* location estimator, see [@hodges1963], [@sen1963]

$\Spread(\x)$[^spread] estimates distribution dispersion (variability or scatter).
It measures the median absolute difference between two random sample elements.
This measure offers a practical alternative to standard deviation ($\StdDev$) and median absolute deviation ($\MAD$).
Compared to $\StdDev$, $\Spread$ is more robust (standard deviation breaks with a single extreme value)
  and has comparable efficiency under normality.
Compared to $\MAD$, $\Spread$ is much more efficient under normality and requires $2.35$ times fewer observations
  to achieve the same precision.

[^spread]: Also known as the *Shamos* scale estimator, see [@shamos1976]

$\Volatility(\x)$ estimates the relative dispersion of the distribution.
Convenient to express in percentage: e.g., a value of $0.2$ means $20\%$ relative to $\Center(\x)$.
$\Volatility$ is scale-invariant, which makes an experiment design more portable.

$\Precision(\x)$ estimates the distance between two $\Center$ estimations of independent random samples.
The interval $\Center(\x) \pm \Precision(\x)$ forms a range that contains the true center value with high confidence.
For even higher confidence, use
  $\Center(\x) \pm 2 \cdot \Precision(\x)$ or
  $\Center(\x) \pm 3 \cdot \Precision(\x)$. 

These estimators build on $\Median(\x)$[^median].
To find the median, $\x$ must first be arranged into a sorted sample[^sorted-sample]:
  $(x_{(1)}, \ldots, x_{(n)})$.
In this ordered sequence, $x_{(1)}$ represents the smallest value and $x_{(n)}$ the largest.
The median then becomes the middle value of this sorted sample.
When the sample size is even, the median equals the average of the two middle values:

[^median]: Also known as *sample median*
[^sorted-sample]: Also known as *order statistics*

$$
\Median(\x) = \begin{cases}
x_{\left((n+1)/2\right)} & \text{if } n \text{ is odd} \\
\frac{x_{(n/2)} + x_{(n/2+1)}}{2} & \text{if } n \text{ is even}
\end{cases}
$$