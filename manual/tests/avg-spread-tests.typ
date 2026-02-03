#import "/manual/definitions.typ": *

== AvgSpread Tests

$ AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m) $

The $AvgSpread$ test suite contains 36 test cases (24 original + 12 unsorted).
Since $AvgSpread$ computes $Spread(vx)$ and $Spread(vy)$ independently, unsorted tests are critical to verify that both samples are sorted independently before computing their spreads.

*Demo examples* ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $vx = (0, 3, 6, 9, 12)$, $vy = (0, 2, 4, 6, 8)$, expected output: $5$ (base case: $(5 dot 6 + 5 dot 4)\/10$)
- `demo-2`: $vx = (0, 3, 6, 9, 12)$, $vy = (0, 3, 6, 9, 12)$, expected output: $6$ (identity case)
- `demo-3`: $vx = (0, 6, 12, 18, 24)$, $vy = (0, 9, 18, 27, 36)$ (= [2×demo-1.x, 3×demo-1.y]), expected output: $15$ (scale equivariance)
- `demo-4`: $vx = (0, 2, 4, 6, 8)$, $vy = (0, 3, 6, 9, 12)$ (= reversed demo-1), expected output: $5$ (symmetry)
- `demo-5`: $vx = (0, 6, 12, 18, 24)$, $vy = (0, 4, 8, 12, 16)$ (= 2 × demo-1), expected output: $10$ (uniform scaling)

*Natural sequences* ($[n, m] in {2, 3} times {2, 3}$) — 4 combinations:

- All combinations for two- and three-element samples, validating the weighted average calculation

*Negative values* ($[n, m] = [2, 2]$) — validates spread calculation with negative values:

- `negative-2-2`: $vx = (-2, -1)$, $vy = (-2, -1)$, expected output: $1$

*Additive distribution* ($[n, m] in {5, 10, 30} times {5, 10, 30}$) — 9 combinations with $Additive(10, 1)$:

- Tests pooled dispersion across different sample size combinations
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

*Uniform distribution* ($[n, m] in {5, 100} times {5, 100}$) — 4 combinations with $Uniform(0, 1)$:

- Validates correct weighting when sample sizes differ substantially
- Random generation: $vx$ uses seed 2, $vy$ uses seed 3

The asymmetric size combinations are particularly important for $AvgSpread$ because the estimator must correctly weight each sample's contribution by its size.

*Composite estimator stress tests* — edge cases for weighted averaging:

- `composite-asymmetric-weights`: $vx = (1, 2)$, $vy = (3, 4, 5, 6, 7, 8, 9, 10)$ (2 vs 8, tests weighting formula)

*Unsorted tests* — critical for verifying independent sorting (12 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: $vx = (12, 0, 6, 3, 9)$, $vy = (0, 2, 4, 6, 8)$ (demo-1 with X unsorted)
- `unsorted-demo-unsorted-y`: $vx = (0, 3, 6, 9, 12)$, $vy = (8, 0, 4, 2, 6)$ (demo-1 with Y unsorted)
- `unsorted-demo-both-unsorted`: $vx = (9, 0, 12, 3, 6)$, $vy = (6, 0, 8, 2, 4)$ (demo-1 both unsorted)
- `unsorted-identity-unsorted`: $vx = (6, 0, 12, 3, 9)$, $vy = (9, 0, 12, 6, 3)$ (demo-2 unsorted)
- `unsorted-negative-unsorted`: $vx = (-1, -2)$, $vy = (-1, -2)$ (negative unsorted)
- `unsorted-asymmetric-weights-unsorted`: $vx = (2, 1)$, $vy = (8, 3, 6, 4, 10, 5, 9, 7)$ (asymmetric unsorted)

These tests verify that implementations compute $Spread(vx)$ and $Spread(vy)$ with properly sorted samples.
