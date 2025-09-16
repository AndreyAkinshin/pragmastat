## RelSpread

$$
\RelSpread(\x) = \frac{\Spread(\x)}{\left| \Center(\x) \right|}
$$

- Measures the relative dispersion of a sample to $\Center(\x)$
- Interpretation examples:
  - $\RelSpread(\x) = 0.01$: data clusters tightly around $\Center(\x)$ with minimal variation
  - $\RelSpread(\x) = 0.1$: moderate variation, typical values range from $90\%$ to $110\%$ of center
  - $\RelSpread(\x) = 1.0$: high variation, values span from near zero to twice the center

**Comparison**

- Robust alternative to the *coefficient of variation*

**Properties**

- Mathematical Domain: $\Center(\x) \neq 0$
- Pragmatic Domain: non-negative values allowing up to $29\%$ zeros
- Unit: relative (dimensionless)
- Location-dependent, scale-invariant

$$
\RelSpread(k \cdot \x) = \RelSpread(\x)
$$

$$
\RelSpread(\x) \geq 0
$$
