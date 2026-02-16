#import "/manual/definitions.typ": *

$ Ratio(vx, vy) = exp(Shift(log vx, log vy)) $

The $Ratio$ test suite contains 37 test cases (25 original + 12 unsorted), excluding zero values due to division constraints.
The new definition uses geometric interpolation (via log-space), which affects expected values for even $m times n$ cases.

*Demo examples* ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $vx = (1, 2, 4, 8, 16)$, $vy = (2, 4, 8, 16, 32)$, expected output: $0.5$ (base case, odd $m times n$)
- `demo-2`: $vx = (1, 2, 4, 8, 16)$, $vy = (1, 2, 4, 8, 16)$, expected output: $1$ (identity property)
- `demo-3`: $vx = (2, 4, 8, 16, 32)$, $vy = (10, 20, 40, 80, 160)$ (= [2×demo-1.x, 5×demo-1.y]), expected output: $0.2$ (scale property)

*Natural sequences* ($[n, m] in {1, 2, 3} times {1, 2, 3}$) — 9 combinations:

- `natural-1-1`: $vx = (1)$, $vy = (1)$, expected output: $1$
- `natural-1-2`: $vx = (1)$, $vy = (1, 2)$, expected output: $approx 0.707$ ($= sqrt(0.5)$, geometric interpolation)
- `natural-1-3`: $vx = (1)$, $vy = (1, 2, 3)$, expected output: $0.5$
- `natural-2-1`: $vx = (1, 2)$, $vy = (1)$, expected output: $approx 1.414$ ($= sqrt(2)$, geometric interpolation)
- `natural-2-2`: $vx = (1, 2)$, $vy = (1, 2)$, expected output: $1$
- `natural-2-3`: $vx = (1, 2)$, $vy = (1, 2, 3)$, expected output: $approx 0.816$ (geometric interpolation)
- `natural-3-1`: $vx = (1, 2, 3)$, $vy = (1)$, expected output: $2$
- `natural-3-2`: $vx = (1, 2, 3)$, $vy = (1, 2)$, expected output: $approx 1.225$ (geometric interpolation)
- `natural-3-3`: $vx = (1, 2, 3)$, $vy = (1, 2, 3)$, expected output: $1$

*Additive distribution* ($[n, m] in {5, 10, 30} times {5, 10, 30}$) — 9 combinations with $Additive(10, 1)$:

- `additive-5-5`, `additive-5-10`, `additive-5-30`
- `additive-10-5`, `additive-10-10`, `additive-10-30`
- `additive-30-5`, `additive-30-10`, `additive-30-30`
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

*Uniform distribution* ($[n, m] in {5, 100} times {5, 100}$) — 4 combinations with $Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $vx$ uses seed 2, $vy$ uses seed 3
- Note: all generated values are strictly positive (no zeros); values near zero test numerical stability of log-transformation

The natural sequences verify the identity property ($Ratio(vx, vx) = 1$) and validate ratio calculations with simple integer inputs.
Note that implementations should handle the practical constraint of avoiding division by values near zero.

*Unsorted tests* — verify independent sorting for ratio calculation (12 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4)}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: $vx = (16, 1, 8, 2, 4)$, $vy = (2, 4, 8, 16, 32)$ (demo-1 with X unsorted)
- `unsorted-demo-unsorted-y`: $vx = (1, 2, 4, 8, 16)$, $vy = (32, 2, 16, 4, 8)$ (demo-1 with Y unsorted)
- `unsorted-demo-both-unsorted`: $vx = (8, 1, 16, 4, 2)$, $vy = (16, 32, 2, 8, 4)$ (demo-1 both unsorted)
- `unsorted-identity-unsorted`: $vx = (4, 1, 8, 2, 16)$, $vy = (16, 1, 8, 4, 2)$ (identity property, both unsorted)
- `unsorted-asymmetric-unsorted-2-3`: $vx = (2, 1)$, $vy = (3, 1, 2)$ (asymmetric, both unsorted)
- `unsorted-power-unsorted-5`: $vx = (16, 2, 8, 1, 4)$, $vy = (32, 4, 16, 2, 8)$ (powers of 2 unsorted)
