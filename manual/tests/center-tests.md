## Center Tests

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

The $\Center$ test suite contains 38 correctness test cases stored in the repository (24 original + 14 unsorted), plus 1 performance test that should be implemented manually (see §Test Framework).

**Demo examples** ($n = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (0, 2, 4, 6, 8)$, expected output: $4$ (base case)
- `demo-2`: $\x = (10, 12, 14, 16, 18)$ (= demo-1 + 10), expected output: $14$ (location equivariance)
- `demo-3`: $\x = (0, 6, 12, 18, 24)$ (= 3 × demo-1), expected output: $12$ (scale equivariance)

**Natural sequences** ($n = 1, 2, 3, 4$) — canonical happy path examples:

- `natural-1`: $\x = (1)$, expected output: $1$
- `natural-2`: $\x = (1, 2)$, expected output: $1.5$
- `natural-3`: $\x = (1, 2, 3)$, expected output: $2$
- `natural-4`: $\x = (1, 2, 3, 4)$, expected output: $2.5$ (smallest even size with rich structure)

**Negative values** ($n = 3$) — sign handling validation:

- `negative-3`: $\x = (-3, -2, -1)$, expected output: $-2$

**Zero values** ($n = 1, 2$) — edge case testing with zeros:

- `zeros-1`: $\x = (0)$, expected output: $0$
- `zeros-2`: $\x = (0, 0)$, expected output: $0$

**Additive distribution** ($n = 5, 10, 30$) — fuzzy testing with $\Additive(10, 1)$:

- `additive-5`, `additive-10`, `additive-30`: random samples generated with seed 0

**Uniform distribution** ($n = 5, 100$) — fuzzy testing with $\Uniform(0, 1)$:

- `uniform-5`, `uniform-100`: random samples generated with seed 1

The random samples validate that $\Center$ performs correctly on realistic distributions at various sample sizes.
The progression from small ($n = 5$) to large ($n = 100$) samples helps identify issues that only manifest at specific scales.

**Algorithm stress tests** — edge cases for fast algorithm implementation:

- `duplicates-5`: $\x = (3, 3, 3, 3, 3)$ (all identical, stress stall handling)
- `duplicates-10`: $\x = (1, 1, 1, 2, 2, 2, 3, 3, 3, 3)$ (many duplicates, stress tie-breaking)
- `parity-odd-7`: $\x = (1, 2, 3, 4, 5, 6, 7)$ (odd sample size for odd total pairs)
- `parity-even-6`: $\x = (1, 2, 3, 4, 5, 6)$ (even sample size for even total pairs)
- `parity-odd-49`: 49-element sequence $(1, 2, \ldots, 49)$ (large odd, 1225 pairs)
- `parity-even-50`: 50-element sequence $(1, 2, \ldots, 50)$ (large even, 1275 pairs)

**Extreme values** — numerical stability and range tests:

- `extreme-large-5`: $\x = (1e8, 2e8, 3e8, 4e8, 5e8)$ (very large values)
- `extreme-small-5`: $\x = (1e{-}8, 2e{-}8, 3e{-}8, 4e{-}8, 5e{-}8)$ (very small positive values)
- `extreme-wide-5`: $\x = (0.001, 1, 100, 1000, 1000000)$ (wide range, tests precision)

**Unsorted tests** — verify sorting correctness (14 tests):

- `unsorted-reverse-{n}` for $n \in \{2, 3, 4, 5, 7\}$: reverse sorted natural sequences (5 tests)
- `unsorted-shuffle-3`: $\x = (2, 1, 3)$ (middle element first)
- `unsorted-shuffle-4`: $\x = (3, 1, 4, 2)$ (interleaved)
- `unsorted-shuffle-5`: $\x = (5, 2, 4, 1, 3)$ (complex shuffle)
- `unsorted-last-first-5`: $\x = (5, 1, 2, 3, 4)$ (last moved to first)
- `unsorted-first-last-5`: $\x = (2, 3, 4, 5, 1)$ (first moved to last)
- `unsorted-duplicates-mixed-5`: $\x = (3, 3, 3, 3, 3)$ (all identical, any order)
- `unsorted-duplicates-unsorted-10`: $\x = (3, 1, 2, 3, 1, 3, 2, 1, 3, 2)$ (duplicates mixed)
- `unsorted-extreme-large-unsorted-5`: $\x = (5e8, 1e8, 4e8, 2e8, 3e8)$ (large values unsorted)
- `unsorted-parity-odd-reverse-7`: $\x = (7, 6, 5, 4, 3, 2, 1)$ (odd size reverse)

These tests ensure implementations correctly sort input data before computing pairwise averages.
The variety of shuffle patterns (reverse, rotation, interleaving, single element displacement) catches common sorting bugs.

**Performance test** — validates the fast $O(n \log n)$ algorithm:

- **Input**: $\x = (1, 2, 3, \ldots, 100000)$
- **Expected output**: $50000.5$
- **Time constraint**: Must complete in under 5 seconds
- **Purpose**: Ensures that the implementation uses the efficient algorithm rather than materializing all $\binom{n+1}{2} \approx 5$ billion pairwise averages

This test case is not stored in the repository because it generates a large JSON file (approximately 1.5 MB).
Each language implementation should manually implement this test with the hardcoded expected result.
