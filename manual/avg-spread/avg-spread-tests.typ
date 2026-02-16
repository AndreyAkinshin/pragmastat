#import "/manual/definitions.typ": *

$ AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m) $

The $AvgSpread$ test suite contains 36 test cases (5 demo + 4 natural + 1 negative + 9 additive + 4 uniform + 1 composite + 12 unsorted).
Since $AvgSpread$ is a weighted average of two $Spread$ estimates, tests validate both the individual spread calculations and the weighting formula.

*Demo examples* ($n = m = 5$) --- from manual introduction, validating properties:

- `demo-1`: $vx = (0, 3, 6, 9, 12)$, $vy = (0, 2, 4, 6, 8)$, expected output: $5$ (base case)
- `demo-2`: $vx = (0, 3, 6, 9, 12)$, $vy = (0, 3, 6, 9, 12)$, expected output: $6$ (equal samples)
- `demo-3`: $vx = (0, 6, 12, 18, 24)$, $vy = (0, 9, 18, 27, 36)$, expected output: $15$ (scale equivariance, $3 times$ demo-1)
- `demo-4`: $vx = (0, 2, 4, 6, 8)$, $vy = (0, 3, 6, 9, 12)$, expected output: $5$ (swap symmetry with demo-1)
- `demo-5`: $vx = (0, 6, 12, 18, 24)$, $vy = (0, 4, 8, 12, 16)$, expected output: $10$ (scale, $2 times$ demo-1)

*Natural sequences* ($[n, m] in {2, 3} times {2, 3}$) --- 4 combinations:

- `natural-2-2`, `natural-2-3`, `natural-3-2`, `natural-3-3`
- Minimum size $n, m >= 2$ required for meaningful dispersion

*Negative values* ($[n, m] = [2, 2]$) --- sign handling validation:

- `negative-2-2`: $vx = (-2, -1)$, $vy = (-2, -1)$, expected output: $1$

*Additive distribution* ($[n, m] in {5, 10, 30} times {5, 10, 30}$) --- 9 combinations with $Additive(10, 1)$:

- `additive-5-5`, `additive-5-10`, `additive-5-30`
- `additive-10-5`, `additive-10-10`, `additive-10-30`
- `additive-30-5`, `additive-30-10`, `additive-30-30`
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

*Uniform distribution* ($[n, m] in {5, 100} times {5, 100}$) --- 4 combinations with $Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

*Composite estimator stress test* --- 1 test:

- `composite-asymmetric-weights`: $vx = (1, 2)$, $vy = (3, 4, 5, 6, 7, 8, 9, 10)$ ($n = 2$, $m = 8$, highly asymmetric weights $w_x = 0.2$, $w_y = 0.8$)

*Unsorted tests* --- verify sorting independence (12 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: demo-1 with X unsorted
- `unsorted-demo-unsorted-y`: demo-1 with Y unsorted
- `unsorted-demo-both-unsorted`: demo-1 with both unsorted
- `unsorted-identity-unsorted`: equal samples, both unsorted
- `unsorted-negative-unsorted`: negative values, both unsorted
- `unsorted-asymmetric-weights-unsorted`: asymmetric weights, both unsorted

As a composite estimator, $AvgSpread$ tests both individual $Spread$ computations and the weighted combination.
Unsorted variants verify end-to-end correctness including the weighting formula.
