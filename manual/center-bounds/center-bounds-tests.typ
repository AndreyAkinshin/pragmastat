#import "/manual/definitions.typ": *

$ CenterBounds(vx, misrate) = [w_((k_"left")), w_((k_"right"))] $

where $vw = { (x_i + x_j) \/ 2 }$ (pairwise averages, sorted) for $i <= j$,
$k_"left" = floor(SignedRankMargin \/ 2) + 1$,
$k_"right" = N - floor(SignedRankMargin \/ 2)$, and $N = n(n+1)\/2$.

The $CenterBounds$ test suite contains 43 test cases (3 demo + 4 natural + 5 property + 7 edge + 4 symmetric + 4 asymmetric + 2 additive + 2 uniform + 4 misrate + 6 unsorted + 2 error cases).
Since $CenterBounds$ returns bounds rather than a point estimate, tests validate that bounds contain $Center(vx)$ and satisfy equivariance properties.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.

*Demo examples* — from manual introduction, validating basic bounds:

- `demo-1`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.1$, expected output: $[1.5, 4.5]$
- `demo-2`: $vx = (1, ..., 10)$, $misrate = 0.01$, expected output: $[2.5, 8.5]$
- `demo-3`: $vx = (0, 2, 4, 6, 8)$, $misrate = 0.1$

These cases illustrate how tighter misrates produce wider bounds.

*Natural sequences* ($n in {5, 7, 10, 20}$, $misrate = 0.01$) — 4 tests:

- `natural-5`: $vx = (1, 2, 3, 4, 5)$, bounds containing $Center = 3$
- `natural-7`: $vx = (1, ..., 7)$, bounds containing $Center = 4$
- `natural-10`: $vx = (1, ..., 10)$, expected output: $[2.5, 8.5]$
- `natural-20`: $vx = (1, ..., 20)$, bounds containing $Center = 10.5$

*Property validation* ($n = 5$, $misrate = 0.05$) — 5 tests:

- `property-identity`: $vx = (1, 2, 3, 4, 5)$, bounds must contain $Center = 3$
- `property-centered`: $vx = (-2, -1, 0, 1, 2)$, bounds must contain $Center = 0$
- `property-location-shift`: $vx = (11, 12, 13, 14, 15)$ (= demo-1 + 10), bounds must be demo-1 bounds + 10
- `property-scale-2x`: $vx = (2, 4, 6, 8, 10)$ (= 2 × demo-1), bounds must be 2× demo-1 bounds
- `property-mixed-signs`: $vx = (-2, -1, 0, 1, 2)$, validates bounds crossing zero

*Edge cases* — boundary conditions and extreme scenarios (7 tests):

- `edge-two-elements`: $vx = (1, 2)$, $misrate = 0.5$ (minimum meaningful sample)
- `edge-three-elements`: $vx = (1, 2, 3)$, $misrate = 0.25$ (small sample)
- `edge-loose-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.5$ (permissive bounds)
- `edge-strict-misrate`: $vx = (1, ..., 10)$, $misrate = 0.002$ (near-minimum misrate for $n=10$)
- `edge-duplicates-10`: $vx = (5, 5, 5, 5, 5, 5, 5, 5, 5, 5)$, $misrate = 0.01$ (all identical, bounds $= [5, 5]$)
- `edge-negative`: $vx = (-5, -4, -3, -2, -1)$, $misrate = 0.05$ (negative values)
- `edge-wide-range`: $vx = (1, 10, 100, 1000, 10000)$, $misrate = 0.1$ (extreme value range)

*Symmetric distributions* ($misrate = 0.05$) — 4 tests with symmetric data:

- `symmetric-5`: $vx = (-2, -1, 0, 1, 2)$, bounds centered around $0$
- `symmetric-7`: $vx = (-3, -2, -1, 0, 1, 2, 3)$, bounds centered around $0$
- `symmetric-10`: $n = 10$ symmetric around $0$
- `symmetric-15`: $n = 15$ symmetric around $0$

These tests validate that symmetric data produces symmetric bounds around the center.

*Asymmetric distributions* ($n = 5$, $misrate = 0.1$) — 4 tests validating bounds with asymmetric data:

- `asymmetric-left-skew`: $vx = (1, 7, 8, 9, 10)$, expected output: $[4, 9.5]$
- `asymmetric-right-skew`: $vx = (1, 2, 3, 4, 10)$, expected output: $[1.5, 7]$
- `asymmetric-bimodal`: $vx = (1, 1, 5, 9, 9)$, expected output: $[1, 9]$
- `asymmetric-outlier`: $vx = (1, 2, 3, 4, 100)$, expected output: $[1.5, 52]$

These tests validate that $CenterBounds$ handles asymmetric data correctly, complementing the symmetric test cases.

*Additive distribution* ($misrate = 0.01$) — 2 tests with $Additive(10, 1)$:

- `additive-10`: $n = 10$, seed 0
- `additive-20`: $n = 20$, seed 0

*Uniform distribution* ($misrate = 0.01$) — 2 tests with $Uniform(0, 1)$:

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

*Error cases* — 2 tests validating input validation:

- `error-single-element`: $vx = (1)$, $misrate = 0.5$ (minimum sample size violation)
- `error-invalid-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.001$ (misrate below minimum achievable)
