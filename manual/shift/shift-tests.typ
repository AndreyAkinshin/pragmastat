#import "/manual/definitions.typ": *

$ Shift(vx, vy) = median_(1 <= i <= n, 1 <= j <= m) (x_i - y_j) $

The $Shift$ test suite contains 60 correctness test cases stored in the repository (42 original + 18 unsorted), plus 1 performance test that should be implemented manually (see #link(<sec-test-framework>)[Test Framework]).

*Demo examples* ($n = m = 5$) — from manual introduction, validating properties:

- `demo-1`: $vx = (0, 2, 4, 6, 8)$, $vy = (10, 12, 14, 16, 18)$, expected output: $-10$ (base case)
- `demo-2`: $vx = (0, 2, 4, 6, 8)$, $vy = (0, 2, 4, 6, 8)$, expected output: $0$ (identity property)
- `demo-3`: $vx = (7, 9, 11, 13, 15)$, $vy = (13, 15, 17, 19, 21)$ (= demo-1 + [7,3]), expected output: $-6$ (location equivariance)
- `demo-4`: $vx = (0, 4, 8, 12, 16)$, $vy = (20, 24, 28, 32, 36)$ (= 2 × demo-1), expected output: $-20$ (scale equivariance)
- `demo-5`: $vx = (10, 12, 14, 16, 18)$, $vy = (0, 2, 4, 6, 8)$ (= reversed demo-1), expected output: $10$ (anti-symmetry)

*Natural sequences* ($[n, m] in {1, 2, 3} times {1, 2, 3}$) — 9 combinations:

- `natural-1-1`: $vx = (1)$, $vy = (1)$, expected output: $0$
- `natural-1-2`: $vx = (1)$, $vy = (1, 2)$, expected output: $-0.5$
- `natural-1-3`: $vx = (1)$, $vy = (1, 2, 3)$, expected output: $-1$
- `natural-2-1`: $vx = (1, 2)$, $vy = (1)$, expected output: $0.5$
- `natural-2-2`: $vx = (1, 2)$, $vy = (1, 2)$, expected output: $0$
- `natural-2-3`: $vx = (1, 2)$, $vy = (1, 2, 3)$, expected output: $-0.5$
- `natural-3-1`: $vx = (1, 2, 3)$, $vy = (1)$, expected output: $1$
- `natural-3-2`: $vx = (1, 2, 3)$, $vy = (1, 2)$, expected output: $0.5$
- `natural-3-3`: $vx = (1, 2, 3)$, $vy = (1, 2, 3)$, expected output: $0$

*Negative values* ($[n, m] = [2, 2]$) — sign handling validation:

- `negative-2-2`: $vx = (-2, -1)$, $vy = (-2, -1)$, expected output: $0$

*Mixed-sign values* ($[n, m] = [2, 2]$) — validates anti-symmetry across zero:

- `mixed-2-2`: $vx = (-1, 1)$, $vy = (-1, 1)$, expected output: $0$

*Zero values* ($[n, m] in {1, 2} times {1, 2}$) — 4 combinations:

- `zeros-1-1`, `zeros-1-2`, `zeros-2-1`, `zeros-2-2`: all produce output $0$

*Additive distribution* ($[n, m] in {5, 10, 30} times {5, 10, 30}$) — 9 combinations with $Additive(10, 1)$:

- `additive-5-5`, `additive-5-10`, `additive-5-30`
- `additive-10-5`, `additive-10-10`, `additive-10-30`
- `additive-30-5`, `additive-30-10`, `additive-30-30`
- Random generation: $vx$ uses seed 0, $vy$ uses seed 1

*Uniform distribution* ($[n, m] in {5, 100} times {5, 100}$) — 4 combinations with $Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $vx$ uses seed 2, $vy$ uses seed 3

The natural sequences validate anti-symmetry ($Shift(vx, vy) = -Shift(vy, vx)$) and the identity property ($Shift(vx, vx) = 0$).
The asymmetric size combinations test the two-sample algorithm with unbalanced inputs.

*Algorithm stress tests* — edge cases for fast binary search algorithm:

- `duplicates-5-5`: $vx = (3, 3, 3, 3, 3)$, $vy = (3, 3, 3, 3, 3)$ (all identical, expected output: $0$)
- `duplicates-10-10`: $vx = (1, 1, 2, 2, 3, 3, 4, 4, 5, 5)$, $vy = (1, 1, 2, 2, 3, 3, 4, 4, 5, 5)$ (many duplicates)
- `parity-odd-7-7`: $vx = (1, 2, 3, 4, 5, 6, 7)$, $vy = (1, 2, 3, 4, 5, 6, 7)$ (odd sizes, 49 differences, expected output: $0$)
- `parity-even-6-6`: $vx = (1, 2, 3, 4, 5, 6)$, $vy = (1, 2, 3, 4, 5, 6)$ (even sizes, 36 differences, expected output: $0$)
- `parity-asymmetric-7-6`: $vx = (1, 2, 3, 4, 5, 6, 7)$, $vy = (1, 2, 3, 4, 5, 6)$ (mixed parity, 42 differences)
- `parity-large-49-50`: $vx = (1, 2, ..., 49)$, $vy = (1, 2, ..., 50)$ (large asymmetric, 2450 differences)

*Extreme asymmetry* — tests with very unbalanced sample sizes:

- `asymmetry-1-100`: $vx = (50)$, $vy = (1, 2, ..., 100)$ (single vs many, 100 differences)
- `asymmetry-2-50`: $vx = (10, 20)$, $vy = (1, 2, ..., 50)$ (tiny vs medium, 100 differences)
- `asymmetry-constant-varied`: $vx = (5, 5, 5, 5, 5)$, $vy = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)$ (constant vs varied)

*Unsorted tests* — verify independent sorting of each sample (18 tests):

- `unsorted-x-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4), (5,5)}$: X unsorted (reversed), Y sorted (3 tests)
- `unsorted-y-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4), (5,5)}$: X sorted, Y unsorted (reversed) (3 tests)
- `unsorted-both-natural-{n}-{m}` for $(n,m) in {(3,3), (4,4), (5,5)}$: both unsorted (reversed) (3 tests)
- `unsorted-reverse-3-3`: $vx = (3, 2, 1)$, $vy = (3, 2, 1)$ (both reversed)
- `unsorted-x-shuffle-3-3`: $vx = (2, 1, 3)$, $vy = (1, 2, 3)$ (X shuffled, Y sorted)
- `unsorted-y-shuffle-3-3`: $vx = (1, 2, 3)$, $vy = (3, 1, 2)$ (X sorted, Y shuffled)
- `unsorted-both-shuffle-4-4`: $vx = (3, 1, 4, 2)$, $vy = (4, 2, 1, 3)$ (both shuffled)
- `unsorted-duplicates-mixed-5-5`: $vx = (3, 3, 3, 3, 3)$, $vy = (3, 3, 3, 3, 3)$ (all identical)
- `unsorted-x-unsorted-duplicates`: $vx = (2, 1, 3, 2, 1)$, $vy = (1, 1, 2, 2, 3)$ (X has unsorted duplicates)
- `unsorted-y-unsorted-duplicates`: $vx = (1, 1, 2, 2, 3)$, $vy = (3, 2, 1, 3, 2)$ (Y has unsorted duplicates)
- `unsorted-asymmetric-unsorted-2-5`: $vx = (2, 1)$, $vy = (5, 2, 4, 1, 3)$ (asymmetric sizes, both unsorted)
- `unsorted-negative-unsorted-3-3`: $vx = (-1, -3, -2)$, $vy = (-2, -3, -1)$ (negative unsorted)

These tests are critical for two-sample estimators because they verify that $vx$ and $vy$ are sorted *independently*.
The variety includes cases where only one sample is unsorted, ensuring implementations don't incorrectly assume pre-sorted input or sort samples together.

*Performance test* — validates the fast $O((m+n) log L)$ binary search algorithm:

- *Input*: $vx = (1, 2, 3, ..., 100000)$, $vy = (1, 2, 3, ..., 100000)$
- *Expected output*: $0$
- *Time constraint*: Must complete in under 5 seconds
- *Purpose*: Ensures that the implementation uses the efficient algorithm rather than materializing all $m n = 10$ billion pairwise differences

This test case is not stored in the repository because it generates a large JSON file (approximately 1.5 MB).
Each language implementation should manually implement this test with the hardcoded expected result.
