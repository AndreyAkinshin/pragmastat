#import "/manual/definitions.typ": *

== Additive ('Normal')

$ Additive(pmean, pstddev) $

- $pmean$: location parameter (center of the distribution), consistent with $Center$
- $pstddev$: scale parameter (standard deviation), can be rescaled to $Spread$

#image("/img/distribution-additive_light.png")

- *Formation:* the sum of many variables $X_1 + X_2 + ... + X_n$ under mild CLT (Central Limit Theorem) conditions (e.g., Lindeberg-Feller).
- *Origin:* historically called 'Normal' or 'Gaussian' distribution after Carl Friedrich Gauss and others.
- *Rename Motivation:* renamed to $Additive$ to reflect its formation mechanism through addition.
- *Properties:* symmetric, bell-shaped, characterized by central limit theorem convergence.
- *Applications:* measurement errors, heights and weights in populations, test scores, temperature variations.
- *Characteristics:* symmetric around the mean, light tails, finite variance.
- *Caution:* no perfectly additive distributions exist in real data;
    all real-world measurements contain some deviations.
  Traditional estimators like $Mean$ and $StdDev$ lack robustness to outliers;
  use them only when strong evidence supports small deviations from additivity with no extreme measurements.
