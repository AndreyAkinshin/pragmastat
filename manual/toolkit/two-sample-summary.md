## Two-Sample Summary

Consider a second sample $\y$ of $m$ real numbers: $\y = (y_1, \ldots, y_m)$.
Estimators to compare $\x$ and $\y$:

$$
\MedShift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

$$
\MedRatio(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
$$

$$
\MedSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

$$
\MedDisparity(\x, \y) = \dfrac{\MedShift(\x, \y)}{\MedSpread(\x, \y)}
$$

These estimators work best for unimodal or narrow distributions, capturing the typical differences between
  $\x$ and $\y$.

$\MedShift(\x, \y)$[^medshift] estimates the median absolute difference between elements of $\x$ and $\y$.
It answers "by how much does $\x$ typically exceed $\y$?" in the original units.
The sign matters: positive means $\x$ is typically larger, negative means $\y$ is typically larger.
E.g., $\MedShift$ of $-5$ means that in $50\%$ of $(x_i, y_j)$ pairs, $y_j - x_i > 5$.

[^medshift]: Also known as the *Hodges--Lehmann shift estimator*

$\MedRatio(\x, \y)$[^medratio] estimates the median ratio of $\x$ elements to $\y$ elements.
It answers "what's the typical ratio between $\x$ and $\y$?" as a multiplier.
For example, $\MedRatio = 1.2$ means that in $50\%$ of $(x_i, y_j)$ pairs, $x_i$ is larger than $y_j$ by at least $20\%$.
Express as percentage change: $(\MedRatio - 1) \times 100\%$.
$\MedRatio$ is scale-invariant, which makes an experiment design more portable.

[^medratio]: Inspired by the *Hodges--Lehmann estimator*

$\MedSpread(\x, \y)$ estimates the averaged variability when considering both samples together.
The measure computes the weighted average of individual spreads, where larger samples contribute more.
This value primarily serves as a scaling factor for $\MedDisparity$.
It represents the typical variability in the combined data and works best for distributions with similar dispersion values.

$\MedDisparity(\x, \y)$[^meddisparity] estimates a normalized absolute difference between $\x$ and $\y$
  expressed in standardized spread units.
Negative values are treated similarly to $\MedShift(\x, \y)$.
$\MedDisparity$ is scale-invariant, which makes an experiment design more portable.

[^meddisparity]: A robust alternative to traditional effect size measures like Cohen's $d$