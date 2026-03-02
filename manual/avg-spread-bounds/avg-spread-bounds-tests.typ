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

The $AvgSpreadBounds$ test suite contains 40 test cases (3 demo + 5 natural + 6 property + 6 edge + 2 distro + 5 misrate + 6 unsorted + 7 error).
Since $AvgSpreadBounds$ returns bounds rather than a point estimate, tests validate that bounds are well-formed and satisfy equivariance properties.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.
Because $SpreadBounds$ is randomized, tests fix a seed to make outputs deterministic.
Both $SpreadBounds$ calls use the same seed (two identical RNG streams).

*Minimum misrate constraint* ---
the equal split requires

$ alpha >= 2^(1-floor(n/2)) $ and $ alpha >= 2^(1-floor(m/2)) $,

so

$ misrate >= 2 dot max(2^(1-floor(n/2)), 2^(1-floor(m/2))) $.

*Demo examples* ($n = m = 30$, $n = m = 20$) --- from manual introduction:

- `demo-1`: $vx = (1, ..., 30)$, $vy = (21, ..., 50)$, $misrate = 0.02$
- `demo-2`: $vx = (1, ..., 30)$, $vy = (21, ..., 50)$, $misrate = 0.005$, wider bounds (tighter misrate)
- `demo-3`: $vx = (1, ..., 20)$, $vy = (5, ..., 24)$, $misrate = 0.05$

These cases illustrate how tighter misrates produce wider bounds.

*Natural sequences* ($misrate$ varies) --- 5 tests:

- `natural-10-10`: $vx = (1, ..., 10)$, $vy = (1, ..., 10)$, $misrate = 0.2$
- `natural-10-15`: $vx = (1, ..., 10)$, $vy = (1, ..., 15)$, $misrate = 0.2$
- `natural-15-10`: $vx = (1, ..., 15)$, $vy = (1, ..., 10)$, $misrate = 0.2$
- `natural-15-15`: $vx = (1, ..., 15)$, $vy = (1, ..., 15)$, $misrate = 0.2$
- `natural-20-20`: $vx = (1, ..., 20)$, $vy = (1, ..., 20)$, $misrate = 0.1$

*Property validation* ($n = m = 10$, $misrate = 0.2$) --- 6 tests:

- `property-identity`: $vx = (0, 2, ..., 18)$, $vy = (0, 2, ..., 18)$, expected output: $[2, 16]$
- `property-location-shift`: $vx = (10, 12, ..., 28)$, $vy = (12, 14, ..., 30)$, expected output: $[2, 16]$ (shift invariance)
- `property-scale-2x`: $vx = (0, 4, ..., 36)$, $vy = (4, 8, ..., 40)$, expected output: $[4, 32]$ (= 2× identity bounds, scale equivariance)
- `property-scale-neg`: $vx = (0, -2, ..., -18)$, $vy = (-2, -4, ..., -20)$, expected output: $[2, 16]$ (= identity bounds, $abs(k)$ scaling)
- `property-symmetry`: $vx = (1, ..., 10)$, $vy = (6, ..., 15)$
- `property-symmetry-swapped`: $vx = (6, ..., 15)$, $vy = (1, ..., 10)$, same output as `property-symmetry` (swap symmetry with equal Bonferroni split)

*Edge cases* --- boundary conditions and extreme scenarios (6 tests):

- `edge-small`: $n = m = 4$, $misrate = 1.0$ (minimum non-trivial)
- `edge-negative`: $vx = (-10, ..., -1)$, $vy = (-20, ..., -11)$, $misrate = 0.2$ (negative values)
- `edge-mixed-signs`: mixed positive/negative values, $misrate = 0.2$
- `edge-wide-range`: powers of 10 from $1$ to $10^9$, $misrate = 0.2$ (extreme value range)
- `edge-asymmetric-8-30`: $n = 8$, $m = 30$, $misrate = 0.3$ (unbalanced sizes)
- `edge-duplicates-mixed`: $vx = (1, 1, 1, 2, 3, 4)$, $vy = (2, 2, 2, 3, 4, 5)$, $misrate = 0.6$ (partial ties)

*Distribution tests* ($misrate = 0.05$) --- 2 tests:

- `additive-20-20`: $n = m = 20$, $Additive(10, 1)$
- `uniform-20-20`: $n = m = 20$, $Uniform(0, 1)$

*Misrate variation* ($vx = (1, ..., 20)$, $vy = (5, ..., 24)$) --- 5 tests:

- `misrate-2e-1`: $misrate = 0.2$
- `misrate-1e-1`: $misrate = 0.1$
- `misrate-5e-2`: $misrate = 0.05$
- `misrate-2e-2`: $misrate = 0.02$
- `misrate-1e-2`: $misrate = 0.01$

These tests validate monotonicity: smaller misrates produce wider bounds.

*Unsorted tests* --- verify independent sorting of $vx$ and $vy$ (6 tests):

- `unsorted-reverse-x`: X reversed, Y sorted
- `unsorted-reverse-y`: X sorted, Y reversed
- `unsorted-reverse-both`: both reversed
- `unsorted-shuffle-x`: X shuffled, Y sorted
- `unsorted-shuffle-y`: X sorted, Y shuffled
- `unsorted-wide-range`: wide value range, both unsorted

These tests validate that $AvgSpreadBounds$ produces identical results regardless of input order.

*Error cases* --- inputs that violate assumptions (7 tests):

- `error-empty-x`: $vx = ()$ (empty X array) — validity error
- `error-empty-y`: $vy = ()$ (empty Y array) — validity error
- `error-single-element-x`: $|vx| = 1$ (too few elements for pairing) — domain error
- `error-single-element-y`: $|vy| = 1$ (too few elements for pairing) — domain error
- `error-constant-x`: constant $vx$ violates sparity ($Spread = 0$)
- `error-constant-y`: constant $vy$ violates sparity ($Spread = 0$)
- `error-misrate-below-min`: misrate below minimum achievable — domain error
