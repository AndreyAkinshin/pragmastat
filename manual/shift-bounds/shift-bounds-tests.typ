#import "/manual/definitions.typ": *

$ ShiftBounds(vx, vy, misrate) = [z_((k_"left")), z_((k_"right"))] $

where

$ vz = { x_i - y_j }_(1 <= i <= n, 1 <= j <= m) quad ("sorted") $

$ k_"left" = floor(PairwiseMargin(n, m, misrate) / 2) + 1 $

$ k_"right" = n m - floor(PairwiseMargin(n, m, misrate) / 2) $

The $ShiftBounds$ test suite contains 61 correctness test cases (3 demo + 9 natural + 6 property + 10 edge + 9 additive + 4 uniform + 5 misrate + 15 unsorted).
Since $ShiftBounds$ returns bounds rather than a point estimate, tests validate that the bounds contain $Shift(vx, vy)$ and satisfy equivariance properties.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.
The domain constraint $misrate >= 2 / binom(n+m, n)$ is enforced; inputs violating this return a domain error.

*Demo examples* ($n = m = 5$) — from manual introduction, validating basic bounds:

- `demo-1`: $vx = (1, 2, 3, 4, 5)$, $vy = (3, 4, 5, 6, 7)$, $misrate = 0.05$, expected output: $[-4, 0]$
- `demo-2`: $vx = (1, 2, 3, 4, 5)$, $vy = (3, 4, 5, 6, 7)$, $misrate = 0.01$, expected output: $[-5, 1]$
- `demo-3`: $vx = (3, 4, 5, 6, 7)$, $vy = (3, 4, 5, 6, 7)$, $misrate = 0.05$, expected output: bounds containing $0$ (identity case)

These cases illustrate how tighter misrates produce wider bounds and validate the identity property where identical samples yield bounds containing zero.

*Natural sequences* ($[n, m] in {5, 8, 10} times {5, 8, 10}$, $misrate = 10^(-2)$) — 9 combinations:

- `natural-5-5`: $vx = (1, ..., 5)$, $vy = (1, ..., 5)$, expected bounds containing $0$
- `natural-5-8`: $vx = (1, ..., 5)$, $vy = (1, ..., 8)$
- `natural-5-10`: $vx = (1, ..., 5)$, $vy = (1, ..., 10)$
- `natural-8-5`: $vx = (1, ..., 8)$, $vy = (1, ..., 5)$
- `natural-8-8`: $vx = (1, ..., 8)$, $vy = (1, ..., 8)$, expected bounds containing $0$
- `natural-8-10`: $vx = (1, ..., 8)$, $vy = (1, ..., 10)$
- `natural-10-5`: $vx = (1, ..., 10)$, $vy = (1, ..., 5)$
- `natural-10-8`: $vx = (1, ..., 10)$, $vy = (1, ..., 8)$
- `natural-10-10`: $vx = (1, ..., 10)$, $vy = (1, ..., 10)$, expected bounds containing $0$

These sizes are chosen to satisfy $misrate >= 2 / binom(n+m, n)$ for all combinations.

*Property validation* ($n = m = 10$, $misrate = 10^(-3)$) — 6 tests:

- `property-identity`: $vx = (0, 2, 4, ..., 18)$, $vy = (0, 2, 4, ..., 18)$, bounds must contain $0$
- `property-location-shift`: $vx = (7, 9, 11, ..., 25)$, $vy = (13, 15, 17, ..., 31)$
  - Must produce same bounds as base case (location invariance)
- `property-scale-2x`: $vx = (2, 4, 6, ..., 20)$, $vy = (6, 8, 10, ..., 24)$
  - Bounds must be 2× the base case bounds (scale equivariance)
- `property-antisymmetry`: $vx = (3, 4, ..., 12)$, $vy = (1, 2, ..., 10)$
  - Bounds must be negated: if original is $[a, b]$, this yields $[-b, -a]$
- `property-negative`: $vx = (-10, -9, ..., -1)$, $vy = (-12, -11, ..., -3)$
  - Validates sign handling with all negative values
- `property-mixed-signs`: $vx = (-4, -3, ..., 5)$, $vy = (-3, -2, ..., 6)$
  - Validates bounds crossing zero with mixed-sign samples

*Edge cases* — boundary conditions and extreme scenarios (10 tests):

- `edge-min-samples`: $vx = (1, 2, 3, 4, 5)$, $vy = (6, 7, 8, 9, 10)$, $misrate = 0.05$
- `edge-permissive-misrate`: $vx = (1, 2, 3, 4, 5)$, $vy = (3, 4, 5, 6, 7)$, $misrate = 0.5$ (very wide bounds)
- `edge-strict-misrate`: $n = m = 20$, $misrate = 10^(-6)$ (very narrow bounds)
- `edge-zero-shift`: $n = m = 10$, all values $= 5$, $misrate = 10^(-3)$ (bounds around 0)
- `edge-asymmetric-3-100`: $n = 3$, $m = 100$, $misrate = 10^(-2)$ (extreme size difference)
- `edge-asymmetric-5-50`: $n = 5$, $m = 50$, $misrate = 10^(-3)$ (highly unbalanced)
- `edge-duplicates`: $vx = (3, 3, 3, 3, 3)$, $vy = (5, 5, 5, 5, 5)$, $misrate = 10^(-2)$ (all duplicates, bounds around -2)
- `edge-wide-range`: $n = m = 10$, values spanning $10^(-3)$ to $10^8$, $misrate = 10^(-3)$ (extreme value range)
- `edge-tiny-values`: $n = m = 10$, values $approx 10^(-8)$, $misrate = 10^(-3)$ (numerical precision)
- `edge-large-values`: $n = m = 10$, values $approx 10^8$, $misrate = 10^(-3)$ (large magnitude)

These edge cases stress-test boundary conditions, numerical stability, and the margin calculation with extreme parameters.

*Additive distribution* ($[n, m] in {10, 30, 50} times {10, 30, 50}$, $misrate = 10^(-3)$) — 9 combinations with $Additive(10, 1)$:

- `additive-10-10`, `additive-10-30`, `additive-10-50`
- `additive-30-10`, `additive-30-30`, `additive-30-50`
- `additive-50-10`, `additive-50-30`, `additive-50-50`
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

These fuzzy tests validate that bounds properly encompass the shift estimate for realistic normally-distributed data at various sample sizes.

*Uniform distribution* ($[n, m] in {10, 100} times {10, 100}$, $misrate = 10^(-4)$) — 4 combinations with $Uniform(0, 1)$:

- `uniform-10-10`, `uniform-10-100`, `uniform-100-10`, `uniform-100-100`
- Random generation: $vx$ uses seed 2, $vy$ uses seed 3

The asymmetric size combinations are particularly important for testing margin calculation with unbalanced samples.

*Misrate variation* ($n = m = 20$, $vx = (0, 2, 4, ..., 38)$, $vy = (10, 12, 14, ..., 48)$) — 5 tests with varying misrates:

- `misrate-1e-2`: $misrate = 10^(-2)$
- `misrate-1e-3`: $misrate = 10^(-3)$
- `misrate-1e-4`: $misrate = 10^(-4)$
- `misrate-1e-5`: $misrate = 10^(-5)$
- `misrate-1e-6`: $misrate = 10^(-6)$

These tests use identical samples with varying misrates to validate the monotonicity property: smaller misrates (higher confidence) produce wider bounds.
The sequence demonstrates how bound width increases as misrate decreases, helping implementations verify correct margin calculation.

*Unsorted tests* — verify independent sorting of $vx$ and $vy$ (15 tests):

- `unsorted-x-natural-5-5`: $vx = (5, 3, 1, 4, 2)$, $vy = (1, 2, 3, 4, 5)$, $misrate = 10^(-2)$ (X reversed, Y sorted)
- `unsorted-y-natural-5-5`: $vx = (1, 2, 3, 4, 5)$, $vy = (5, 3, 1, 4, 2)$, $misrate = 10^(-2)$ (X sorted, Y reversed)
- `unsorted-both-natural-5-5`: $vx = (5, 3, 1, 4, 2)$, $vy = (5, 3, 1, 4, 2)$, $misrate = 10^(-2)$ (both reversed)
- `unsorted-x-shuffle-5-5`: $vx = (3, 1, 5, 4, 2)$, $vy = (1, 2, 3, 4, 5)$, $misrate = 10^(-2)$ (X shuffled)
- `unsorted-y-shuffle-5-5`: $vx = (1, 2, 3, 4, 5)$, $vy = (4, 2, 5, 1, 3)$, $misrate = 10^(-2)$ (Y shuffled)
- `unsorted-both-shuffle-5-5`: $vx = (3, 1, 5, 4, 2)$, $vy = (2, 4, 1, 5, 3)$, $misrate = 10^(-2)$ (both shuffled)
- `unsorted-demo-unsorted-x`: $vx = (5, 1, 4, 2, 3)$, $vy = (3, 4, 5, 6, 7)$, $misrate = 0.05$ (demo-1 X unsorted)
- `unsorted-demo-unsorted-y`: $vx = (1, 2, 3, 4, 5)$, $vy = (7, 3, 6, 4, 5)$, $misrate = 0.05$ (demo-1 Y unsorted)
- `unsorted-demo-both-unsorted`: $vx = (4, 1, 5, 2, 3)$, $vy = (6, 3, 7, 4, 5)$, $misrate = 0.05$ (demo-1 both unsorted)
- `unsorted-identity-unsorted`: $vx = (4, 1, 5, 2, 3)$, $vy = (5, 1, 4, 3, 2)$, $misrate = 10^(-2)$ (identity property, both unsorted)
- `unsorted-negative-unsorted`: $vx = (-1, -5, -3, -2, -4)$, $vy = (-2, -4, -3, -5, -1)$, $misrate = 10^(-2)$ (negative values unsorted)
- `unsorted-asymmetric-5-10`: $vx = (2, 5, 1, 3, 4)$, $vy = (10, 5, 2, 8, 4, 1, 9, 3, 7, 6)$, $misrate = 10^(-2)$ (asymmetric sizes, both unsorted)
- `unsorted-duplicates`: $vx = (3, 3, 3, 3, 3)$, $vy = (5, 5, 5, 5, 5)$, $misrate = 10^(-2)$ (all duplicates, any order)
- `unsorted-mixed-duplicates-x`: $vx = (2, 1, 3, 2, 1)$, $vy = (1, 1, 2, 2, 3)$, $misrate = 10^(-2)$ (X has unsorted duplicates)
- `unsorted-mixed-duplicates-y`: $vx = (1, 1, 2, 2, 3)$, $vy = (3, 2, 1, 3, 2)$, $misrate = 10^(-2)$ (Y has unsorted duplicates)

These unsorted tests are critical because $ShiftBounds$ computes bounds from pairwise differences, requiring both samples to be sorted independently.
The variety ensures implementations don't incorrectly assume pre-sorted input or sort samples together.
Each test must produce identical output to its sorted counterpart, validating that the implementation correctly handles the sorting step.

*No performance test* — $ShiftBounds$ uses the $"FastShift"$ algorithm internally, which is already validated by the $Shift$ performance test.
Since bounds computation involves only two quantile calculations from the pairwise differences (at positions determined by $PairwiseMargin$),
the performance characteristics are equivalent to computing two $Shift$ estimates, which completes efficiently for large samples.
