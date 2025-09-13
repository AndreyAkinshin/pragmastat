## RelSpread

$$
\RelSpread(\x) = \frac{\Spread(\x)}{\left| \Center(\x) \right|}
$$

**Practical Recommendations**

The $\RelSpread$ provides scale-invariant insight into the distribution dispersion normalized by the center value.

Interpretation examples:

- $\RelSpread(\x) = 1\%$: data clusters tightly around $\Center(\x)$ with minimal variation
- $\RelSpread(\x) = 10\%$: moderate variation, typical values range from $90\%$ to $110\%$ of center
- $\RelSpread(\x) = 100\%$: high variation, values span from near zero to twice the center

**Key Facts**

- Measures the relative dispersion of a sample to $\Center(\x)$
- Domain:
  - Mathematical Domain: $\Center(\x) \neq 0$
  - Logical Domain: all sample elements have the same sign, sample doesn't contain zeros
  - Pragmatic Domain: non-negative values allowing up to $29\%$ zeros
- Robust alternative to the *coefficient of variation*
- Scale-invariant, which makes experimental design more portable

**Properties**

$$
\RelSpread(k \cdot \x) = \RelSpread(\x)
$$

$$
\RelSpread(\x) \geq 0
$$
