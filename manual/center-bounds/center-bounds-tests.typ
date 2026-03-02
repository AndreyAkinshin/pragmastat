#import "/manual/definitions.typ": *

$ CenterBounds(vx, misrate) = [w_((k_"left")), w_((k_"right"))] $

where $vw = { (x_i + x_j) \/ 2 }$ (pairwise averages, sorted) for $i <= j$,
$k_"left" = floor(SignedRankMargin \/ 2) + 1$,
$k_"right" = N - floor(SignedRankMargin \/ 2)$, and $N = n(n+1)\/2$.

The $CenterBounds$ test suite contains 38 test cases (3 demo + 4 natural + 5 property + 7 edge + 4 symmetric + 2 additive + 2 uniform + 4 misrate + 6 unsorted + 1 error).
Since $CenterBounds$ returns bounds rather than a point estimate, tests validate that bounds contain $Center(vx)$ and satisfy equivariance properties.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.

*Demo examples* — from manual introduction, validating basic bounds:

- `demo-1`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.1$, expected output: $[1.5, 4.5]$
- `demo-2`: $vx = (1, ..., 10)$, $misrate = 0.01$, expected output: $[2.5, 8.5]$
- `demo-3`: $vx = (0, 2, 4, 6, 8)$, $misrate = 0.1$, expected output: $[1, 7]$

These cases illustrate how tighter misrates produce wider bounds.

*Natural sequences* — 4 tests:

- `natural-5`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.1$, expected output: $[1.5, 4.5]$
- `natural-7`: $vx = (1, ..., 7)$, $misrate = 0.05$, expected output: $[2, 6]$
- `natural-10`: $vx = (1, ..., 10)$, $misrate = 0.01$, expected output: $[2.5, 8.5]$
- `natural-20`: $vx = (1, ..., 20)$, $misrate = 0.01$, bounds containing $Center = 10.5$

*Property validation* ($n = 5$, $misrate = 0.1$) — 5 tests:

- `property-identity`: $vx = (0, 2, 4, 6, 8)$, expected output: $[1, 7]$
- `property-centered`: $vx = (-3, -1, 0, 1, 3)$, expected output: $[-2, 2]$
- `property-location-shift`: $vx = (10, 12, 14, 16, 18)$ (= property-identity + 10), expected output: $[11, 17]$ (location equivariance)
- `property-scale-2x`: $vx = (2, 4, 6, 8, 10)$, expected output: $[3, 9]$
- `property-mixed-signs`: $vx = (-2, -1, 0, 1, 2)$, validates bounds crossing zero

*Edge cases* — boundary conditions and extreme scenarios (7 tests):

- `edge-two-elements`: $vx = (1, 3)$, $misrate = 0.5$, expected output: $[1, 3]$
- `edge-three-elements`: $vx = (1, 2, 3)$, $misrate = 0.25$ (small sample)
- `edge-loose-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.5$ (permissive bounds)
- `edge-strict-misrate`: $vx = (1, ..., 10)$, $misrate = 0.01$
- `edge-duplicates-10`: $vx = (5, 5, 5, 5, 5, 5, 5, 5, 5, 5)$, $misrate = 0.01$ (all identical, bounds $= [5, 5]$)
- `edge-negative`: $vx = (-5, -4, -3, -2, -1)$, $misrate = 0.1$ (negative values)
- `edge-wide-range`: $vx = (0.001, 1, 100, 1000, 10000)$, $misrate = 0.1$ (extreme value range)

*Symmetric distributions* (mixed misrates) — 4 tests with symmetric data:

- `symmetric-5`: $vx = (-2, -1, 0, 1, 2)$, $misrate = 0.1$, bounds centered around $0$
- `symmetric-7`: $vx = (-3, -2, -1, 0, 1, 2, 3)$, $misrate = 0.05$, bounds centered around $0$
- `symmetric-10`: $n = 10$ symmetric around $0$, $misrate = 0.01$
- `symmetric-15`: $n = 15$ symmetric around $0$, $misrate = 0.01$

These tests validate that symmetric data produces symmetric bounds around the center.

*Additive distribution* ($misrate = 0.05$) — 2 tests with $Additive(10, 1)$:

- `additive-10`: $n = 10$, seed 0
- `additive-20`: $n = 20$, seed 0

*Uniform distribution* ($misrate = 0.05$) — 2 tests with $Uniform(0, 1)$:

- `uniform-10`: $n = 10$, seed 1
- `uniform-20`: $n = 20$, seed 1

*Misrate variation* ($vx = (1, ..., 10)$) — 4 tests with varying misrates:

- `misrate-1e-1`: $misrate = 0.1$
- `misrate-5e-2`: $misrate = 0.05$
- `misrate-1e-2`: $misrate = 0.01$
- `misrate-5e-3`: $misrate = 0.005$

These tests validate monotonicity: smaller misrates produce wider bounds.

*Unsorted tests* — verify sorting independence (6 tests):

- `unsorted-reverse-5`: $vx = (5, 4, 3, 2, 1)$, must equal `natural-5` output
- `unsorted-reverse-7`: $vx = (7, 6, 5, 4, 3, 2, 1)$, must equal `natural-7` output
- `unsorted-shuffle-5`: $vx$ shuffled, must equal sorted counterpart
- `unsorted-shuffle-7`: $vx$ shuffled, must equal sorted counterpart
- `unsorted-negative-5`: negative values unsorted
- `unsorted-mixed-signs-5`: mixed signs unsorted

These tests validate that $CenterBounds$ produces identical results regardless of input order.

*Error case* — input validation:

- `error-empty-x`: $vx = ()$, $misrate = 0.2$ — empty array violates validity
