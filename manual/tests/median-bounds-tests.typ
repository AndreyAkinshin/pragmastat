#import "/manual/definitions.typ": *

== MedianBounds Tests

$ MedianBounds(vx, misrate) = [x_((k)), x_((n-k+1))] $

where $k$ is the largest integer satisfying $2 dot Pr(B <= k-1) <= misrate$ and $B tilde "Binomial"(n, 0.5)$.

The $MedianBounds$ test suite contains 36 test cases (3 demo + 4 natural + 4 asymmetric + 7 edge + 2 additive + 2 uniform + 3 misrate + 6 unsorted + 3 property + 2 error cases).
Unlike $CenterBounds$, $MedianBounds$ requires no symmetry assumption and uses order statistics directly.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.

*Demo examples* ($n = 5$) — from manual introduction:

- `demo-1`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.1$, expected output: $[1, 5]$
- `demo-2`: $vx = (1, ..., 10)$, $misrate = 0.05$, expected output: $[2, 9]$
- `demo-3`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.05$, expected output: $[1, 5]$

For $n = 5$, the minimum achievable misrate is $2^(1-5) = 0.0625$, so tighter misrates produce identical bounds.

*Natural sequences* ($n in {5, 7, 10, 20}$, $misrate = 0.05$) — 4 tests:

- `natural-5`: $vx = (1, 2, 3, 4, 5)$, bounds containing $Median = 3$
- `natural-7`: $vx = (1, ..., 7)$, bounds containing $Median = 4$
- `natural-10`: $vx = (1, ..., 10)$, expected output: $[2, 9]$
- `natural-20`: $vx = (1, ..., 20)$, bounds containing $Median = 10.5$

*Asymmetric distributions* ($n = 5$, $misrate = 0.1$) — 4 tests validating no symmetry requirement:

- `asymmetric-left-skew`: $vx = (1, 7, 8, 9, 10)$, bounds $[1, 10]$
- `asymmetric-right-skew`: $vx = (1, 2, 3, 4, 10)$, bounds $[1, 10]$
- `asymmetric-bimodal`: $vx = (1, 1, 5, 9, 9)$, bounds $[1, 9]$
- `asymmetric-outlier`: $vx = (1, 2, 3, 4, 100)$, bounds containing median despite outlier

These tests are critical because $MedianBounds$ explicitly does not assume symmetry, unlike $CenterBounds$.
The bounds correctly contain the population median regardless of data distribution shape.

*Edge cases* — boundary conditions (7 tests):

- `edge-two-elements`: $vx = (1, 2)$, $misrate = 0.5$ (minimum meaningful sample)
- `edge-three-elements`: $vx = (1, 2, 3)$, $misrate = 0.5$ (small sample)
- `edge-loose-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.5$ (permissive bounds)
- `edge-strict-misrate`: $vx = (1, ..., 10)$, $misrate = 0.05$
- `edge-duplicates`: $vx = (5, 5, 5, 5, 5)$, $misrate = 0.1$ (all identical, bounds $= [5, 5]$)
- `edge-negative`: $vx = (-5, -4, -3, -2, -1)$, $misrate = 0.1$ (negative values)
- `edge-wide-range`: $vx = (1, 10, 100, 1000)$, $misrate = 0.1$ (extreme value range)

*Additive distribution* ($misrate = 0.05$) — 2 tests with $Additive(10, 1)$:

- `additive-10`: $n = 10$, seed 0
- `additive-20`: $n = 20$, seed 0

*Uniform distribution* ($misrate = 0.05$) — 2 tests with $Uniform(0, 1)$:

- `uniform-10`: $n = 10$, seed 1
- `uniform-20`: $n = 20$, seed 1

*Misrate variation* ($vx = (1, ..., 10)$) — 3 tests:

- `misrate-1e-1`: $misrate = 0.1$
- `misrate-5e-2`: $misrate = 0.05$
- `misrate-1e-2`: $misrate = 0.01$

*Unsorted tests* — verify sorting independence (6 tests):

- `unsorted-reverse-5`: $vx = (5, 4, 3, 2, 1)$, must equal sorted counterpart
- `unsorted-reverse-7`: $vx = (7, 6, ..., 1)$, must equal sorted counterpart
- `unsorted-shuffle-5`: $vx$ shuffled
- `unsorted-shuffle-7`: $vx$ shuffled
- `unsorted-negative-5`: negative values unsorted
- `unsorted-mixed-signs-5`: mixed signs unsorted

These tests validate that $MedianBounds$ produces identical results regardless of input order, since the bounds use order statistics that require sorting.

*Property validation* ($n = 5$, $misrate = 0.1$) — 3 tests:

- `property-identity`: $vx = (1, 2, 3, 4, 5)$, bounds must contain $Median = 3$
- `property-location-shift`: $vx = (11, 12, 13, 14, 15)$ (= base + 10), bounds must be base bounds + 10
- `property-scale-2x`: $vx = (2, 4, 6, 8, 10)$ (= 2 × base), bounds must be 2× base bounds

*Error cases* — 2 tests validating input validation:

- `error-single-element`: $vx = (1)$, $misrate = 0.5$ (minimum sample size violation)
- `error-invalid-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.001$ (misrate below minimum achievable)
