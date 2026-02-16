#import "/manual/definitions.typ": *

$ AvgSpreadBounds(vx, vy, misrate) = [L_A, U_A] $

Let $alpha = misrate / 2$ (equal Bonferroni split).
Compute
$[L_x, U_x] = SpreadBounds(vx, alpha)$ and
$[L_y, U_y] = SpreadBounds(vy, alpha)$
using disjoint-pair sign-test inversion (see $SpreadBounds$).
Let $w_x = n / (n + m)$ and $w_y = m / (n + m)$.
Return
$[L_A, U_A] = [w_x L_x + w_y L_y, w_x U_x + w_y U_y]$.

The $AvgSpreadBounds$ test suite validates:

- bounds are well-formed ($L_A <= U_A$ and non-negative)
- shift invariance and scale equivariance
- monotonicity in $misrate$
- symmetry under swapping $vx$ and $vy$ (with equal split)
- error cases for invalid inputs and misrate domain violations

Because $SpreadBounds$ is randomized, tests fix a seed to make outputs deterministic.
Both $SpreadBounds$ calls use the same seed (two identical RNG streams).

*Minimum misrate constraint* ---
the equal split requires

$ alpha >= 2^(1-floor(n/2)) $ and $ alpha >= 2^(1-floor(m/2)) $,

so

$ misrate >= 2 dot max(2^(1-floor(n/2)), 2^(1-floor(m/2))) $.
