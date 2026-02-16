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

The $SpreadBounds$ test suite contains 46 test cases (3 demo + 4 natural + 4 property + 7 edge + 3 additive + 2 uniform + 5 misrate + 5 conservatism + 8 unsorted + 5 error cases).
Since $SpreadBounds$ returns bounds rather than a point estimate, tests validate that bounds are well-formed and satisfy equivariance properties under a fixed seed.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.
Because pairing and cutoff selection are randomized, tests fix `seed` to keep outputs deterministic.

*Demo examples* --- from manual introduction, validating basic bounds:

- `demo-1`: $vx = (1, 2, ..., 30)$, $misrate = 0.01$, bounds containing $Spread = 9$
- `demo-2`: $vx = (1, 2, ..., 30)$, $misrate = 0.002$, wider bounds (tighter misrate)
- `demo-3`: $vx = (1, 2, ..., 15)$, $misrate = 0.07$

These cases illustrate how tighter misrates produce wider bounds and how sample size affects bound width.

*Natural sequences* (misrate varies by size) --- 4 tests:

- `natural-10`: $vx = (1, 2, ..., 10)$, $misrate = 0.15$
- `natural-15`: $vx = (1, 2, ..., 15)$, $misrate = 0.05$
- `natural-20`: $vx = (1, 2, ..., 20)$, $misrate = 0.05$
- `natural-30`: $vx = (1, 2, ..., 30)$, $misrate = 0.05$

*Property validation* ($n = 10$, $misrate = 0.2$) --- 4 tests:

- `property-identity`: $vx = (1, 2, ..., 10)$, bounds must contain $Spread$
- `property-location-shift`: $vx = (11, 12, ..., 20)$ (= identity + 10), bounds must equal identity bounds (shift invariance)
- `property-scale-2x`: $vx = (2, 4, ..., 20)$ (= 2 $times$ identity), bounds must be 2$times$ identity bounds (scale equivariance)
- `property-scale-neg`: $vx = (-10, -9, ..., -1)$ (= $-1 times$ identity), bounds must equal identity bounds ($abs(k)$ scaling)

*Edge cases* --- boundary conditions and extreme scenarios (7 tests):

- `edge-small-non-trivial`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.8$ (small but non-trivial bounds)
- `edge-large-misrate`: $vx = (1, 2, ..., 10)$, $misrate = 0.5$ (permissive bounds)
- `edge-duplicates-mixed`: $vx = (1, 1, 1, 2, 3, 4, 5)$, $misrate = 0.5$ (partial ties)
- `edge-wide-range`: $vx = (1, 10, 100, 1000, 10000)$, $misrate = 0.8$ (extreme value range)
- `edge-negative`: $vx = (-5, -4, -3, -2, -1)$, $misrate = 0.8$ (negative values)
- `edge-large-n`: $vx = (1, 2, ..., 100)$, $misrate = 0.01$ (large sample, tighter sign-test bounds)
- `edge-n2`: $vx = (1, 3)$, $misrate = 1.0$ (minimum sample size, only valid misrate is 1.0)

*Additive distribution* (misrate varies by size) --- 3 tests with $Additive(10, 1)$:

- `additive-20`: $n = 20$, $misrate = 0.02$
- `additive-30`: $n = 30$, $misrate = 0.01$
- `additive-50`: $n = 50$, $misrate = 0.01$

*Uniform distribution* (misrate varies by size) --- 2 tests with $Uniform(0, 1)$:

- `uniform-20`: $n = 20$, $misrate = 0.02$
- `uniform-50`: $n = 50$, $misrate = 0.01$

*Misrate variation* ($vx = (1, 2, ..., 25)$) --- 5 tests with varying misrates:

- `misrate-5e-1`: $misrate = 0.5$
- `misrate-1e-1`: $misrate = 0.1$
- `misrate-5e-2`: $misrate = 0.05$
- `misrate-1e-2`: $misrate = 0.01$
- `misrate-2e-3`: $misrate = 0.002$

These tests validate monotonicity: smaller misrates produce wider bounds.

*Conservatism tests* ($misrate = 0.1$) --- 5 tests unique to $SpreadBounds$:

- `conservatism-12`: $vx = (1, 2, ..., 12)$, sign-test bounds are wide relative to $Spread$
- `conservatism-15`: $vx = (1, 2, ..., 15)$
- `conservatism-20`: $vx = (1, 2, ..., 20)$
- `conservatism-30`: $vx = (1, 2, ..., 30)$
- `conservatism-50`: $vx = (1, 2, ..., 50)$

These tests document how discreteness-driven conservatism decreases with increasing sample size.
For small $n$, bounds may span a large part of the pairwise-difference range.
For large $n$, bounds tighten to a practical interval around $Spread$.

*Unsorted tests* --- verify stable behavior on non-sorted inputs (8 tests):

- `unsorted-reverse-10`: $vx = (10, 9, ..., 1)$, $misrate = 0.2$
- `unsorted-reverse-15`: $vx = (15, 14, ..., 1)$, $misrate = 0.07$
- `unsorted-shuffle-10`: $vx$ shuffled, $misrate = 0.2$
- `unsorted-shuffle-15`: $vx$ shuffled, $misrate = 0.07$
- `unsorted-negative-5`: negative values unsorted ($misrate = 0.8$)
- `unsorted-mixed-signs-5`: mixed signs unsorted ($misrate = 0.8$)
- `unsorted-duplicates`: $vx = (1, 3, 1, 3, 2)$, unsorted with duplicates ($misrate = 0.8$)
- `unsorted-wide-range`: $vx = (1000, 1, 100, 10, 10000)$, unsorted wide range ($misrate = 0.8$)

These tests validate that $SpreadBounds$ produces sensible bounds for arbitrary input order under a fixed seed.

*Error cases* --- inputs that violate assumptions (5 tests):

- `error-empty-array`: $vx = ()$, $misrate = 0.5$ — empty array violates validity
- `error-single-element`: $vx = (1)$, $misrate = 0.5$ — $m = 0$ violates domain ($n$ too small to form pairs)
- `error-misrate-zero`: $vx = (1, 2, ..., 10)$, $misrate = 0$ — below minimum achievable misrate
- `error-invalid-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.001$ — below minimum achievable misrate ($2^(1-2) = 0.5$)
- `error-constant-sample`: $vx = (1, 1, 1, 1, 1)$, $misrate = 0.5$ — constant sample violates sparity ($Spread = 0$)

Note: $SpreadBounds$ has a minimum misrate constraint.
The sign-test inversion requires $misrate >= 2^(1-m)$.
