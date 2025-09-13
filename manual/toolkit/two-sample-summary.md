## Two-Sample Summary

The $\Shift$ expresses the median difference between two random variables:

$$
\Shift(X, Y) = \Median(X - Y)
$$

When taking measurements from $X$ and $Y$,
  the difference falls below $\Shift(X, Y)$ with $50\%$ probability and above it with $50\%$ probability.
Identical distributions produce zero shift: $\Shift(X, X) = 0$.

The estimator for the shift[^shift] is defined as follows:

[^shift]: Also known as the *Hodges--Lehmann shift estimator*

$$
\boxed{
  \Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
}
$$

This estimator calculates the median difference between elements of $\x$ and $\y$.
Sign interpretation proves critical: positive shifts indicate larger $\x$ values, negative shifts indicate larger $\y$ values.
Small values of $\Spread(X)$ and $\Spread(Y)$ make $\Shift(\x, \y)$ closely approximate the difference between any $x_i$ and $y_j$.

---

Absolute units for expressing differences between $X$ and $Y$ create problems and reduce experimental portability.
Relative measures often provide better solutions:

$$
\Ratio(X, Y) = \Median\left( \frac{X}{Y} \right)
$$

The $\Ratio(X, Y)$ captures the median ratio between random measurements from $X$ and $Y$:

$$
\boxed{
  \Ratio(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
}
$$

This estimator $\Ratio(\x, \y)$[^ratio] calculates the median ratio of $\x$ elements to $\y$ elements.
For example, $\Ratio = 1.2$ indicates that the median ratio is $1.2$,
  meaning $50\%$ of $(x_i, y_j)$ pairs have ratios below $1.2$ and $50\%$ above $1.2$.
Percentage expression follows $(\Ratio - 1) \times 100\%$.
Scale invariance makes ratio estimation useful for portable experimental designs.

[^ratio]: Inspired by the *Hodges--Lehmann estimator*

---

Neither shift nor ratio works universally well.
Portable experimental design benefits from expressing absolute differences in scale-invariant units.
This requirement introduces *disparity*:

$$
\Disparity(X, Y) = \frac{\Median(X - Y)}{\AvgSpread(X, Y)}
$$

where

$$
\AvgSpread(X, Y) = \frac{\Spread(X) + \Spread(Y)}{2}
$$

The $\Disparity(X, Y)$ normalizes the shift by spread, expressing differences in scale-invariant spread units.


$$
\boxed{
  \Disparity(\x, \y) = \dfrac{\Shift(\x, \y)}{\AvgSpread(\x, \y)}
}
$$

where

$$
\AvgSpread(\x, \y) = \dfrac{n \cdot \Spread(\x) + m \cdot \Spread(\y)}{n + m}
$$
