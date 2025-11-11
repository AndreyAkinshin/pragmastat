## Spread Tests

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

The $\Spread$ test suite contains 38 correctness test cases stored in the repository (24 original + 14 unsorted), plus 1 performance test that should be implemented manually (see §Test Framework).

**Demo examples** ($n = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (0, 2, 4, 6, 8)$, expected output: $4$ (base case)
- `demo-2`: $\x = (10, 12, 14, 16, 18)$ (= demo-1 + 10), expected output: $4$ (location invariance)
- `demo-3`: $\x = (0, 4, 8, 12, 16)$ (= 2 × demo-1), expected output: $8$ (scale equivariance)

**Natural sequences** ($n = 1, 2, 3, 4$):

- `natural-1`: $\x = (1)$, expected output: $0$ (single element has zero dispersion)
- `natural-2`: $\x = (1, 2)$, expected output: $1$
- `natural-3`: $\x = (1, 2, 3)$, expected output: $1$
- `natural-4`: $\x = (1, 2, 3, 4)$, expected output: $1.5$ (smallest even size with rich structure)

**Negative values** ($n = 3$) — sign handling validation:

- `negative-3`: $\x = (-3, -2, -1)$, expected output: $1$

**Zero values** ($n = 1, 2$):

- `zeros-1`: $\x = (0)$, expected output: $0$
- `zeros-2`: $\x = (0, 0)$, expected output: $0$

**Additive distribution** ($n = 5, 10, 30$) — $\Additive(10, 1)$:

- `additive-5`, `additive-10`, `additive-30`: random samples generated with seed 0

**Uniform distribution** ($n = 5, 100$) — $\Uniform(0, 1)$:

- `uniform-5`, `uniform-100`: random samples generated with seed 1

The natural sequence cases validate the basic pairwise difference calculation.
The zero cases confirm that constant samples correctly produce zero spread.

**Algorithm stress tests** — edge cases for fast algorithm implementation:

- `duplicates-5`: $\x = (3, 3, 3, 3, 3)$ (all identical, expected output: $0$)
- `duplicates-10`: $\x = (1, 1, 1, 2, 2, 2, 3, 3, 3, 3)$ (many duplicates, stress tie-breaking)
- `parity-odd-7`: $\x = (1, 2, 3, 4, 5, 6, 7)$ (odd sample size, 21 differences)
- `parity-even-6`: $\x = (1, 2, 3, 4, 5, 6)$ (even sample size, 15 differences)
- `parity-odd-49`: 49-element sequence $(1, 2, \ldots, 49)$ (large odd, 1176 differences)
- `parity-even-50`: 50-element sequence $(1, 2, \ldots, 50)$ (large even, 1225 differences)

**Extreme values** — numerical stability and range tests:

- `extreme-large-5`: $\x = (1e8, 2e8, 3e8, 4e8, 5e8)$ (very large values)
- `extreme-small-5`: $\x = (1e{-}8, 2e{-}8, 3e{-}8, 4e{-}8, 5e{-}8)$ (very small positive values)
- `extreme-wide-5`: $\x = (0.001, 1, 100, 1000, 1000000)$ (wide range, tests precision)

**Unsorted tests** — verify sorting correctness (14 tests):

- `unsorted-reverse-{n}` for $n \in \{2, 3, 4, 5, 7\}$: reverse sorted natural sequences (5 tests)
- `unsorted-shuffle-3`: $\x = (3, 1, 2)$ (rotated)
- `unsorted-shuffle-4`: $\x = (4, 2, 1, 3)$ (mixed order)
- `unsorted-shuffle-5`: $\x = (5, 1, 3, 2, 4)$ (partial shuffle)
- `unsorted-last-first-5`: $\x = (5, 1, 2, 3, 4)$ (last moved to first)
- `unsorted-first-last-5`: $\x = (2, 3, 4, 5, 1)$ (first moved to last)
- `unsorted-duplicates-mixed-5`: $\x = (3, 3, 3, 3, 3)$ (all identical)
- `unsorted-duplicates-unsorted-10`: $\x = (2, 3, 1, 3, 2, 1, 2, 3, 1, 3)$ (duplicates mixed)
- `unsorted-extreme-wide-unsorted-5`: $\x = (1000, 0.001, 1000000, 100, 1)$ (wide range unsorted)
- `unsorted-negative-unsorted-5`: $\x = (-1, -5, -2, -4, -3)$ (negative unsorted)

These tests verify that implementations correctly sort input before computing pairwise differences.
Since $\Spread$ uses absolute differences, order-dependent bugs would manifest differently than in $\Center$.

**Performance test** — validates the fast $O(n \log n)$ algorithm:

- **Input**: $\x = (1, 2, 3, \ldots, 100000)$
- **Expected output**: $29290$
- **Time constraint**: Must complete in under 5 seconds
- **Purpose**: Ensures that the implementation uses the efficient algorithm rather than materializing all $\binom{n}{2} \approx 5$ billion pairwise differences

This test case is not stored in the repository because it generates a large JSON file (approximately 1.5 MB).
Each language implementation should manually implement this test with the hardcoded expected result.
