#import "/manual/definitions.typ": *

$ DisparityBounds(vx, vy, misrate) = ShiftBounds(vx, vy, misrate) / AvgSpreadBounds(vx, vy, misrate) $

The $DisparityBounds$ test suite contains 39 test cases (3 demo + 5 natural + 6 property + 5 edge + 5 misrate + 2 distro + 6 unsorted + 7 error).
Since $DisparityBounds$ returns bounds rather than a point estimate, tests validate that the bounds contain $Disparity(vx, vy)$ and satisfy equivariance properties.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.
Because the denominator ($AvgSpreadBounds$) uses randomized $SpreadBounds$, tests fix a `seed` to keep outputs deterministic.

*Demo examples* ($n = m = 30$, $n = m = 20$) --- from manual introduction:

- `demo-1`: $vx = (1, ..., 30)$, $vy = (21, ..., 50)$, $misrate = 0.02$
- `demo-2`: $vx = (1, ..., 30)$, $vy = (21, ..., 50)$, $misrate = 0.005$, wider bounds (tighter misrate)
- `demo-3`: $vx = (1, ..., 20)$, $vy = (5, ..., 24)$

These cases illustrate how tighter misrates produce wider bounds.

*Natural sequences* ($misrate = 0.2$) --- 5 tests:

- `natural-10-10`: $vx = (1, ..., 10)$, $vy = (1, ..., 10)$, bounds containing $0$
- `natural-10-15`: $vx = (1, ..., 10)$, $vy = (1, ..., 15)$
- `natural-15-10`: $vx = (1, ..., 15)$, $vy = (1, ..., 10)$
- `natural-15-15`: $vx = (1, ..., 15)$, $vy = (1, ..., 15)$, bounds containing $0$
- `natural-20-20`: $vx = (1, ..., 20)$, $vy = (1, ..., 20)$, bounds containing $0$

*Property validation* ($n = m = 10$, $misrate = 0.2$) --- 6 tests:

- `property-identity`: $vx = (0, 2, ..., 18)$, $vy = (0, 2, ..., 18)$, bounds must contain $0$
- `property-location-shift`: $vx$ and $vy$ shifted by constant, same bounds as identity (location invariance)
- `property-scale-2x`: $vx$ and $vy$ scaled by 2, same bounds as identity (scale invariance)
- `property-scale-neg`: $vx$ and $vy$ negated, bounds preserved ($"abs"$ scaling)
- `property-symmetry`: $vx = (1, ..., 10)$, $vy = (6, ..., 15)$, observed bounds
- `property-symmetry-swapped`: $vx$ and $vy$ swapped, bounds negated (anti-symmetry)

*Edge cases* --- boundary conditions (5 tests):

- `edge-small`: $n = m = 6$, $misrate = 0.6$ (small samples)
- `edge-negative`: negative values for both samples
- `edge-mixed-signs`: mixed positive/negative values
- `edge-wide-range`: extreme value range
- `edge-asymmetric-10-20`: $n = 10$, $m = 20$ (unbalanced sizes)

*Misrate variation* ($vx = (1, ..., 20)$, $vy = (5, ..., 24)$) --- 5 tests:

- `misrate-2e-1`: $misrate = 0.2$
- `misrate-1e-1`: $misrate = 0.1$
- one intermediate-misrate case between $0.1$ and $0.02$
- `misrate-2e-2`: $misrate = 0.02$
- `misrate-1e-2`: $misrate = 0.01$

These tests validate monotonicity: smaller misrates produce wider bounds.

*Distribution tests* ($misrate$ varies) --- 2 tests:

- `additive-20-20`: $n = m = 20$, $Additive(10, 1)$
- `uniform-20-20`: $n = m = 20$, $Uniform(0, 1)$

*Unsorted tests* --- verify independent sorting of $vx$ and $vy$ (6 tests):

- `unsorted-reverse-x`: X reversed, Y sorted
- `unsorted-reverse-y`: X sorted, Y reversed
- `unsorted-reverse-both`: both reversed
- `unsorted-shuffle-x`: X shuffled, Y sorted
- `unsorted-shuffle-y`: X sorted, Y shuffled
- `unsorted-wide-range`: wide value range, both unsorted

These tests validate that $DisparityBounds$ produces identical results regardless of input order.

*Error cases* --- inputs that violate assumptions (7 tests):

- `error-empty-x`: $vx = ()$ (empty X array)
- `error-empty-y`: $vy = ()$ (empty Y array)
- `error-single-element-x`: $|vx| = 1$ (too few elements for pairing)
- `error-single-element-y`: $|vy| = 1$ (too few elements for pairing)
- `error-constant-x`: constant $vx$ violates sparity ($Spread = 0$)
- `error-constant-y`: constant $vy$ violates sparity ($Spread = 0$)
- `error-misrate-below-min`: misrate below minimum achievable
