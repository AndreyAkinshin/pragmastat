#import "/manual/definitions.typ": *

$ SignedRankMargin(n, misrate) $

The $SignedRankMargin$ test suite contains 37 correctness test cases (4 demo + 6 boundary + 7 exact + 20 medium).

*Demo examples* ($n = 30$) — from manual introduction:

- `demo-1`: $n=30$, $misrate=10^(-6)$, expected output: $46$
- `demo-2`: $n=30$, $misrate=10^(-5)$, expected output: $74$
- `demo-3`: $n=30$, $misrate=10^(-4)$, expected output: $112$
- `demo-4`: $n=30$, $misrate=10^(-3)$, expected output: $158$

These demo cases match the reference values used throughout the manual to illustrate $CenterBounds$ construction.

*Boundary cases* — minimum achievable misrate validation:

- `boundary-n2-min`: $n=2$, minimum misrate for $n=2$ (expected output: $0$)
- `boundary-n3-min`: $n=3$, minimum misrate for $n=3$
- `boundary-n4-min`: $n=4$, minimum misrate for $n=4$
- `boundary-loose`: $n=5$, very loose achievable misrate
- `boundary-tight`: $n=10$, stricter achievable misrate
- `boundary-very-tight`: $n=20$, $misrate=0.001$ (very strict misrate)

These boundary cases validate correct handling of minimum achievable misrate (formula: $2^(1-n)$) and edge conditions.

*Exact computation* ($n <= 10$) — validates dynamic programming path (selected cases shown):

Selected cases cover several achievable small-sample misrates, from loose settings down to the strict end of the exact grid.

These cases exercise the exact Wilcoxon signed-rank CDF computation for small samples where dynamic programming is used.

*Medium samples* ($n in {15, 20, 30, 50, 100}$ × 4 misrates) — 20 tests:

- Misrates range from loose achievable values down to $10^(-4)$.
- Test naming: `medium-n{n}-mr{k}` where $k$ encodes the misrate
- Examples:
  - `medium-n50-mr1e3`: $n=50$, $misrate=0.001$
  - `medium-n100-mr1e4`: $n=100$, $misrate=0.0001$

The medium sample tests validate the transition region between exact computation ($n <= 63$) and approximate computation, ensuring consistent results across sample sizes and misrate values.
