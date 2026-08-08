#import "/manual/definitions.typ": *

== Additive ('Normal') <sec-additive>

$ Additive(pmean, pstddev) $

- $pmean$: location parameter (center of the distribution), consistent with $Center$
- $pstddev$: scale parameter (standard deviation), can be rescaled to $Spread$

#image("/img/distribution-additive_light.png")

- *Formation:* the sum of many variables $X_1 + X_2 + ... + X_n$ under mild CLT (Central Limit Theorem) conditions (e.g., Lindeberg--Feller).
- *Origin:* historically called 'Normal' or 'Gaussian' distribution after Carl Friedrich Gauss and others.
- *Rename Motivation:* renamed to $Additive$ to reflect its formation mechanism through addition.
- *Properties:* symmetric, bell-shaped, characterized by central limit theorem convergence.
- *Applications:* measurement errors, heights and weights in populations, test scores, temperature variations.
- *Characteristics:* symmetric around the mean, light tails, finite variance.
- *Caution:* no perfectly additive distributions exist in real data;
    all real-world measurements contain deviations.
  Traditional estimators like $Mean$ and $StdDev$ lack robustness to outliers;
  use them only when strong evidence supports approximate additivity with no extreme measurements.

=== Notes

#include "additive-notes.typ"

All three functions are assembled from the toolkit's own primitives rather than from the platform's, so the
  seven implementations return identical bits, and the `dist-additive` suite compares them as raw binary64
  payloads rather than against a tolerance.

On the whole real line, with $z = (v - pmean) \/ pstddev$:

$ Density(v) = ExpFunction(-z^2 \/ 2) / (pstddev dot sqrt(2 pi)) $
$ Cumulative(v) = AdditiveCumulative(z) $
$ Quantile(c) = pmean + pstddev dot AdditiveQuantile(c) $

This family is what #link(<sec-additive-quantile>)[$AdditiveQuantile$] was built for.
The inverse used to be two Newton steps taken against an error function accurate to
  $1.5 dot 10^(-7)$, which left the quantile eight orders of magnitude coarser than the cumulative
  standing beside it.
The support is unbounded, so $Quantile$ returns the infinities at the ends of $(0, 1)$.

=== Tests

#include "additive-tests.typ"
