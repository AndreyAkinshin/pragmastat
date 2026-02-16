#import "/manual/definitions.typ": *

$ SignedRankMargin(n, misrate) $

The $SignedRankMargin$ test suite contains 39 correctness test cases (4 demo + 6 boundary + 7 exact + 20 medium + 2 error).

*Demo examples* ($n = 30$) — from manual introduction:

- `demo-1`: $n=30$, $misrate=10^(-6)$, expected output: $46$
- `demo-2`: $n=30$, $misrate=10^(-5)$, expected output: $74$
- `demo-3`: $n=30$, $misrate=10^(-4)$, expected output: $112$
- `demo-4`: $n=30$, $misrate=10^(-3)$, expected output: $158$

These demo cases match the reference values used throughout the manual to illustrate $CenterBounds$ construction.

*Boundary cases* — minimum achievable misrate validation:

- `boundary-n2-min`: $n=2$, $misrate=0.5$ (minimum misrate for $n=2$, expected output: $0$)
- `boundary-n3-min`: $n=3$, $misrate=0.25$ (minimum misrate for $n=3$)
- `boundary-n4-min`: $n=4$, $misrate=0.125$ (minimum misrate for $n=4$)
- `boundary-loose`: $n=5$, $misrate=0.5$ (permissive misrate)
- `boundary-tight`: $n=10$, $misrate=0.01$ (strict misrate)
- `boundary-very-tight`: $n=20$, $misrate=0.001$ (very strict misrate)

These boundary cases validate correct handling of minimum achievable misrate (formula: $2^(1-n)$) and edge conditions.

*Exact computation* ($n <= 10$) — validates dynamic programming path:

- `exact-n5-mr1e1`: $n=5$, $misrate=0.1$
- `exact-n6-mr1e1`: $n=6$, $misrate=0.1$
- `exact-n6-mr5e2`: $n=6$, $misrate=0.05$
- `exact-n10-mr1e1`: $n=10$, $misrate=0.1$, expected output: $22$
- `exact-n10-mr1e2`: $n=10$, $misrate=0.01$
- `exact-n10-mr5e2`: $n=10$, $misrate=0.05$
- `exact-n10-mr5e3`: $n=10$, $misrate=0.005$

These cases exercise the exact Wilcoxon signed-rank CDF computation for small samples where dynamic programming is used.

*Medium samples* ($n in {15, 20, 30, 50, 100}$ × 4 misrates) — 20 tests:

- Misrate values: $misrate in {10^(-1), 10^(-2), 10^(-3), 10^(-4)}$
- Test naming: `medium-n{n}-mr{k}` where $k$ encodes the misrate
- Examples:
  - `medium-n15-mr1e1`: $n=15$, $misrate=0.1$
  - `medium-n30-mr1e2`: $n=30$, $misrate=0.01$, expected output: $220$
  - `medium-n50-mr1e3`: $n=50$, $misrate=0.001$
  - `medium-n100-mr1e4`: $n=100$, $misrate=0.0001$

The medium sample tests validate the transition region between exact computation ($n <= 63$) and approximate computation, ensuring consistent results across sample sizes and misrate values.

*Error case* — domain violation:

- `error-n1`: $n=1$, $misrate=0.5$ (invalid: misrate below minimum achievable $2^(1-1) = 1.0$)
- `error-n0`: $n=0$, $misrate=0.05$ (invalid: n must be positive)

This error case verifies that implementations correctly reject $n=1$ with $misrate=0.5$ as invalid input, since the minimum achievable misrate for $n=1$ is $2^0 = 1.0$.
