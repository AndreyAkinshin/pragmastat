#import "/manual/definitions.typ": *

$ Disparity(vx, vy) = Shift(vx, vy) / AvgSpread(vx, vy) $

The $Disparity$ test suite contains 28 test cases (16 original + 12 unsorted).
Since $Disparity$ combines $Shift$ and $AvgSpread$, unsorted tests verify both components handle sorting correctly.

*Demo examples* ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $vx = (0, 3, 6, 9, 12)$, $vy = (0, 2, 4, 6, 8)$, expected output: $0.4$ (base case: $2\/5$)
- `demo-2`: $vx = (5, 8, 11, 14, 17)$, $vy = (5, 7, 9, 11, 13)$ (= demo-1 + 5), expected output: $0.4$ (location invariance)
- `demo-3`: $vx = (0, 6, 12, 18, 24)$, $vy = (0, 4, 8, 12, 16)$ (= 2 × demo-1), expected output: $0.4$ (scale invariance)
- `demo-4`: $vx = (0, 2, 4, 6, 8)$, $vy = (0, 3, 6, 9, 12)$ (= reversed demo-1), expected output: $-0.4$ (anti-symmetry)

*Natural sequences* ($[n, m] in {2, 3} times {2, 3}$) — 4 combinations:

- `natural-2-2`, `natural-2-3`, `natural-3-2`, `natural-3-3`
- Minimum size $n, m >= 2$ required for meaningful dispersion calculations

*Negative values* ($[n, m] = [2, 2]$) — end-to-end validation with negative values:

- `negative-2-2`: $vx = (-2, -1)$, $vy = (-2, -1)$, expected output: $0$

*Uniform distribution* ($[n, m] in {5, 100} times {5, 100}$) — 4 combinations with $Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

The smaller test set for $Disparity$ reflects implementation confidence.
Since $Disparity$ combines $Shift$ and $AvgSpread$, correct implementation of those components ensures $Disparity$ correctness.
The test cases validate the division operation and confirm scale-free properties.

*Composite estimator stress tests* — edge cases for effect size calculation:

- `composite-small-avgspread`: $vx = (10.001, 10.002, 10.003)$, $vy = (10.004, 10.005, 10.006)$ (tiny spread, large shift)
- `composite-large-avgspread`: $vx = (1, 100, 200)$, $vy = (50, 150, 250)$ (large spread, small shift)
- `composite-extreme-disparity`: $vx = (1, 1.001)$, $vy = (100, 100.001)$ (extreme ratio, tests precision)

*Unsorted tests* — verify both Shift and AvgSpread handle sorting (12 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: $vx = (12, 0, 6, 3, 9)$, $vy = (0, 2, 4, 6, 8)$ (demo-1 with X unsorted)
- `unsorted-demo-unsorted-y`: $vx = (0, 3, 6, 9, 12)$, $vy = (8, 0, 4, 2, 6)$ (demo-1 with Y unsorted)
- `unsorted-demo-both-unsorted`: $vx = (9, 0, 12, 3, 6)$, $vy = (6, 0, 8, 2, 4)$ (demo-1 both unsorted)
- `unsorted-location-invariance-unsorted`: $vx = (17, 5, 11, 8, 14)$, $vy = (13, 5, 9, 7, 11)$ (demo-2 unsorted)
- `unsorted-scale-invariance-unsorted`: $vx = (24, 0, 12, 6, 18)$, $vy = (16, 0, 8, 4, 12)$ (demo-3 unsorted)
- `unsorted-anti-symmetry-unsorted`: $vx = (8, 0, 4, 2, 6)$, $vy = (12, 0, 6, 3, 9)$ (demo-4 reversed and unsorted)

As a composite estimator, $Disparity$ tests both the numerator ($Shift$) and denominator ($AvgSpread$).
Unsorted variants verify end-to-end correctness including invariance properties.
