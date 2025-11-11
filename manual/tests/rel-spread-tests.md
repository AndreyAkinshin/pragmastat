## RelSpread Tests

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
