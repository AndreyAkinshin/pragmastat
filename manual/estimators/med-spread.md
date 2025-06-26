## MedSpread

$$
\MedSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

**Practical Recommendations**

$\MedSpread$ primarily serves as a scaling factor for $\MedDisparity$.
It represents the combined dispersion of both samples, weighted by sample size.
Works best for distributions with similar dispersion values.

**Key Facts**

- Measures average dispersion across two samples
- Domain: any real numbers
- Provides a robust alternative to the pooled standard deviation

**Properties**

$$
\MedSpread(\x, \x) = \Spread(\x)
$$

$$
\MedSpread(k_1 \cdot \x, k_2 \cdot \x) = \frac{ |k_1| + |k_2| }{2} \cdot \Spread(\x)
$$

$$
\MedSpread(\x, \y) = \MedSpread(\y, \x)
$$

$$
\MedSpread(k \cdot \x, k \cdot \y) = |k| \cdot \MedSpread(\x, \y)
$$
