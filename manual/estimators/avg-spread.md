## AvgSpread

$$
\AvgSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

- Measures average dispersion across two samples
- Serves as a normalization factor for $\Disparity$
- Performs best for distributions with close dispersion values

**Comparison**

- Robust alternative to the *pooled standard deviation*

**Properties**

- Domain: any real numbers
- Unit: the same as measurements
- Location-invariant, scale-invariant

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
