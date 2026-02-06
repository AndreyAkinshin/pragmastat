#import "/manual/definitions.typ": *

== MedianBounds

$ MedianBounds(vx, misrate) = [x_((k)), x_((n-k+1))] $

where $k$ is the largest integer satisfying $2 dot Pr(B <= k-1) <= misrate$
and $B tilde "Binomial"(n, 0.5)$

Robust bounds on $Median(vx)$ with specified coverage.

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — sign test confidence interval for the median],
  [*Interpretation* — $misrate$ is probability that true median falls outside bounds],
  [*Domain* — any real numbers, $n >= 2$, $misrate >= 2^(1-n)$],
  [*Unit* — same as measurements],
  [*Note* — assumes weak continuity only (no symmetry required); exact for all $n$ using binomial distribution],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift equivariance* #h(2em) $MedianBounds(vx + k, misrate) = MedianBounds(vx, misrate) + k$],
  [*Scale equivariance* #h(2em) $MedianBounds(k dot vx, misrate) = k dot MedianBounds(vx, misrate)$],
)

#v(0.3em)
*Example*

- `MedianBounds([1..10], 0.1) = [2, 9]` where `Median = 5.5`
- Bounds fail to cover true median with probability $approx misrate$

#v(0.5em)
$MedianBounds$ provides not just the estimated median but also the uncertainty of that estimate.
The function returns an interval of plausible median values given the data.
Set $misrate$ to control how often the bounds might fail to contain the true median:
use $10^(-6)$ for critical decisions where errors are costly, or $10^(-3)$ for everyday analysis.
These bounds require no symmetry assumption, only weak continuity.
If the bounds exclude some reference value, that suggests the true median differs reliably from that value.
