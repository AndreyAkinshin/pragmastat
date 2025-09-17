## AvgSpread

$$
\AvgSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

- Measures average dispersion across two samples
- Pragmatic alternative to the 'pooled standard deviation'
- Note: $\AvgSpread(\x, \y) \neq \Spread(\x \cup \y)$ in general (defined pooled scale, not spread of concatenated sample)
- Domain: any real numbers
- Unit: the same as measurements

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
