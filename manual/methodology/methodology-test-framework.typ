#import "/manual/definitions.typ": *

The reference test framework consists of three components:

*Test generation* — The C\# implementation defines test inputs programmatically using builder patterns.
For deterministic cases, inputs are explicitly specified.
For random cases, the framework uses controlled seeds with `System.Random` to ensure reproducibility across all platforms.

The random generation mechanism works as follows:

- Each test suite builder maintains a seed counter initialized to zero.
- For one-sample estimators, each distribution type receives the next available seed.
  The same random generator produces all samples for all sizes within that distribution.
- For two-sample estimators, each pair of distributions receives two consecutive seeds:
  one for the $vx$ sample generator and one for the $vy$ sample generator.
- The seed counter increments with each random generator creation, ensuring deterministic test data generation.

Every parametric family draws by inverse transform: one draw $U$ from $Uniform(0, 1)$ gives a
  sample as that family's own quantile applied to $U$.
For $Additive$, that is

$ X = mu + sigma dot AdditiveQuantile(U) $

where $mu$ is the mean and $sigma$ is the standard deviation.
For $Uniform$, the quantile is affine and the identity reads directly as

$ X = min + U dot (max - min) . $

The framework executes the reference implementation on all generated inputs and serializes input-output pairs to JSON format.

*Test validation* — Each language implementation loads the JSON test cases and executes them against its local estimator implementation.
Assertions verify that outputs match expected values within a given numerical tolerance (typically $10^(-10)$ for relative error).

*Test data format* — Each test case is a JSON file containing `input` and `output` fields.
For one-sample estimators, the input contains array `x` and optional `parameters`.
For two-sample estimators, input contains arrays `x` and `y`.
For bounds estimators ($ShiftBounds$, $RatioBounds$), input additionally contains `misrate`.
Output is a single numeric value for point estimators, or an object with `lower` and `upper` fields for bounds estimators.

*Performance testing* — The toolkit provides $O(n log n)$ implementations for $Center$, $Spread$, and $Shift$ estimators,
dramatically more efficient than naive implementations that materialize all pairwise combinations.
Performance tests use sample size $n = 100,000$ (for one-sample) or $n = m = 100,000$ (for two-sample).
This specific size creates a clear performance distinction:
the implementations ($O(n log n)$ or $O((m+n) log L)$) complete in under 5 seconds on modern hardware across all supported languages,
while naive implementations ($O(n^2 log n)$ or $O(m n log(m n))$) would be prohibitively slow (taking hours or failing due to memory exhaustion).
With $n = 100,000$, naive approaches would need to materialize approximately 5 billion pairwise values for $Center$/$Spread$
or 10 billion for $Shift$, whereas the implementations require only $O(n)$ additional memory.
Performance tests serve dual purposes: correctness validation at scale and performance regression detection,
ensuring implementations use the efficient algorithms and remain practical for real-world datasets with hundreds of thousands of observations.
Performance test specifications are provided in the respective estimator sections above.

This framework ensures that all seven language implementations maintain strict numerical agreement across the full test suite.
