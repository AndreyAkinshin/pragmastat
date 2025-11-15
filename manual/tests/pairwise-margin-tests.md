## PairwiseMargin Tests

$$
\PairwiseMargin(n, m, \misrate)
$$

The $\PairwiseMargin$ test suite contains 346 correctness test cases (4 demo + 32 natural + 10 edge + 300 comprehensive grid).

**Demo examples** ($n = m = 30$) — from manual introduction:

- `demo-1`: $n=30$, $m=30$, $\misrate=10^{-6}$, expected output: $276$
- `demo-2`: $n=30$, $m=30$, $\misrate=10^{-5}$, expected output: $328$
- `demo-3`: $n=30$, $m=30$, $\misrate=10^{-4}$, expected output: $390$
- `demo-4`: $n=30$, $m=30$, $\misrate=10^{-3}$, expected output: $464$

These demo cases match the reference values used throughout the manual to illustrate $\ShiftBounds$ construction.

**Natural sequences** ($[n, m] \in \{1, 2, 3, 4\} \times \{1, 2, 3, 4\}$ × 2 misrates) — 32 tests:

- Misrate values: $\misrate \in \{10^{-1}, 10^{-2}\}$
- Test naming: `natural-{n}-{m}-mr{k}` where $k$ is the negative log10 of misrate
- Examples:
  - `natural-1-1-mr1`: $n=1$, $m=1$, $\misrate=0.1$, expected output: $0$
  - `natural-2-2-mr1`: $n=2$, $m=2$, $\misrate=0.1$, expected output: $0$
  - `natural-3-3-mr2`: $n=3$, $m=3$, $\misrate=0.01$, expected output: $0$
  - `natural-4-4-mr1`: $n=4$, $m=4$, $\misrate=0.1$, expected output: $4$

The natural sequences provide canonical examples with small, easily verified parameter values.

**Edge cases** — boundary condition validation:

- `boundary-min`: $n=1$, $m=1$, $\misrate=0.5$ (minimum samples, expected output: $0$)
- `boundary-zero-margin-small`: $n=2$, $m=2$, $\misrate=10^{-6}$ (misrate too strict, expected output: $0$)
- `boundary-loose`: $n=5$, $m=5$, $\misrate=0.9$ (very permissive misrate)
- `symmetry-2-5`: $n=2$, $m=5$, $\misrate=0.1$ (tests symmetry property)
- `symmetry-5-2`: $n=5$, $m=2$, $\misrate=0.1$ (symmetric counterpart, same output as above)
- `symmetry-3-7`: $n=3$, $m=7$, $\misrate=0.05$ (asymmetric sizes)
- `symmetry-7-3`: $n=7$, $m=3$, $\misrate=0.05$ (symmetric counterpart)
- `asymmetry-extreme-1-100`: $n=1$, $m=100$, $\misrate=0.1$ (extreme size difference)
- `asymmetry-extreme-100-1`: $n=100$, $m=1$, $\misrate=0.1$ (reversed extreme)
- `asymmetry-extreme-2-50`: $n=2$, $m=50$, $\misrate=0.05$ (highly unbalanced)

These edge cases validate correct handling of boundary conditions, the symmetry property $\PairwiseMargin(n, m, \misrate) = \PairwiseMargin(m, n, \misrate)$, and extreme asymmetry in sample sizes.

**Comprehensive grid** — systematic coverage for thorough validation:

Small sample combinations ($[n, m] \in \{1, 2, 3, 4, 5\} \times \{1, 2, 3, 4, 5\}$ × 6 misrates) — 150 tests:

- Misrate values: $\misrate \in \{10^{-1}, 10^{-2}, 10^{-3}, 10^{-4}, 10^{-5}, 10^{-6}\}$
- Test naming: `n{n}_m{m}_mr{k}` where $k$ is the negative log10 of misrate
- Examples:
  - `n1_m1_mr1`: $n=1$, $m=1$, $\misrate=0.1$, expected output: $0$
  - `n5_m5_mr1`: $n=5$, $m=5$, $\misrate=0.1$, expected output: $10$
  - `n5_m5_mr3`: $n=5$, $m=5$, $\misrate=0.001$, expected output: $0$

Large sample combinations ($[n, m] \in \{10, 20, 30, 50, 100\} \times \{10, 20, 30, 50, 100\}$ × 6 misrates) — 150 tests:

- Misrate values: same as small samples
- Test naming: `n{n}_m{m}_r{k}` where $k$ is the negative log10 of misrate
- Examples:
  - `n10_m10_r1`: $n=10$, $m=10$, $\misrate=0.1$, expected output: $56$
  - `n10_m10_r6`: $n=10$, $m=10$, $\misrate=10^{-6}$, expected output: $0$
  - `n50_m50_r3`: $n=50$, $m=50$, $\misrate=0.001$, expected output: $1556$
  - `n100_m100_r6`: $n=100$, $m=100$, $\misrate=10^{-6}$, expected output: $6060$

The comprehensive grid validates both symmetric ($n = m$) and asymmetric sample size combinations across six orders of magnitude in misrate, ensuring robust coverage of the parameter space.