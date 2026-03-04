#import "/manual/definitions.typ": *

$ SpreadBounds(vx, misrate) = [L, U] $

Let $m = floor(n / 2)$.
Draw a value-independent random disjoint pairing of the sample,
compute $vd = { abs(x_{pi(2i-1)} - x_{pi(2i)}) }$ for $i = 1..m$,
and sort ascending.

Let $r$ be the largest integer such that
$sum_(i=0)^r binom(m, i) / 2^m <= misrate / 2$.
If the target lies between two adjacent CDF steps, $r$ is randomized between
$r$ and $r + 1$ to match the requested misrate.
Define $k_L = r + 1$ and $k_U = m - r$.

Return $[L, U] = [d_((k_L)), d_((k_U))]$.

The $SpreadBounds$ test suite contains 43 test cases (3 demo + 4 natural + 4 property + 7 edge + 3 additive + 2 uniform + 5 misrate + 5 conservatism + 8 unsorted + 2 error).
Since $SpreadBounds$ returns bounds rather than a point estimate, tests validate that bounds are well-formed and satisfy equivariance properties under a fixed seed.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.
Because pairing and cutoff selection are randomized, tests fix `seed` to keep outputs deterministic.

*Demo examples* --- from manual introduction, validating basic bounds:

- `demo-1`: $vx = (1, 2, ..., 30)$, bounds containing $Spread = 9$
- `demo-2`: $vx = (1, 2, ..., 30)$, stricter fixture misrate, wider bounds
- `demo-3`: $vx = (1, 2, ..., 15)$, looser fixture misrate

These cases illustrate how tighter misrates produce wider bounds and how sample size affects bound width.

*Natural sequences* --- 4 tests:

- `natural-10`: $vx = (1, 2, ..., 10)$
- `natural-15`: $vx = (1, 2, ..., 15)$
- `natural-20`: $vx = (1, 2, ..., 20)$
- `natural-30`: $vx = (1, 2, ..., 30)$

*Property validation* ($n = 10$) --- 4 tests:

- `property-identity`: $vx = (1, 2, ..., 10)$, bounds must contain $Spread$
- `property-location-shift`: $vx = (11, 12, ..., 20)$ (= identity + 10), bounds must equal identity bounds (shift invariance)
- `property-scale-2x`: $vx = (2, 4, ..., 20)$ (= 2 $times$ identity), bounds must be 2$times$ identity bounds (scale equivariance)
- `property-scale-neg`: $vx = (-10, -9, ..., -1)$ (= $-1 times$ identity), bounds must equal identity bounds ($abs(k)$ scaling)

*Edge cases* --- boundary conditions and extreme scenarios (7 tests):

- `edge-small-non-trivial`: $vx = (1, 2, 3, 4, 5)$ (small but non-trivial bounds)
- `edge-large-misrate`: $vx = (1, 2, ..., 10)$ (permissive bounds)
- `edge-duplicates-mixed`: $vx = (1, 1, 1, 2, 3, 4, 5)$ (partial ties)
- `edge-wide-range`: $vx = (1, 10, 100, 1000, 10000)$ (extreme value range)
- `edge-negative`: $vx = (-5, -4, -3, -2, -1)$ (negative values)
- `edge-large-n`: $vx = (1, 2, ..., 100)$ (large sample, tighter sign-test bounds)
- `edge-n2`: $vx = (1, 3)$ (minimum sample size, only the maximal achievable misrate is valid)

*Additive distribution* (reference fixture misrates) --- 3 tests with $Additive(10, 1)$:

- `additive-20`: $n = 20$
- `additive-30`: $n = 30$
- `additive-50`: $n = 50$

*Uniform distribution* (reference fixture misrates) --- 2 tests with $Uniform(0, 1)$:

- `uniform-20`: $n = 20$
- `uniform-50`: $n = 50$

*Misrate variation* ($vx = (1, 2, ..., 25)$) --- 5 tests with varying fixture misrates:

These tests validate monotonicity: smaller misrates produce wider bounds.

*Conservatism tests* (loose achievable fixture misrates) --- 5 tests unique to $SpreadBounds$:

- `conservatism-12`: $vx = (1, 2, ..., 12)$, sign-test bounds are wide relative to $Spread$
- `conservatism-15`: $vx = (1, 2, ..., 15)$
- `conservatism-20`: $vx = (1, 2, ..., 20)$
- `conservatism-30`: $vx = (1, 2, ..., 30)$
- `conservatism-50`: $vx = (1, 2, ..., 50)$

These tests document how discreteness-driven conservatism decreases with increasing sample size.
For small $n$, bounds may span a large part of the pairwise-difference range.
For large $n$, bounds tighten to a practical interval around $Spread$.

*Unsorted tests* --- verify stable behavior on non-sorted inputs (8 tests):

- `unsorted-reverse-10`: $vx = (10, 9, ..., 1)$
- `unsorted-reverse-15`: $vx = (15, 14, ..., 1)$, $misrate = 0.07$
- `unsorted-shuffle-10`: $vx$ shuffled
- `unsorted-shuffle-15`: $vx$ shuffled, $misrate = 0.07$
- `unsorted-negative-5`: negative values unsorted
- `unsorted-mixed-signs-5`: mixed signs unsorted
- `unsorted-duplicates`: $vx = (1, 3, 1, 3, 2)$, unsorted with duplicates
- `unsorted-wide-range`: $vx = (1000, 1, 100, 10, 10000)$, unsorted wide range

These tests validate that $SpreadBounds$ produces sensible bounds for arbitrary input order under a fixed seed.

*Error cases* --- inputs that violate assumptions (2 tests):

- `error-empty-x`: $vx = ()$ — empty array violates validity
- `error-constant-x`: $vx = (5, 5, ..., 5)$ ($n = 20$) — constant sample violates sparity ($Spread = 0$)

Note: $SpreadBounds$ has a minimum misrate constraint.
The sign-test inversion requires $misrate >= 2^(1-m)$.
