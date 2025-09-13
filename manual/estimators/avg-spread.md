## AvgSpread

$$
\AvgSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

**Practical Recommendations**

The $\AvgSpread$ primarily serves as a scaling factor for $\Disparity$.
It represents the combined dispersion of both samples, weighted by sample size.
Performs best for distributions with similar dispersion values.

**Key Facts**

- Measures average dispersion across two samples
- Domain: any real numbers
- Provides robust alternative to the pooled standard deviation

**Properties**

$$
\AvgSpread(\x, \x) = \Spread(\x)
$$

$$
\AvgSpread(k_1 \cdot \x, k_2 \cdot \x) = \frac{ |k_1| + |k_2| }{2} \cdot \Spread(\x)
$$

$$
\AvgSpread(\x, \y) = \AvgSpread(\y, \x)
$$

$$
\AvgSpread(k \cdot \x, k \cdot \y) = |k| \cdot \AvgSpread(\x, \y)
$$
