#import "/manual/definitions.typ": *

== CenterBoundsApprox

$ CenterBoundsApprox(vx, misrate, "seed") = [q_(alpha\/2), q_(1-alpha\/2)] $

where $q_p$ is the $p$-th quantile of bootstrap $Center$ estimates

Nominal bounds on $Center(vx)$ with specified coverage.

#block(
  fill: rgb("#fff3cd"),
  inset: 10pt,
  radius: 4pt,
  width: 100%,
)[
  *Undercoverage Warning*

  The bootstrap percentile method has known undercoverage for small samples.
  When requesting 95% confidence ($misrate = 0.05$), actual coverage is typically
  85–92% for $n < 30$. This is inherent to the method, not a bug.

  For exact coverage, use $CenterBounds$ (if symmetry holds) or $MedianBounds$ (no symmetry).
]

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — bootstrap percentile confidence interval for Hodges-Lehmann],
  [*Interpretation* — $misrate$ is NOMINAL (not exact) error probability],
  [*Domain* — any real numbers, $n >= 2$, $misrate >= max(2\/B, 2^(1-n))$ where $B = 10000$ iterations (both bootstrap resolution and signed-rank minimum apply)],
  [*Unit* — same as measurements],
  [*Note* — assumes weak continuity only (no symmetry); deterministic with fixed seed; uses $m$-out-of-$n$ subsampling with $m = 5000$ when $n > 5000$, scaling bounds at asymptotic $sqrt(n)$ rate to full sample size],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift equivariance* #h(2em) $CenterBoundsApprox(vx + k, misrate, "seed") = CenterBoundsApprox(vx, misrate, "seed") + k$],
  [*Scale equivariance* #h(2em) $CenterBoundsApprox(k dot vx, misrate, "seed") = k dot CenterBoundsApprox(vx, misrate, "seed")$],
)

#v(0.3em)
*Example*

- `CenterBoundsApprox([1..10], 0.05) = [3.5, 7.5]` where `Center = 5.5` (default seed "center-bounds-approx")
- Bounds fail to cover true center with probability $approx misrate$ (nominal, not exact)

#v(0.5em)
$CenterBoundsApprox$ provides not just the estimated center but also the uncertainty of that estimate.
The function returns an interval of plausible center values given the data using bootstrap resampling.
Set $misrate$ to control the nominal error rate (actual coverage may differ for small $n$).
These bounds require no symmetry assumption, only weak continuity.
If the bounds exclude some reference value, that suggests the true center differs reliably from that value.
