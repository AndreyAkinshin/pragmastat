## AvgSpread Tests

$$
\AvgSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

The $\AvgSpread$ test suite contains 49 test cases (35 original + 14 unsorted).
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

**Unsorted tests** — critical for verifying independent sorting (14 tests):

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
