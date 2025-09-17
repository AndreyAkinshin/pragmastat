## Shift

$$
\Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

- Measures the median of pairwise differences between elements of two samples
- Equals the *Hodges-Lehmann estimator* for two samples ([@hodges1963])
- Asymptotically, $\Shift[X, Y]$ is the median of the difference of random measurements from $X$ and $Y$
- Domain: any real numbers
- Unit: the same as measurements

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
