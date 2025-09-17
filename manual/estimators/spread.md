## Spread

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

- Measures dispersion (variability, scatter)
- Corner case: for $n=1$, $\Spread(\x) = 0$
- Equals the *Shamos scale estimator* ([@shamos1976]), renamed to $\Spread$ for clarity
- Pragmatic alternative to the standard deviation and the median absolute deviation
- Asymptotically, $\Spread[X]$ is the median of the absolute difference of two random measurements from $X$
- Straightforward implementations have $O(n^2 \log n)$ complexity; a fast $O(n \log n)$ version is provided in the Algorithms section.
- Domain: any real numbers
- Unit: the same as measurements

$$
\Spread(\x + k) = \Spread(\x)
$$

$$
\Spread(k \cdot \x) = |k| \cdot \Spread(\x)
$$

$$
\Spread(\x) \geq 0
$$
