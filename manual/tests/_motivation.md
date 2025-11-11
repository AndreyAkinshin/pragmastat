## Motivation

The toolkit maintains seven implementations across different programming languages: Python, TypeScript, R, C#, Kotlin, Rust, and Go.
Each implementation must produce identical numerical results for all estimators.
Maintaining correctness across this diverse set of languages requires a rigorous reference test suite.

Reference tests serve three critical purposes:

- **Cross-language validation.** All implementations must pass identical test cases, ensuring consistent behavior regardless of language choice.
- **Regression prevention.** Changes to any implementation can be validated against the reference outputs to detect unintended modifications.
- **Implementation guidance.** The test cases provide concrete examples that guide developers in implementing the toolkit in new languages.

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
All test cases are defined programmatically, executed to produce expected outputs, and serialized to JSON.
Other implementations load these JSON files and verify that their estimators produce matching results within a given numerical tolerance.
