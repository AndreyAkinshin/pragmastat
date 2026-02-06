#import "/manual/definitions.typ": *

== CenterBounds

$ CenterBounds(vx, misrate) = [w_((k_"left")), w_((k_"right"))] $

where $vw = { (x_i + x_j) \/ 2 }$ (pairwise averages, sorted) for $i <= j$,
$k_"left" = floor(SignedRankMargin \/ 2) + 1$,
$k_"right" = N - floor(SignedRankMargin \/ 2)$, and $N = n(n+1)\/2$

Robust bounds on $Center(vx)$ with specified coverage.

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — Wilcoxon signed-rank confidence interval for Hodges-Lehmann pseudomedian],
  [*Interpretation* — $misrate$ is probability that true center falls outside bounds],
  [*Domain* — any real numbers, $n >= 2$, $misrate >= 2^(1-n)$],
  [*Unit* — same as measurements],
  [*Note* — assumes weak symmetry and weak continuity; exact for $n <= 63$, Edgeworth approximation for $n > 63$],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift equivariance* #h(2em) $CenterBounds(vx + k, misrate) = CenterBounds(vx, misrate) + k$],
  [*Scale equivariance* #h(2em) $CenterBounds(k dot vx, misrate) = k dot CenterBounds(vx, misrate)$],
)

#v(0.3em)
*Example*

- `CenterBounds([1..10], 0.01) = [2.5, 8.5]` where `Center = 5.5`
- Bounds fail to cover true center with probability $approx misrate$

#v(0.5em)
$CenterBounds$ provides not just the estimated center but also the uncertainty of that estimate.
The function returns an interval of plausible center values given the data.
Set $misrate$ to control how often the bounds might fail to contain the true center:
use $10^(-6)$ for critical decisions where errors are costly, or $10^(-3)$ for everyday analysis.
These bounds require weak symmetry but no specific distributional form.
If the bounds exclude some reference value, that suggests the true center differs reliably from that value.
