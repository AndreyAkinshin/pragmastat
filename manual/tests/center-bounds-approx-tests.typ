#import "/manual/definitions.typ": *

== CenterBoundsApprox Tests

$ CenterBoundsApprox(vx, misrate, "seed") = [q_(alpha\/2), q_(1-alpha\/2)] $

where $q_p$ is the $p$-th quantile of bootstrap $Center$ estimates.

The $CenterBoundsApprox$ test suite contains 32 test cases (2 demo + 3 natural + 4 asymmetric + 5 edge + 3 additive + 3 uniform + 2 misrate + 5 property + 5 error).
Unlike exact methods, $CenterBoundsApprox$ uses bootstrap resampling and requires a seed for reproducibility.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.

*Demo examples* ($n = 5$) — from manual introduction:

- `demo-1`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.1$, seed = "demo-seed-1", expected output: $[2, 4]$
- `demo-3`: $vx = (0, 2, 4, 6, 8)$, $misrate = 0.1$, seed = "demo-seed-3"

The seed ensures cross-language determinism — all implementations must produce identical results for the same seed.

*Natural sequences* — 3 tests:

- `natural-5`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.1$, seed = "natural-5"
- `natural-10`: $vx = (1, ..., 10)$, $misrate = 0.05$, seed = "natural-10", expected output: $[3.5, 7.5]$
- `natural-20`: $vx = (1, ..., 20)$, $misrate = 0.05$, seed = "natural-20"

*Asymmetric distributions* ($n = 5$, $misrate = 0.1$) — 4 tests validating bootstrap with asymmetric data:

- `asymmetric-left-skew`: $vx = (1, 7, 8, 9, 10)$, seed = "asym-left"
- `asymmetric-right-skew`: $vx = (1, 2, 3, 4, 10)$, seed = "asym-right"
- `asymmetric-bimodal`: $vx = (1, 1, 5, 9, 9)$, seed = "asym-bimodal"
- `asymmetric-outlier`: $vx = (1, 2, 3, 4, 100)$, seed = "asym-outlier"

These tests validate that $CenterBoundsApprox$ handles asymmetric data correctly, which is its primary use case when $CenterBounds$ (requiring symmetry) cannot be used.

*Edge cases* — boundary conditions (5 tests):

- `edge-two-elements`: $vx = (1, 2)$, $misrate = 0.1$, seed = "edge-two" (minimum sample)
- `edge-three-elements`: $vx = (1, 2, 3)$, $misrate = 0.1$, seed = "edge-three"
- `edge-loose-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.5$, seed = "edge-loose"
- `edge-duplicates`: $vx = (5, 5, 5, 5, 5)$, $misrate = 0.1$, seed = "edge-dup" (bounds $= [5, 5]$)
- `edge-negative`: $vx = (-5, -4, -3, -2, -1)$, $misrate = 0.1$, seed = "edge-neg"

*Additive distribution* — 3 tests with $Additive(10, 1)$:

- `additive-5`: $n = 5$, $misrate = 0.1$, seed = "additive-5"
- `additive-10`: $n = 10$, $misrate = 0.05$, seed = "additive-10"
- `additive-20`: $n = 20$, $misrate = 0.05$, seed = "additive-20"

*Uniform distribution* — 3 tests with $Uniform(0, 1)$:

- `uniform-5`: $n = 5$, $misrate = 0.1$, seed = "uniform-5"
- `uniform-10`: $n = 10$, $misrate = 0.05$, seed = "uniform-10"
- `uniform-20`: $n = 20$, $misrate = 0.05$, seed = "uniform-20"

*Misrate variation* ($vx = (0, 2, 4, 6, 8, 10)$) — 2 tests:

- `misrate-1e-1`: $misrate = 0.1$, seed = "misrate-1"
- `misrate-5e-2`: $misrate = 0.05$, seed = "misrate-2"

These tests validate that smaller misrates produce wider bounds within the resolution of bootstrap quantiles.

*Property tests* — 5 tests:

- `property-base`: Verifies bounds contain the center point estimate
- `property-determinism`: Same input and seed produce identical bounds
- `property-permutation-invariance`: Results are invariant to input ordering
- `property-location-shift`: $CenterBoundsApprox(vx + k) = CenterBoundsApprox(vx) + k$
- `property-scale-2x`: $CenterBoundsApprox(2 dot vx) = 2 dot CenterBoundsApprox(vx)$

*Error cases* — 5 tests validating input validation:

- `error-single-element`: $vx = (1)$, $misrate = 0.5$ (minimum sample size violation)
- `error-invalid-misrate`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.00001$ (misrate below minimum achievable)
- `demo-2`: $vx = (1, 2, 3, 4, 5)$, $misrate = 0.01$ (misrate below $2^(1-n) = 0.0625$ for $n = 5$)
- `misrate-1e-2`: $vx = (0, 2, 4, 6, 8, 10)$, $misrate = 0.01$ (misrate below $2^(1-n) = 0.03125$ for $n = 6$)
- `misrate-1e-3`: $vx = (0, 2, 4, 6, 8, 10)$, $misrate = 0.001$ (misrate below $2^(1-n) = 0.03125$ for $n = 6$)

*No unsorted tests* — $CenterBoundsApprox$ sorts input internally for permutation invariance, making unsorted tests redundant.
The implementation must produce identical results regardless of input order by design.

*Seed determinism* — all tests specify explicit seeds to ensure:

- Cross-language reproducibility (same seed → same results in all implementations)
- Test stability (results don't change between runs)
- Debugging support (failed tests can be reproduced exactly)

The default seed "center-bounds-approx" is used when no seed is specified, ensuring consistent behavior across the toolkit.
