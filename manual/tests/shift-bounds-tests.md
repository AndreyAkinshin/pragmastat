## ShiftBounds Tests

$$
\ShiftBounds(\x, \y, \misrate) = [z_{(k_{\mathrm{left}})}; z_{(k_{\mathrm{right}})}]
$$

where

$$
\z = \left\{ x_i - y_j \right\}_{1 \leq i \leq n,\, 1 \leq j \leq m} \text{ (sorted)}
$$

$$
k_{\mathrm{left}} = \lfloor \PairwiseMargin(n, m, \misrate) / 2 \rfloor + 1
$$

$$
k_{\mathrm{right}} = nm - \lfloor \PairwiseMargin(n, m, \misrate) / 2 \rfloor
$$

The $\ShiftBounds$ test suite contains 61 correctness test cases (3 demo + 9 natural + 6 property + 10 edge + 9 additive + 4 uniform + 5 misrate + 15 unsorted).
Since $\ShiftBounds$ returns bounds rather than a point estimate, tests validate that the bounds contain $\Shift(\x, \y)$ and satisfy equivariance properties.
Each test case output is a JSON object with `lower` and `upper` fields representing the interval bounds.

**Demo examples** ($n = m = 5$) — from manual introduction, validating basic bounds:

- `demo-1`: $\x = (1, 2, 3, 4, 5)$, $\y = (3, 4, 5, 6, 7)$, $\misrate = 0.05$, expected output: $[-4, 0]$
- `demo-2`: $\x = (1, 2, 3, 4, 5)$, $\y = (3, 4, 5, 6, 7)$, $\misrate = 0.01$, expected output: $[-5, 1]$
- `demo-3`: $\x = (3, 4, 5, 6, 7)$, $\y = (3, 4, 5, 6, 7)$, $\misrate = 0.05$, expected output: bounds containing $0$ (identity case)

These cases illustrate how tighter misrates produce wider bounds and validate the identity property where identical samples yield bounds containing zero.

**Natural sequences** ($[n, m] \in \{1, 2, 3\} \times \{1, 2, 3\}$, $\misrate = 10^{-2}$) — 9 combinations:

- `natural-1-1`: $\x = (1)$, $\y = (1)$, expected bounds containing $0$
- `natural-1-2`: $\x = (1)$, $\y = (1, 2)$, expected bounds containing $-0.5$
- `natural-1-3`: $\x = (1)$, $\y = (1, 2, 3)$, expected bounds containing $-1$
- `natural-2-1`: $\x = (1, 2)$, $\y = (1)$, expected bounds containing $0.5$
- `natural-2-2`: $\x = (1, 2)$, $\y = (1, 2)$, expected bounds containing $0$
- `natural-2-3`: $\x = (1, 2)$, $\y = (1, 2, 3)$, expected bounds containing $-0.5$
- `natural-3-1`: $\x = (1, 2, 3)$, $\y = (1)$, expected bounds containing $1$
- `natural-3-2`: $\x = (1, 2, 3)$, $\y = (1, 2)$, expected bounds containing $0.5$
- `natural-3-3`: $\x = (1, 2, 3)$, $\y = (1, 2, 3)$, expected bounds containing $0$

These canonical cases validate that bounds properly contain the corresponding $\Shift$ values and handle small sample sizes correctly.

**Property validation** ($n = m = 5$, $\misrate = 10^{-3}$) — 6 tests:

- `property-identity`: $\x = (0, 2, 4, 6, 8)$, $\y = (0, 2, 4, 6, 8)$, bounds must contain $0$
- `property-location-shift`: $\x = (7, 9, 11, 13, 15)$, $\y = (13, 15, 17, 19, 21)$ (= demo-1 + [7, 3])
  - Must produce same bounds as base case (location invariance)
- `property-scale-2x`: $\x = (2, 4, 6, 8, 10)$, $\y = (6, 8, 10, 12, 14)$ (= 2 × demo-1)
  - Bounds must be 2× the base case bounds (scale equivariance)
- `property-antisymmetry`: $\x = (3, 4, 5, 6, 7)$, $\y = (1, 2, 3, 4, 5)$ (= reversed demo-1)
  - Bounds must be negated: if original is $[a, b]$, this yields $[-b, -a]$
- `property-negative`: $\x = (-5, -4, -3, -2, -1)$, $\y = (-7, -6, -5, -4, -3)$
  - Validates sign handling with all negative values
- `property-mixed-signs`: $\x = (-2, -1, 0, 1, 2)$, $\y = (-1, 0, 1, 2, 3)$
  - Validates bounds crossing zero with mixed-sign samples

**Edge cases** — boundary conditions and extreme scenarios (10 tests):

- `edge-min-samples`: $\x = (1)$, $\y = (2)$, $\misrate = 10^{-2}$ (minimum samples, single difference)
- `edge-permissive-misrate`: $\x = (1, 2, 3, 4, 5)$, $\y = (3, 4, 5, 6, 7)$, $\misrate = 0.5$ (very wide bounds)
- `edge-strict-misrate`: $\x = (1, 2, 3, 4, 5)$, $\y = (3, 4, 5, 6, 7)$, $\misrate = 10^{-6}$ (very narrow bounds)
- `edge-zero-shift`: $\x = (5, 5, 5)$, $\y = (5, 5, 5)$, $\misrate = 10^{-3}$ (all identical, bounds around 0)
- `edge-asymmetric-1-100`: $\x = (50)$, $\y = (1, 2, \ldots, 100)$, $\misrate = 10^{-2}$ (extreme size difference)
- `edge-asymmetric-2-50`: $\x = (25, 26)$, $\y = (1, 2, \ldots, 50)$, $\misrate = 10^{-3}$ (highly unbalanced)
- `edge-duplicates`: $\x = (3, 3, 3, 3, 3)$, $\y = (5, 5, 5, 5, 5)$, $\misrate = 10^{-2}$ (all duplicates, bounds around -2)
- `edge-wide-range`: $\x = (0.001, 1, 100, 1000, 10000)$, $\y = (0.1, 10, 1000, 100000)$, $\misrate = 10^{-3}$ (extreme value range)
- `edge-tiny-values`: $\x = (1e{-}8, 2e{-}8, 3e{-}8)$, $\y = (2e{-}8, 3e{-}8, 4e{-}8)$, $\misrate = 10^{-2}$ (numerical precision)
- `edge-large-values`: $\x = (1e8, 2e8, 3e8)$, $\y = (2e8, 3e8, 4e8)$, $\misrate = 10^{-2}$ (large magnitude)

These edge cases stress-test boundary conditions, numerical stability, and the margin calculation with extreme parameters.

**Additive distribution** ($[n, m] \in \{5, 10, 30\} \times \{5, 10, 30\}$, $\misrate = 10^{-3}$) — 9 combinations with $\Additive(10, 1)$:

- `additive-5-5`, `additive-5-10`, `additive-5-30`
- `additive-10-5`, `additive-10-10`, `additive-10-30`
- `additive-30-5`, `additive-30-10`, `additive-30-30`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

These fuzzy tests validate that bounds properly encompass the shift estimate for realistic normally-distributed data at various sample sizes.

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$, $\misrate = 10^{-4}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The asymmetric size combinations are particularly important for testing margin calculation with unbalanced samples.

**Misrate variation** ($\x = (0, 2, 4, 6, 8)$, $\y = (10, 12, 14, 16, 18)$) — 5 tests with varying misrates:

- `misrate-1e-2`: $\misrate = 10^{-2}$
- `misrate-1e-3`: $\misrate = 10^{-3}$
- `misrate-1e-4`: $\misrate = 10^{-4}$
- `misrate-1e-5`: $\misrate = 10^{-5}$
- `misrate-1e-6`: $\misrate = 10^{-6}$

These tests use identical samples with varying misrates to validate the monotonicity property: smaller misrates (higher confidence) produce wider bounds.
The sequence demonstrates how bound width increases as misrate decreases, helping implementations verify correct margin calculation.

**Unsorted tests** — verify independent sorting of $\x$ and $\y$ (15 tests):

- `unsorted-x-natural-3-3`: $\x = (3, 2, 1)$, $\y = (1, 2, 3)$, $\misrate = 10^{-2}$ (X reversed, Y sorted)
- `unsorted-y-natural-3-3`: $\x = (1, 2, 3)$, $\y = (3, 2, 1)$, $\misrate = 10^{-2}$ (X sorted, Y reversed)
- `unsorted-both-natural-3-3`: $\x = (3, 2, 1)$, $\y = (3, 2, 1)$, $\misrate = 10^{-2}$ (both reversed)
- `unsorted-x-shuffle-4-4`: $\x = (3, 1, 4, 2)$, $\y = (1, 2, 3, 4)$, $\misrate = 10^{-3}$ (X shuffled)
- `unsorted-y-shuffle-4-4`: $\x = (1, 2, 3, 4)$, $\y = (4, 2, 1, 3)$, $\misrate = 10^{-3}$ (Y shuffled)
- `unsorted-both-shuffle-4-4`: $\x = (3, 1, 4, 2)$, $\y = (2, 4, 1, 3)$, $\misrate = 10^{-3}$ (both shuffled)
- `unsorted-demo-unsorted-x`: $\x = (5, 1, 4, 2, 3)$, $\y = (3, 4, 5, 6, 7)$, $\misrate = 0.05$ (demo-1 X unsorted)
- `unsorted-demo-unsorted-y`: $\x = (1, 2, 3, 4, 5)$, $\y = (7, 3, 6, 4, 5)$, $\misrate = 0.05$ (demo-1 Y unsorted)
- `unsorted-demo-both-unsorted`: $\x = (4, 1, 5, 2, 3)$, $\y = (6, 3, 7, 4, 5)$, $\misrate = 0.05$ (demo-1 both unsorted)
- `unsorted-identity-unsorted`: $\x = (4, 1, 5, 2, 3)$, $\y = (5, 1, 4, 3, 2)$, $\misrate = 10^{-2}$ (identity property, both unsorted)
- `unsorted-negative-unsorted`: $\x = (-1, -3, -2)$, $\y = (-2, -3, -1)$, $\misrate = 10^{-2}$ (negative values unsorted)
- `unsorted-asymmetric-2-5`: $\x = (2, 1)$, $\y = (5, 2, 4, 1, 3)$, $\misrate = 10^{-3}$ (asymmetric sizes, both unsorted)
- `unsorted-duplicates`: $\x = (3, 3, 3, 3, 3)$, $\y = (5, 5, 5, 5, 5)$, $\misrate = 10^{-2}$ (all duplicates, any order)
- `unsorted-mixed-duplicates-x`: $\x = (2, 1, 3, 2, 1)$, $\y = (1, 1, 2, 2, 3)$, $\misrate = 10^{-3}$ (X has unsorted duplicates)
- `unsorted-mixed-duplicates-y`: $\x = (1, 1, 2, 2, 3)$, $\y = (3, 2, 1, 3, 2)$, $\misrate = 10^{-3}$ (Y has unsorted duplicates)

These unsorted tests are critical because $\ShiftBounds$ computes bounds from pairwise differences, requiring both samples to be sorted independently.
The variety ensures implementations don't incorrectly assume pre-sorted input or sort samples together.
Each test must produce identical output to its sorted counterpart, validating that the implementation correctly handles the sorting step.

**No performance test** — $\ShiftBounds$ uses the $\FastShift$ algorithm internally, which is already validated by the $\Shift$ performance test.
Since bounds computation involves only two quantile calculations from the pairwise differences (at positions determined by $\PairwiseMargin$),
the performance characteristics are equivalent to computing two $\Shift$ estimates, which completes efficiently for large samples.