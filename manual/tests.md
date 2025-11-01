# Reference Tests

## Motivation

The toolkit maintains seven implementations across different programming languages: Python, TypeScript, R, C#, Kotlin, Rust, and Go.
Each implementation must produce identical numerical results for all estimators.
Maintaining correctness across this diverse set of languages requires a rigorous reference test suite.

Reference tests serve three critical purposes:

- **Cross-language validation.** All implementations must pass identical test cases, ensuring consistent behavior regardless of language choice.
- **Regression prevention.** Changes to any implementation can be validated against the reference outputs to detect unintended modifications.
- **Implementation guidance.** The test cases provide concrete examples that guide developers implementing the toolkit in new languages.

The test design follows established quality assurance principles:

- **Minimal sufficiency.** The test set should be as small as possible while still providing high confidence in correctness.
  Smaller test suites reduce CI execution time and simplify maintenance.
- **Comprehensive coverage.** Tests must cover both typical use cases and edge cases that expose potential implementation errors.
- **Deterministic reproducibility.** All random test cases use fixed seeds to ensure identical results across all platforms and implementations.

The test suite balances three categories:

- **Canonical cases** use deterministic, easily verified inputs like natural number sequences.
  These provide intuitive examples where correct outputs can be validated by inspection.
- **Edge cases** test boundary conditions such as single-element samples, zero values, and minimum viable sample sizes.
  These expose off-by-one errors, division by zero, and other common implementation mistakes.
- **Fuzzy tests** use controlled random number generation to explore the input space beyond hand-crafted examples.
  Random tests catch issues that might not be apparent from simple deterministic cases.

The C# implementation serves as the reference generator.
All test cases are defined programmatically, executed to produce expected outputs, and serialized to JSON format.
Other implementations load these JSON files and verify their estimators produce matching results within numerical tolerance.

## Center

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

The $\Center$ test suite contains 39 correctness test cases stored in the repository (24 original + 15 unsorted), plus 1 performance test that should be implemented manually (see §Test Framework).

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

**Unsorted tests** — verify sorting correctness (15 tests):

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

## Spread

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

The $\Spread$ test suite contains 39 correctness test cases stored in the repository (24 original + 15 unsorted), plus 1 performance test that should be implemented manually (see §Test Framework).

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

**Unsorted tests** — verify sorting correctness (15 tests):

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

## RelSpread

$$
\RelSpread(\x) = \frac{\Spread(\x)}{\left| \Center(\x) \right|}
$$

The $\RelSpread$ test suite contains 25 test cases (15 original + 10 unsorted) focusing on relative dispersion.

**Demo examples** ($n = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (0, 2, 4, 6, 8)$, expected output: $1$ (base case)
- `demo-2`: $\x = (0, 10, 20, 30, 40)$ (= 5 × demo-1), expected output: $1$ (scale invariance)

**Natural sequences** ($n = 1, 2, 3, 4$):

- `natural-1`: $\x = (1)$, expected output: $0$
- `natural-2`: $\x = (1, 2)$, expected output: $\approx 0.667$
- `natural-3`: $\x = (1, 2, 3)$, expected output: $0.5$
- `natural-4`: $\x = (1, 2, 3, 4)$, expected output: $0.6$ (validates composite with even size)

**Negative values** ($n = 3$) — validates absolute value in denominator:

- `negative-3`: $\x = (-3, -2, -1)$, expected output: $0.5$

**Uniform distribution** ($n = 5, 10, 20, 30, 100$) — $\Uniform(0, 1)$:

- `uniform-5`, `uniform-10`, `uniform-20`, `uniform-30`, `uniform-100`: random samples generated with seed 0

The uniform distribution tests span multiple sample sizes to verify that $\RelSpread$ correctly normalizes dispersion.
The absence of zero-value tests reflects the domain constraint requiring $\Center(\x) \neq 0$.

**Composite estimator stress tests** — edge cases specific to division operation:

- `composite-small-center`: $\x = (0.001, 0.002, 0.003, 0.004, 0.005)$ (small center, tests division stability)
- `composite-large-spread`: $\x = (1, 100, 200, 300, 1000)$ (large spread relative to center)
- `composite-extreme-ratio`: $\x = (1, 1.0001, 1.0002, 1.0003, 1.0004)$ (tiny spread, tests precision)

**Unsorted tests** — verify sorting for composite estimator (10 tests):

- `unsorted-reverse-{n}` for $n \in \{3, 4, 5\}$: reverse sorted natural sequences (3 tests)
- `unsorted-shuffle-4`: $\x = (4, 1, 3, 2)$ (mixed order)
- `unsorted-shuffle-5`: $\x = (5, 3, 1, 4, 2)$ (complex shuffle)
- `unsorted-negative-unsorted-3`: $\x = (-1, -3, -2)$ (negative unsorted)
- `unsorted-demo-unsorted-5`: $\x = (8, 0, 4, 2, 6)$ (demo case unsorted)
- `unsorted-composite-small-unsorted`: $\x = (0.005, 0.001, 0.003, 0.002, 0.004)$ (small center unsorted)
- `unsorted-composite-large-unsorted`: $\x = (1000, 1, 300, 100, 200)$ (large spread unsorted)
- `unsorted-extreme-ratio-unsorted-4`: $\x = (1.0003, 1, 1.0002, 1.0001)$ (extreme ratio unsorted)

Since $\RelSpread$ combines both $\Center$ and $\Spread$, these tests verify that sorting works correctly for composite estimators.

## Shift

$$
\Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

The $\Shift$ test suite contains 60 correctness test cases stored in the repository (42 original + 18 unsorted), plus 1 performance test that should be implemented manually (see §Test Framework).

**Demo examples** ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (0, 2, 4, 6, 8)$, $\y = (10, 12, 14, 16, 18)$, expected output: $-10$ (base case)
- `demo-2`: $\x = (0, 2, 4, 6, 8)$, $\y = (0, 2, 4, 6, 8)$, expected output: $0$ (identity property)
- `demo-3`: $\x = (7, 9, 11, 13, 15)$, $\y = (13, 15, 17, 19, 21)$ (= demo-1 + [7,3]), expected output: $-6$ (location equivariance)
- `demo-4`: $\x = (0, 4, 8, 12, 16)$, $\y = (20, 24, 28, 32, 36)$ (= 2 × demo-1), expected output: $-20$ (scale equivariance)
- `demo-5`: $\x = (10, 12, 14, 16, 18)$, $\y = (0, 2, 4, 6, 8)$ (= reversed demo-1), expected output: $10$ (anti-symmetry)

**Natural sequences** ($[n, m] \in \{1, 2, 3\} \times \{1, 2, 3\}$) — 9 combinations:

- `natural-1-1`: $\x = (1)$, $\y = (1)$, expected output: $0$
- `natural-1-2`: $\x = (1)$, $\y = (1, 2)$, expected output: $-0.5$
- `natural-1-3`: $\x = (1)$, $\y = (1, 2, 3)$, expected output: $-1$
- `natural-2-1`: $\x = (1, 2)$, $\y = (1)$, expected output: $0.5$
- `natural-2-2`: $\x = (1, 2)$, $\y = (1, 2)$, expected output: $0$
- `natural-2-3`: $\x = (1, 2)$, $\y = (1, 2, 3)$, expected output: $-0.5$
- `natural-3-1`: $\x = (1, 2, 3)$, $\y = (1)$, expected output: $1$
- `natural-3-2`: $\x = (1, 2, 3)$, $\y = (1, 2)$, expected output: $0.5$
- `natural-3-3`: $\x = (1, 2, 3)$, $\y = (1, 2, 3)$, expected output: $0$

**Negative values** ($[n, m] = [2, 2]$) — sign handling validation:

- `negative-2-2`: $\x = (-2, -1)$, $\y = (-2, -1)$, expected output: $0$

**Mixed-sign values** ($[n, m] = [2, 2]$) — validates anti-symmetry across zero:

- `mixed-2-2`: $\x = (-1, 1)$, $\y = (-1, 1)$, expected output: $0$

**Zero values** ($[n, m] \in \{1, 2\} \times \{1, 2\}$) — 4 combinations:

- `zeros-1-1`, `zeros-1-2`, `zeros-2-1`, `zeros-2-2`: all produce output $0$

**Additive distribution** ($[n, m] \in \{5, 10, 30\} \times \{5, 10, 30\}$) — 9 combinations with $\Additive(10, 1)$:

- `additive-5-5`, `additive-5-10`, `additive-5-30`
- `additive-10-5`, `additive-10-10`, `additive-10-30`
- `additive-30-5`, `additive-30-10`, `additive-30-30`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The natural sequences validate anti-symmetry ($\Shift(\x, \y) = -\Shift(\y, \x)$) and the identity property ($\Shift(\x, \x) = 0$).
The asymmetric size combinations test the two-sample algorithm with unbalanced inputs.

**Algorithm stress tests** — edge cases for fast binary search algorithm:

- `duplicates-5-5`: $\x = (3, 3, 3, 3, 3)$, $\y = (3, 3, 3, 3, 3)$ (all identical, expected output: $0$)
- `duplicates-10-10`: $\x = (1, 1, 2, 2, 3, 3, 4, 4, 5, 5)$, $\y = (1, 1, 2, 2, 3, 3, 4, 4, 5, 5)$ (many duplicates)
- `parity-odd-7-7`: $\x = (1, 2, 3, 4, 5, 6, 7)$, $\y = (1, 2, 3, 4, 5, 6, 7)$ (odd sizes, 49 differences, expected output: $0$)
- `parity-even-6-6`: $\x = (1, 2, 3, 4, 5, 6)$, $\y = (1, 2, 3, 4, 5, 6)$ (even sizes, 36 differences, expected output: $0$)
- `parity-asymmetric-7-6`: $\x = (1, 2, 3, 4, 5, 6, 7)$, $\y = (1, 2, 3, 4, 5, 6)$ (mixed parity, 42 differences)
- `parity-large-49-50`: $\x = (1, 2, \ldots, 49)$, $\y = (1, 2, \ldots, 50)$ (large asymmetric, 2450 differences)

**Extreme asymmetry** — tests with very unbalanced sample sizes:

- `asymmetry-1-100`: $\x = (50)$, $\y = (1, 2, \ldots, 100)$ (single vs many, 100 differences)
- `asymmetry-2-50`: $\x = (10, 20)$, $\y = (1, 2, \ldots, 50)$ (tiny vs medium, 100 differences)
- `asymmetry-constant-varied`: $\x = (5, 5, 5, 5, 5)$, $\y = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)$ (constant vs varied)

**Unsorted tests** — verify independent sorting of each sample (18 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4), (5,5)\}$: X unsorted (reversed), Y sorted (3 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4), (5,5)\}$: X sorted, Y unsorted (reversed) (3 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4), (5,5)\}$: both unsorted (reversed) (3 tests)
- `unsorted-reverse-3-3`: $\x = (3, 2, 1)$, $\y = (3, 2, 1)$ (both reversed)
- `unsorted-x-shuffle-3-3`: $\x = (2, 1, 3)$, $\y = (1, 2, 3)$ (X shuffled, Y sorted)
- `unsorted-y-shuffle-3-3`: $\x = (1, 2, 3)$, $\y = (3, 1, 2)$ (X sorted, Y shuffled)
- `unsorted-both-shuffle-4-4`: $\x = (3, 1, 4, 2)$, $\y = (4, 2, 1, 3)$ (both shuffled)
- `unsorted-duplicates-mixed-5-5`: $\x = (3, 3, 3, 3, 3)$, $\y = (3, 3, 3, 3, 3)$ (all identical)
- `unsorted-x-unsorted-duplicates`: $\x = (2, 1, 3, 2, 1)$, $\y = (1, 1, 2, 2, 3)$ (X has unsorted duplicates)
- `unsorted-y-unsorted-duplicates`: $\x = (1, 1, 2, 2, 3)$, $\y = (3, 2, 1, 3, 2)$ (Y has unsorted duplicates)
- `unsorted-asymmetric-unsorted-2-5`: $\x = (2, 1)$, $\y = (5, 2, 4, 1, 3)$ (asymmetric sizes, both unsorted)
- `unsorted-negative-unsorted-3-3`: $\x = (-1, -3, -2)$, $\y = (-2, -3, -1)$ (negative unsorted)

These tests are critical for two-sample estimators because they verify that $\x$ and $\y$ are sorted **independently**.
The variety includes cases where only one sample is unsorted, ensuring implementations don't incorrectly assume pre-sorted input or sort samples together.

**Performance test** — validates the fast $O((m+n) \log L)$ binary search algorithm:

- **Input**: $\x = (1, 2, 3, \ldots, 100000)$, $\y = (1, 2, 3, \ldots, 100000)$
- **Expected output**: $0$
- **Time constraint**: Must complete in under 5 seconds
- **Purpose**: Ensures that the implementation uses the efficient algorithm rather than materializing all $mn = 10$ billion pairwise differences

This test case is not stored in the repository because it generates a large JSON file (approximately 1.5 MB).
Each language implementation should manually implement this test with the hardcoded expected result.

## Ratio

$$
\Ratio(\x, \y) = \underset{1 \leq i \leq n, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
$$

The $\Ratio$ test suite contains 38 test cases (26 original + 12 unsorted), excluding zero values due to division constraints.

**Demo examples** ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (1, 2, 4, 8, 16)$, $\y = (2, 4, 8, 16, 32)$, expected output: $0.5$ (base case)
- `demo-2`: $\x = (1, 2, 4, 8, 16)$, $\y = (1, 2, 4, 8, 16)$, expected output: $1$ (identity property)
- `demo-3`: $\x = (2, 4, 8, 16, 32)$, $\y = (10, 20, 40, 80, 160)$ (= [2×demo-1.x, 5×demo-1.y]), expected output: $0.2$ (scale property)

**Natural sequences** ($[n, m] \in \{1, 2, 3\} \times \{1, 2, 3\}$) — 9 combinations:

- `natural-1-1`: $\x = (1)$, $\y = (1)$, expected output: $1$
- `natural-1-2`: $\x = (1)$, $\y = (1, 2)$, expected output: $\approx 0.667$
- `natural-1-3`: $\x = (1)$, $\y = (1, 2, 3)$, expected output: $0.5$
- `natural-2-1`: $\x = (1, 2)$, $\y = (1)$, expected output: $1.5$
- `natural-2-2`: $\x = (1, 2)$, $\y = (1, 2)$, expected output: $1$
- `natural-2-3`: $\x = (1, 2)$, $\y = (1, 2, 3)$, expected output: $\approx 0.833$
- `natural-3-1`: $\x = (1, 2, 3)$, $\y = (1)$, expected output: $2$
- `natural-3-2`: $\x = (1, 2, 3)$, $\y = (1, 2)$, expected output: $1.5$
- `natural-3-3`: $\x = (1, 2, 3)$, $\y = (1, 2, 3)$, expected output: $1$

**Additive distribution** ($[n, m] \in \{5, 10, 30\} \times \{5, 10, 30\}$) — 9 combinations with $\Additive(10, 1)$:

- `additive-5-5`, `additive-5-10`, `additive-5-30`
- `additive-10-5`, `additive-10-10`, `additive-10-30`
- `additive-30-5`, `additive-30-10`, `additive-30-30`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The natural sequences verify the identity property ($\Ratio(\x, \x) = 1$) and validate ratio calculations with simple integer inputs.
Note that implementations should handle the practical constraint of avoiding division by values near zero.

**Unsorted tests** — verify independent sorting for ratio calculation (12 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: $\x = (16, 1, 8, 2, 4)$, $\y = (2, 4, 8, 16, 32)$ (demo-1 with X unsorted)
- `unsorted-demo-unsorted-y`: $\x = (1, 2, 4, 8, 16)$, $\y = (32, 2, 16, 4, 8)$ (demo-1 with Y unsorted)
- `unsorted-demo-both-unsorted`: $\x = (8, 1, 16, 4, 2)$, $\y = (16, 32, 2, 8, 4)$ (demo-1 both unsorted)
- `unsorted-identity-unsorted`: $\x = (4, 1, 8, 2, 16)$, $\y = (16, 1, 8, 4, 2)$ (identity property, both unsorted)
- `unsorted-asymmetric-unsorted-2-3`: $\x = (2, 1)$, $\y = (3, 1, 2)$ (asymmetric, both unsorted)
- `unsorted-power-unsorted-5`: $\x = (16, 2, 8, 1, 4)$, $\y = (32, 4, 16, 2, 8)$ (powers of 2 unsorted)

## AvgSpread

$$
\AvgSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

The $\AvgSpread$ test suite contains 50 test cases (35 original + 15 unsorted).
Since $\AvgSpread$ computes $\Spread(\x)$ and $\Spread(\y)$ independently, unsorted tests are critical to verify that both samples are sorted independently before computing their spreads.

**Demo examples** ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (0, 3, 6, 9, 12)$, $\y = (0, 2, 4, 6, 8)$, expected output: $5$ (base case: $(5 \cdot 6 + 5 \cdot 4)/10$)
- `demo-2`: $\x = (0, 3, 6, 9, 12)$, $\y = (0, 3, 6, 9, 12)$, expected output: $6$ (identity case)
- `demo-3`: $\x = (0, 6, 12, 18, 24)$, $\y = (0, 9, 18, 27, 36)$ (= [2×demo-1.x, 3×demo-1.y]), expected output: $15$ (scale equivariance)
- `demo-4`: $\x = (0, 2, 4, 6, 8)$, $\y = (0, 3, 6, 9, 12)$ (= reversed demo-1), expected output: $5$ (symmetry)
- `demo-5`: $\x = (0, 6, 12, 18, 24)$, $\y = (0, 4, 8, 12, 16)$ (= 2 × demo-1), expected output: $10$ (uniform scaling)

**Natural sequences** ($[n, m] \in \{1, 2, 3\} \times \{1, 2, 3\}$) — 9 combinations:

- All combinations from single-element to three-element samples, validating the weighted average calculation

**Negative values** ($[n, m] = [2, 2]$) — validates spread calculation with negative values:

- `negative-2-2`: $\x = (-2, -1)$, $\y = (-2, -1)$, expected output: $1$

**Zero values** ($[n, m] \in \{1, 2\} \times \{1, 2\}$) — 4 combinations:

- All produce output $0$ since $\Spread$ of constant samples is zero

**Additive distribution** ($[n, m] \in \{5, 10, 30\} \times \{5, 10, 30\}$) — 9 combinations with $\Additive(10, 1)$:

- Tests pooled dispersion across different sample size combinations
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- Validates correct weighting when sample sizes differ substantially
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The asymmetric size combinations are particularly important for $\AvgSpread$ because the estimator must correctly weight each sample's contribution by its size.

**Composite estimator stress tests** — edge cases for weighted averaging:

- `composite-asymmetric-weights`: $\x = (1, 2)$, $\y = (3, 4, 5, 6, 7, 8, 9, 10)$ (2 vs 8, tests weighting formula)
- `composite-zero-spread-one`: $\x = (5, 5, 5)$, $\y = (1, 2, 3, 4, 5)$ (one zero spread, tests edge case)
- `composite-extreme-sizes`: $\x = (10)$, $\y = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)$ (1 vs 10, extreme weighting)

**Unsorted tests** — critical for verifying independent sorting (15 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: $\x = (12, 0, 6, 3, 9)$, $\y = (0, 2, 4, 6, 8)$ (demo-1 with X unsorted)
- `unsorted-demo-unsorted-y`: $\x = (0, 3, 6, 9, 12)$, $\y = (8, 0, 4, 2, 6)$ (demo-1 with Y unsorted)
- `unsorted-demo-both-unsorted`: $\x = (9, 0, 12, 3, 6)$, $\y = (6, 0, 8, 2, 4)$ (demo-1 both unsorted)
- `unsorted-identity-unsorted`: $\x = (6, 0, 12, 3, 9)$, $\y = (9, 0, 12, 6, 3)$ (demo-2 unsorted)
- `unsorted-negative-unsorted`: $\x = (-1, -2)$, $\y = (-1, -2)$ (negative unsorted)
- `unsorted-zero-unsorted-2-2`: $\x = (0, 0)$, $\y = (0, 0)$ (zeros, any order)
- `unsorted-asymmetric-weights-unsorted`: $\x = (2, 1)$, $\y = (8, 3, 6, 4, 10, 5, 9, 7)$ (asymmetric unsorted)
- `unsorted-zero-spread-x-unsorted`: $\x = (5, 5, 5)$, $\y = (5, 1, 4, 2, 3)$ (zero spread X, Y unsorted)

These tests verify that implementations compute $\Spread(\x)$ and $\Spread(\y)$ with properly sorted samples.

## Disparity

$$
\Disparity(\x, \y) = \dfrac{\Shift(\x, \y)}{\AvgSpread(\x, \y)}
$$

The $\Disparity$ test suite contains 28 test cases (16 original + 12 unsorted).
Since $\Disparity$ combines $\Shift$ and $\AvgSpread$, unsorted tests verify both components handle sorting correctly.

**Demo examples** ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $\x = (0, 3, 6, 9, 12)$, $\y = (0, 2, 4, 6, 8)$, expected output: $0.4$ (base case: $2/5$)
- `demo-2`: $\x = (5, 8, 11, 14, 17)$, $\y = (5, 7, 9, 11, 13)$ (= demo-1 + 5), expected output: $0.4$ (location invariance)
- `demo-3`: $\x = (0, 6, 12, 18, 24)$, $\y = (0, 4, 8, 12, 16)$ (= 2 × demo-1), expected output: $0.4$ (scale invariance)
- `demo-4`: $\x = (0, 2, 4, 6, 8)$, $\y = (0, 3, 6, 9, 12)$ (= reversed demo-1), expected output: $-0.4$ (anti-symmetry)

**Natural sequences** ($[n, m] \in \{2, 3\} \times \{2, 3\}$) — 4 combinations:

- `natural-2-2`, `natural-2-3`, `natural-3-2`, `natural-3-3`
- Minimum size $n, m \geq 2$ required for meaningful dispersion calculations

**Negative values** ($[n, m] = [2, 2]$) — end-to-end validation with negative values:

- `negative-2-2`: $\x = (-2, -1)$, $\y = (-2, -1)$, expected output: $0$

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

The smaller test set for $\Disparity$ reflects implementation confidence.
Since $\Disparity$ combines $\Shift$ and $\AvgSpread$, correct implementation of those components ensures $\Disparity$ correctness.
The test cases validate the division operation and confirm scale-free properties.

**Composite estimator stress tests** — edge cases for effect size calculation:

- `composite-small-avgspread`: $\x = (10.001, 10.002, 10.003)$, $\y = (10.004, 10.005, 10.006)$ (tiny spread, large shift)
- `composite-large-avgspread`: $\x = (1, 100, 200)$, $\y = (50, 150, 250)$ (large spread, small shift)
- `composite-extreme-disparity`: $\x = (1, 1.001)$, $\y = (100, 100.001)$ (extreme ratio, tests precision)

**Unsorted tests** — verify both Shift and AvgSpread handle sorting (12 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: X unsorted (reversed), Y sorted (2 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: X sorted, Y unsorted (reversed) (2 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) \in \{(3,3), (4,4)\}$: both unsorted (reversed) (2 tests)
- `unsorted-demo-unsorted-x`: $\x = (12, 0, 6, 3, 9)$, $\y = (0, 2, 4, 6, 8)$ (demo-1 with X unsorted)
- `unsorted-demo-unsorted-y`: $\x = (0, 3, 6, 9, 12)$, $\y = (8, 0, 4, 2, 6)$ (demo-1 with Y unsorted)
- `unsorted-demo-both-unsorted`: $\x = (9, 0, 12, 3, 6)$, $\y = (6, 0, 8, 2, 4)$ (demo-1 both unsorted)
- `unsorted-location-invariance-unsorted`: $\x = (17, 5, 11, 8, 14)$, $\y = (13, 5, 9, 7, 11)$ (demo-2 unsorted)
- `unsorted-scale-invariance-unsorted`: $\x = (24, 0, 12, 6, 18)$, $\y = (16, 0, 8, 4, 12)$ (demo-3 unsorted)
- `unsorted-anti-symmetry-unsorted`: $\x = (8, 0, 4, 2, 6)$, $\y = (12, 0, 6, 3, 9)$ (demo-4 reversed and unsorted)

As a composite estimator, $\Disparity$ tests both the numerator ($\Shift$) and denominator ($\AvgSpread$).
Unsorted variants verify end-to-end correctness including invariance properties.

## Test Framework

The reference test framework consists of three components:

**Test generation** — The C# implementation defines test inputs programmatically using builder patterns.
For deterministic cases, inputs are explicitly specified.
For random cases, the framework uses controlled seeds with `System.Random` to ensure reproducibility across all platforms.

The random generation mechanism works as follows:

- Each test suite builder maintains a seed counter initialized to zero.
- For one-sample estimators, each distribution type receives the next available seed.
  The same random generator produces all samples for all sizes within that distribution.
- For two-sample estimators, each pair of distributions receives two consecutive seeds:
  one for the $\x$ sample generator and one for the $\y$ sample generator.
- The seed counter increments with each random generator creation, ensuring deterministic test data generation.

For $\Additive$ distributions, random values are generated using the Box-Müller transform,
  which converts pairs of uniform random values into normally distributed values.
The transform applies the formula:

$$
X = \mu + \sigma \sqrt{-2 \ln(U_1)} \sin(2\pi U_2)
$$

where $U_1, U_2$ are uniform random values from $\Uniform(0, 1)$, $\mu$ is the mean, and $\sigma$ is the standard deviation.

For $\Uniform$ distributions, random values are generated directly using the quantile function:

$$
X = \min + U \cdot (\max - \min)
$$

where $U$ is a uniform random value from $\Uniform(0, 1)$.

The framework executes the reference implementation on all generated inputs and serializes input-output pairs to JSON format.

**Test validation** — Each language implementation loads the JSON test cases and executes them against the local estimator implementation.
Assertions verify that outputs match expected values within numerical tolerance (typically $10^{-10}$ for relative error).

**Test data format** — Each test case is a JSON file containing `input` and `output` fields.
For one-sample estimators, input contains array `x` and optional `parameters`.
For two-sample estimators, input contains arrays `x` and `y`.
Output is a single numeric value.

**Performance testing** — The toolkit provides $O(n \log n)$ fast algorithms for $\Center$, $\Spread$, and $\Shift$ estimators,
dramatically more efficient than naive implementations that materialize all pairwise combinations.
Performance tests use sample size $n = 100{,}000$ (for one-sample) or $n = m = 100{,}000$ (for two-sample).
This specific size creates a clear performance distinction:
fast implementations ($O(n \log n)$ or $O((m+n) \log L)$) complete in under 5 seconds on modern hardware across all supported languages,
while naive implementations ($O(n^2 \log n)$ or $O(mn \log(mn))$) would be prohibitively slow (taking hours or failing due to memory exhaustion).
With $n = 100{,}000$, naive approaches would need to materialize approximately 5 billion pairwise values for $\Center$/$\Spread$
or 10 billion for $\Shift$, whereas fast algorithms require only $O(n)$ additional memory.
Performance tests serve dual purposes: correctness validation at scale and performance regression detection,
ensuring implementations use the efficient algorithms and remain practical for real-world datasets with hundreds of thousands of observations.
Performance test specifications are provided in the respective estimator sections above.

This framework ensures that all seven language implementations maintain strict numerical agreement across the full test suite.