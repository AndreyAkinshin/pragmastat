## Shift

$$
\Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

- Measures the median difference between elements of two samples
- Value interpretation: half of $x_i-y_j$ values are smaller than $\Shift(\x, \y)$, the other half are larger
- Sign interpretation: positive shifts indicate larger $\x$ values, negative shifts indicate larger $\y$ values
- For samples with small $\RelSpread$, $\Shift(\x, \y)$ approximates pairwise differences $x_i - y_j$
- Equals the *Hodges-Lehmann estimator* for two samples ([@hodges1963])

**Properties**

- Domain: any real numbers
- Unit: the same as measurements
- Location-invariant, scale-dependent

$$
\Shift(\x, \x) = 0
$$

$$
\Shift(\x + k_x, \y + k_y) = \Shift(\x, \y) + k_x\!-\!k_y
$$

$$
\Shift(k \cdot \x, k \cdot \y) = k \cdot \Shift(\x, \y)
$$

$$
\Shift(\x, \y) = -\Shift(\y, \x)
$$
