#import "/manual/definitions.typ": *

$ PairwiseMargin(n, m, misrate) $

The $PairwiseMargin$ test suite contains 178 test cases (4 demo + 4 natural + 10 edge + 12 small grid + 148 large grid).
The domain constraint $misrate >= 2 / binom(n+m, n)$ is enforced; inputs violating this return a domain error.
Combinations where the requested misrate falls below the minimum achievable misrate are excluded from the grid.

*Demo examples* ($n = m = 30$) — from manual introduction:

- `demo-1`: $n=30$, $m=30$, $misrate=10^(-6)$, expected output: $276$
- `demo-2`: $n=30$, $m=30$, $misrate=10^(-5)$, expected output: $328$
- `demo-3`: $n=30$, $m=30$, $misrate=10^(-4)$, expected output: $390$
- `demo-4`: $n=30$, $m=30$, $misrate=10^(-3)$, expected output: $464$

These demo cases match the reference values used throughout the manual to illustrate $ShiftBounds$ construction.

*Natural sequences* ($[n, m] in {1, 2, 3, 4} times {1, 2, 3, 4}$, filtered by minimum achievable misrate) — 4 tests:

- After filtering, only four loose-misrate combinations survive: $(3, 3)$, $(3, 4)$, $(4, 3)$, and $(4, 4)$.
- The symmetric $(3, 3)$ case produces margin $0$.

*Edge cases* — boundary condition validation (10 tests):

- `boundary-min`: $n=1$, $m=1$, maximum achievable misrate (expected output: $0$)
- `boundary-zero-margin-small`: $n=20$, $m=20$, $misrate=10^(-6)$ (strict misrate with sufficient samples)
- `boundary-loose`: $n=5$, $m=5$, very loose achievable misrate
- `symmetry-2-5`: $n=2$, $m=5$ (tests symmetry property)
- `symmetry-5-2`: $n=5$, $m=2$ (symmetric counterpart, same output as above)
- `symmetry-3-7`: $n=3$, $m=7$ (asymmetric sizes)
- `symmetry-7-3`: $n=7$, $m=3$ (symmetric counterpart)
- `asymmetry-extreme-1-100`: $n=1$, $m=100$ (extreme size difference)
- `asymmetry-extreme-100-1`: $n=100$, $m=1$ (reversed extreme)
- `asymmetry-extreme-2-50`: $n=2$, $m=50$ (highly unbalanced)

These edge cases validate correct handling of boundary conditions, the symmetry property $PairwiseMargin(n, m, misrate) = PairwiseMargin(m, n, misrate)$, and extreme asymmetry in sample sizes.

*Comprehensive grid* — systematic coverage for thorough validation:

Small sample combinations ($[n, m] in {1, 2, 3, 4, 5} times {1, 2, 3, 4, 5}$ × 6 misrates, filtered) — 12 tests:

- Misrates span six orders of magnitude, from loose achievable values down to $10^(-6)$.
- Combinations where $misrate < 2 / binom(n+m, n)$ are excluded
- Test naming: `n{n}_m{m}_mr{k}` where $k$ is the negative log10 of misrate

Large sample combinations ($[n, m] in {10, 20, 30, 50, 100} times {10, 20, 30, 50, 100}$ × 6 misrates, filtered) — 148 tests:

- Misrate values: same as small samples
- Combinations where $misrate < 2 / binom(n+m, n)$ are excluded (affects $n = m = 10$ at misrates $10^(-5)$ and $10^(-6)$)
- Test naming: `n{n}_m{m}_r{k}` where $k$ is the negative log10 of misrate
- Examples:
  - `n50_m50_r3`: $n=50$, $m=50$, $misrate=0.001$, expected output: $1556$
  - `n100_m100_r6`: $n=100$, $m=100$, $misrate=10^(-6)$, expected output: $6060$

The comprehensive grid validates both symmetric ($n = m$) and asymmetric sample size combinations across six orders of magnitude in misrate, ensuring robust coverage of the parameter space.
