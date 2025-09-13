## One-Sample Summary

$$
\boxed{
  \Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
}
$$

The *center*[^center] provides the primary insight into any sample (consistent with the median for symmetric distributions).
This estimator excels in problems requiring central tendency measures.

Compared to the median, the center produces higher efficiency under normality and
  requires $1.5$ times fewer observations for equivalent precision.
Compared to the arithmetic mean, the center offers greater robustness (tolerates nearly one-third outliers)
  while maintaining comparable efficiency under normality.

[^center]: Also known as the *Hodges--Lehmann* location estimator or pseudomedian, see [@hodges1963], [@sen1963]

---

$$
\boxed{
  \Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
}
$$

The *spread*[^spread] provides the second key insight into sample characteristics by measuring dispersion (variability or scatter).
This estimator $\Spread(\x)$ calculates the median absolute difference between all pairs of measurements.
Under normal distribution conditions, multiplication by a constant converts spread to other dispersion measures.

Compared to standard deviation, spread provides greater robustness (tolerates nearly one-third outliers)
  with comparable efficiency under normality.
Compared to median absolute deviation, spread produces higher efficiency under normality and
  requires $2.35$ times fewer observations for equivalent precision.

[^spread]: Also known as the *Shamos* scale estimator, see [@shamos1976]

---

$$
\boxed{
  \RelSpread(\x) = \frac{\Spread(\x)}{\left| \Center(\x) \right|}
}
$$

The relative spread $\RelSpread(\x)$ measures dispersion normalized by the center value.
Percentage expression proves convenient: $0.2$ represents $20\%$ relative to $\Center(\x)$.
Scale invariance makes $\RelSpread$ useful for portable experimental designs.
