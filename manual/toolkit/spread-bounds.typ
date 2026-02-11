#import "/manual/definitions.typ": *

=== SpreadBounds

$ SpreadBounds(vx, misrate) = [d_((k_L)), d_((k_U))] $

where $m = floor(n / 2)$, $vd$ is the sorted absolute differences from a random disjoint pairing,
$k_L = r + 1$, $k_U = m - r$, and $r$ is the randomized sign-test cutoff for $"Binomial"(m, 1 / 2)$.

Robust bounds on $Spread(vx)$ with specified coverage.

#v(0.3em)
#list(marker: none, tight: true,
  [*Interpretation* --- $misrate$ is probability that true spread falls outside bounds],
  [*Domain* --- any real numbers, $n >= 2$, $misrate >= 2^(1-m)$],
  [*Assumptions* --- #link(<sec-sparity>)[`sparity(x)`]],
  [*Unit* --- same as measurements],
  [*Note* --- disjoint-pair sign-test inversion; randomized cutoff matches requested misrate exactly under weak continuity; conservative with ties],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift invariance* #h(2em) $SpreadBounds(vx + c, misrate) = SpreadBounds(vx, misrate)$],
  [*Scale equivariance* #h(2em) $SpreadBounds(c dot vx, misrate) = abs(c) dot SpreadBounds(vx, misrate)$],
  [*Non-negativity* #h(2em) $SpreadBounds(vx, misrate) = [a, b]$ where $a >= 0, b >= 0$],
  [*Monotonicity in misrate* #h(2em) smaller $misrate$ produces wider bounds],
)

#v(0.3em)
*Example*

- `SpreadBounds([1..30], 0.01)` where `Spread = 9`
- Bounds fail to cover true spread with probability $approx misrate$

#v(0.5em)
$SpreadBounds$ provides distribution-free bounds on the $Spread$ estimate.
It uses disjoint pairs and an exact sign-test inversion,
which guarantees coverage regardless of the underlying distribution.
Set $misrate$ to control how often the bounds might fail to contain the true spread:
use $10^(-3)$ for everyday analysis or $10^(-6)$ for critical decisions.
The cutoff $r$ is clamped so that $floor(r / 2) <= (m - 1) / 2$,
ensuring the lower and upper order statistics remain within the sorted differences.

#v(0.5em)
*Minimum sample size* ---
the sign test on $m = floor(n / 2)$ pairs has minimum achievable misrate $2^(1-m)$:

#v(0.3em)
#align(center,
  table(
    columns: 5,
    align: center,
    table.header[$misrate$][$10^(-1)$][$10^(-2)$][$10^(-3)$][$10^(-6)$],
    [$n_min$], [10], [16], [22], [42],
  )
)
