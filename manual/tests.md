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

The $\Center$ test suite contains 10 test cases validating the robust average estimator.

**Natural sequences** ($n = 1, 2, 3$) — canonical happy path examples:

- `natural-1`: $\x = (1)$, expected output: $1$
- `natural-2`: $\x = (1, 2)$, expected output: $1.5$
- `natural-3`: $\x = (1, 2, 3)$, expected output: $2$

**Zero values** ($n = 1, 2$) — edge case testing with zeros:

- `zeros-1`: $\x = (0)$, expected output: $0$
- `zeros-2`: $\x = (0, 0)$, expected output: $0$

**Additive distribution** ($n = 5, 10, 30$) — fuzzy testing with $\Additive(10, 1)$:

- `normal-5`, `normal-10`, `normal-30`: random samples generated with seed 0

**Uniform distribution** ($n = 5, 100$) — fuzzy testing with $\Uniform(0, 1)$:

- `uniform-5`, `uniform-100`: random samples generated with seed 1

The random samples validate that $\Center$ performs correctly on realistic distributions at various sample sizes.
The progression from small ($n = 5$) to large ($n = 100$) samples helps identify issues that only manifest at specific scales.

## Spread

$$
\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|
$$

The $\Spread$ test suite contains 10 test cases with identical structure to $\Center$.

**Natural sequences** ($n = 1, 2, 3$):

- `natural-1`: $\x = (1)$, expected output: $0$ (single element has zero dispersion)
- `natural-2`: $\x = (1, 2)$, expected output: $1$
- `natural-3`: $\x = (1, 2, 3)$, expected output: $1$

**Zero values** ($n = 1, 2$):

- `zeros-1`: $\x = (0)$, expected output: $0$
- `zeros-2`: $\x = (0, 0)$, expected output: $0$

**Additive distribution** ($n = 5, 10, 30$) — $\Additive(10, 1)$:

- `normal-5`, `normal-10`, `normal-30`: random samples generated with seed 0

**Uniform distribution** ($n = 5, 100$) — $\Uniform(0, 1)$:

- `uniform-5`, `uniform-100`: random samples generated with seed 1

The natural sequence cases validate the basic pairwise difference calculation.
The zero cases confirm that constant samples correctly produce zero spread.

## RelSpread

$$
\RelSpread(\x) = \frac{\Spread(\x)}{\left| \Center(\x) \right|}
$$

The $\RelSpread$ test suite contains 8 test cases focusing on relative dispersion.

**Natural sequences** ($n = 1, 2, 3$):

- `natural-1`: $\x = (1)$, expected output: $0$
- `natural-2`: $\x = (1, 2)$, expected output: $\approx 0.667$
- `natural-3`: $\x = (1, 2, 3)$, expected output: $0.5$

**Uniform distribution** ($n = 5, 10, 20, 30, 100$) — $\Uniform(0, 1)$:

- `uniform-5`, `uniform-10`, `uniform-20`, `uniform-30`, `uniform-100`: random samples generated with seed 0

The uniform distribution tests span multiple sample sizes to verify that $\RelSpread$ correctly normalizes dispersion.
The absence of zero-value tests reflects the domain constraint requiring $\Center(\x) \neq 0$.

## Shift

$$
\Shift(\x, \y) = \underset{1 \leq i \leq n,\,\, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)
$$

The $\Shift$ test suite contains 26 test cases covering two-sample comparisons.

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

**Zero values** ($[n, m] \in \{1, 2\} \times \{1, 2\}$) — 4 combinations:

- `zeros-1-1`, `zeros-1-2`, `zeros-2-1`, `zeros-2-2`: all produce output $0$

**Additive distribution** ($[n, m] \in \{5, 10, 30\} \times \{5, 10, 30\}$) — 9 combinations with $\Additive(10, 1)$:

- `normal-5-5`, `normal-5-10`, `normal-5-30`
- `normal-10-5`, `normal-10-10`, `normal-10-30`
- `normal-30-5`, `normal-30-10`, `normal-30-30`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The natural sequences validate anti-symmetry ($\Shift(\x, \y) = -\Shift(\y, \x)$) and the identity property ($\Shift(\x, \x) = 0$).
The asymmetric size combinations test the two-sample algorithm with unbalanced inputs.

## Ratio

$$
\Ratio(\x, \y) = \underset{1 \leq i \leq n, 1 \leq j \leq m}{\Median} \left( \dfrac{x_i}{y_j} \right)
$$

The $\Ratio$ test suite contains 22 test cases, excluding zero values due to division constraints.

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

- `normal-5-5`, `normal-5-10`, `normal-5-30`
- `normal-10-5`, `normal-10-10`, `normal-10-30`
- `normal-30-5`, `normal-30-10`, `normal-30-30`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The natural sequences verify the identity property ($\Ratio(\x, \x) = 1$) and validate ratio calculations with simple integer inputs.
Note that implementations should handle the practical constraint of avoiding division by values near zero.

## AvgSpread

$$
\AvgSpread(\x, \y) = \dfrac{n\Spread(\x) + m\Spread(\y)}{n + m}
$$

The $\AvgSpread$ test suite contains 26 test cases with identical structure to $\Shift$.

**Natural sequences** ($[n, m] \in \{1, 2, 3\} \times \{1, 2, 3\}$) — 9 combinations:

- All combinations from single-element to three-element samples, validating the weighted average calculation

**Zero values** ($[n, m] \in \{1, 2\} \times \{1, 2\}$) — 4 combinations:

- All produce output $0$ since $\Spread$ of constant samples is zero

**Additive distribution** ($[n, m] \in \{5, 10, 30\} \times \{5, 10, 30\}$) — 9 combinations with $\Additive(10, 1)$:

- Tests pooled dispersion across different sample size combinations
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- Validates correct weighting when sample sizes differ substantially
- Random generation: $\x$ uses seed 2, $\y$ uses seed 3

The asymmetric size combinations are particularly important for $\AvgSpread$ because the estimator must correctly weight each sample's contribution by its size.

## Disparity

$$
\Disparity(\x, \y) = \dfrac{\Shift(\x, \y)}{\AvgSpread(\x, \y)}
$$

The $\Disparity$ test suite contains 8 test cases — a reduced set reflecting the estimator's composite nature.

**Natural sequences** ($[n, m] \in \{2, 3\} \times \{2, 3\}$) — 4 combinations:

- `natural-2-2`, `natural-2-3`, `natural-3-2`, `natural-3-3`
- Minimum size $n, m \geq 2$ required for meaningful dispersion calculations

**Uniform distribution** ($[n, m] \in \{5, 100\} \times \{5, 100\}$) — 4 combinations with $\Uniform(0, 1)$:

- `uniform-5-5`, `uniform-5-100`, `uniform-100-5`, `uniform-100-100`
- Random generation: $\x$ uses seed 0, $\y$ uses seed 1

The smaller test set for $\Disparity$ reflects implementation confidence.
Since $\Disparity$ combines $\Shift$ and $\AvgSpread$, correct implementation of those components ensures $\Disparity$ correctness.
The test cases validate the division operation and confirm scale-free properties.

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

For $\Additive$ ('Normal') distributions, random values are generated using the Box-Müller transform,
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

This framework ensures that all seven language implementations maintain strict numerical agreement across the full test suite.