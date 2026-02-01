#import "/manual/definitions.typ": *

= Methodology

This chapter examines the methodological principles that guide the pragmastat's design and application.

== Pragmatic Philosophy

The toolkit's foundations rest on pragmatist epistemology:
truth is determined by practical consequences, not abstract correspondence with reality.

#v(0.3em)
#list(marker: none, tight: true,
  [*Truth is what works* — An estimator is "correct" if it produces useful results across realistic conditions],
  [*Meaning from consequences* — The value of a statistical method lies in what it enables, not its theoretical elegance],
  [*Theory serves practice* — Mathematical analysis provides insight, but empirical validation determines adoption],
  [*Utility as criterion* — When methods conflict, prefer the one that solves more real problems],
)

This stance inverts the traditional relationship between theory and practice.
Rather than deriving methods from first principles and hoping they apply,
we evaluate methods by their performance and seek theoretical understanding afterward.

#pagebreak()
== Procedure-First Empiricism

Traditional statistical practice follows an assumptions-first methodology:

+ Assume a data-generating model (e.g., "observations are normally distributed")
+ Derive the optimal procedure under those assumptions
+ Apply the procedure to data, hoping assumptions approximately hold

This toolkit inverts the process:

+ Select procedures based on desired properties (robustness, equivariance, interpretability)
+ Empirically measure performance across a wide range of conditions
+ Use theory to explain and predict observed behavior

Monte Carlo simulation serves as the primary instrument of knowledge.
Rather than deriving asymptotic formulas for estimator variance,
we measure actual variance across thousands of simulated samples.
Drift tables in this manual are empirically measured, not analytically derived.

This approach has practical advantages:
simulations can explore conditions that resist closed-form analysis,
and empirical results are self-validating — they show what actually happens,
not what theory predicts should happen.

For the formal treatment of domain assumptions that govern valid inputs,
see the #link(<sec-assumptions>)[Assumptions] chapter.

#pagebreak()
== Epistemic Humility

No perfectly Gaussian, log-normal, or Pareto distributions exist in real data.
Every distribution we name is a useful fiction — a model we employ because it approximates reality well enough for our purposes,
while knowing it cannot be exactly correct.

#v(0.3em)
#list(marker: none, tight: true,
  [*Models are approximations* — They capture essential structure while ignoring irrelevant details],
  [*Approximations fail at boundaries* — Edge cases, extreme values, and distribution tails often violate assumptions],
  [*Graceful degradation* — Methods should produce sensible (if less precise) results when assumptions weaken],
)

The toolkit embodies this humility by choosing estimators that remain interpretable and bounded
even when distributional assumptions break down.
A robust estimator may sacrifice some efficiency under ideal conditions
in exchange for reliable behavior when conditions degrade.

#pagebreak()
== The Pairwise Principle

A structural insight unifies all primary robust estimators in this toolkit:
they are medians of pairwise operations.

#v(0.5em)
#table(
  columns: 3,
  [*Estimator*], [*Pairwise Operation*], [*Result*],
  [$Center$], [$(x_i + x_j) \/ 2$], [Median of pairwise averages],
  [$Spread$], [$abs(x_i - x_j)$], [Median of pairwise differences],
  [$Shift$], [$x_i - y_j$], [Median of cross-sample differences],
  [$Dominance$], [$bold(1)(x_i > y_j)$], [Proportion of pairwise comparisons],
)

#v(0.5em)
This pairwise structure provides three benefits:

#v(0.3em)
#list(marker: none, tight: true,
  [*Natural robustness* — Comparing measurements to each other, not to external references, limits outlier influence],
  [*Self-calibration* — The sample serves as its own reference distribution, requiring no external assumptions],
  [*Algebraic closure* — Pairwise operations preserve symmetry and equivariance properties],
)

The pairwise principle also enables efficient computation.
Matrices of pairwise operations have structural properties (sorted rows and columns)
that fast algorithms exploit to achieve $O(n log n)$ complexity.

#pagebreak()
== Median as Universal Aggregator

The median is the final step in each pairwise estimator.
Why median specifically?

The median achieves the maximum possible breakdown point (50%)
among all translation-equivariant location estimators.
Up to half the data can be arbitrarily corrupted before the median becomes unbounded.

However, $Center$ and $Spread$ achieve only 29% breakdown — not 50%.
This is deliberate: a tradeoff between robustness and precision.

#v(0.5em)
#table(
  columns: 4,
  [*Breakdown*], [*Robustness*], [*Precision*], [*Estimators*],
  [0%], [None], [Optimal under assumptions], [$Mean$, $StdDev$],
  [29%], [Substantial], [Near-optimal], [$Center$, $Spread$],
  [50%], [Maximum], [Reduced], [$Median$, $MAD$],
)

#v(0.5em)
The 29% breakdown point survives approximately one corrupted measurement in four
while maintaining roughly 95% asymptotic efficiency under ideal Gaussian conditions.
This represents the practical optimum: enough robustness for realistic contamination levels,
enough efficiency to compete with traditional methods when data is clean.

#pagebreak()
== Convergence Conventions

Drift normalizes estimator variability by $sqrt(n)$, making precision comparable across sample sizes:

$ Drift = Spread("estimates") times sqrt(n) $

This normalization embeds a deliberate assumption: most useful estimators converge at the $sqrt(n)$ rate.
The Central Limit Theorem guarantees this rate for means under mild conditions,
and median-based estimators inherit similar convergence behavior.

#v(0.3em)
#list(marker: none, tight: true,
  [*Common case default* — $sqrt(n)$ convergence covers the vast majority of practical estimators],
  [*Intuitive interpretation* — Drift represents "effective standard deviation at $n = 1$"],
  [*Mental calculation* — Expected precision at any $n$ is simply $Drift \/ sqrt(n)$],
)

#v(0.5em)
For estimators with non-standard convergence (e.g., extreme value statistics),
drift generalizes to $n^("instability")$ where instability differs from $0.5$.
But the toolkit deliberately uses $sqrt(n)$ throughout because it matches the common case
and provides intuitive interpretation without complicating the universal mechanism.

This is pragmatic universalism: adopt the common case as default,
acknowledge exceptions exist, and handle them explicitly rather than
burdening the common case with unnecessary generality.

#pagebreak()
== Structural Unity

All robust estimators in this toolkit share a common mathematical structure:

$ "Estimator" = Median("Pairwise Operations") $

This structural unity is not merely aesthetic — it enables unified algorithmic optimization.

#v(0.3em)
#list(marker: none, tight: true,
  [*Sorted structure* — Matrices of pairwise operations have sorted rows and columns],
  [*Monahan's algorithm* — Exploits sorted structure for $O(n log n)$ $Center$/$Spread$],
  [*Fast shift* — Exploits cross-sample matrix structure for efficient two-sample comparison],
)

Because all estimators share the same "median of pairwise" form,
insights that accelerate one can often be adapted to accelerate others.
A single theoretical framework covers all primary estimators.

#pagebreak()
== Generative Naming

Names in this toolkit encode operational knowledge rather than historical provenance.

#v(0.5em)
#table(
  columns: 3,
  [*Traditional*], [*Pragmastat*], [*What's Encoded*],
  [Gaussian / Normal], [$Additive$], [Formation: sum of independent factors (CLT)],
  [Log-normal / Galton], [$Multiplic$], [Formation: product of independent factors],
  [Pareto], [$Power$], [Behavior: power-law relationship],
  [Hodges-Lehmann], [$Center$], [Function: measures central tendency],
  [Shamos], [$Spread$], [Function: measures variability],
  [(none)], [sparity], [Assumption: property of having positive spread],
)

#v(0.5em)
When you read "$Additive$", your mind activates a generative model:
this distribution arises when many independent factors add together.
When you read "Gaussian", you must recall an association with Carl Friedrich Gauss,
then remember what properties that name implies.

Generative names create immediate intuition about when a model applies.
$Additive$ distributions arise from additive processes.
$Multiplic$ distributions arise from multiplicative processes.
The name itself encodes the formation mechanism.

#pagebreak()
== The Inversion Principle

Traditional statistical outputs often require mental transformation before use.
This toolkit inverts such framings to present information in directly actionable form,
  following principles of user-centered design (@norman2013).

#v(0.5em)
#table(
  columns: 3,
  [*Traditional*], [*Pragmastat*], [*Reason for Inversion*],
  [Confidence level (95%)], [$misrate$ (0.05)], [Direct error interpretation],
  [Confidence interval], [Bounds], [Plain language, no jargon],
  [Hypothesis test (p-value)], [Bounds estimation], ["What's plausible?" not "Is zero plausible?"],
  [Efficiency (variance ratio)], [Drift (spread-based)], [Works with heavy tails],
)

#v(0.5em)
Consider the confidence level vs. misrate inversion.
A "95% confidence interval" requires understanding:
"If I repeated this procedure infinitely, 95% of intervals would contain the true value."
A "5% misrate" states directly: "This procedure errs about 5% of the time."

The shift from confidence intervals to bounds, and from hypothesis testing to interval estimation,
moves from frequentist theology toward decision-relevant inference.
The practitioner asks "What values are plausible for this parameter?"
rather than "Can I reject the hypothesis that this parameter equals zero?"

#pagebreak()
== Multi-Audience Design

This manual serves readers with diverse backgrounds and conflicting preferences:

#v(0.5em)
#table(
  columns: 3,
  [*Audience*], [*Priorities*], [*Challenges*],
  [Experienced academics], [Rigor, derivation, formalism, citations], [May find practical focus too shallow],
  [Professional developers], [Examples, APIs, searchability, minimalism], [May find theory intimidating],
  [Students and beginners], [Clarity, intuition, progressive disclosure], [Need both theory and practice],
  [Large language models], [Structure, consistency, unambiguous definitions], [Need form-independent content],
)

#v(0.5em)
These audiences have conflicting needs.
Academics want complete derivations; developers want quick answers.
Beginners need gentle introductions; experts need dense references.
LLMs need predictable structure; humans appreciate variety.

The manual targets a "neutral zone" where all audiences find acceptable content:

#v(0.3em)
#list(marker: none, tight: true,
  [*Signature first* — Mathematical definition immediately visible],
  [*Example second* — Concrete computation before abstract explanation],
  [*Detail optional* — Properties, corner cases, and theory follow for those who need them],
  [*Every sentence earns its place* — No filler prose, no redundant explanation],
)

#v(0.5em)
*Structural Principles*

#v(0.3em)
#list(marker: none, tight: true,
  [*Concrete over abstract* — Numbers and examples before symbols and theory],
  [*Precision without verbosity* — Mathematical rigor in minimal words],
  [*Consistent layout* — Same structure across all toolkit items enables scanning],
  [*Self-contained sections* — Each section readable independently],
)

#v(0.5em)
*LLM-Friendliness*

The manual's structure also serves machine readers:

#v(0.3em)
#list(marker: none, tight: true,
  [*Predictable patterns* — Consistent section ordering aids extraction],
  [*Explicit definitions* — No implicit knowledge assumed],
  [*Tabular data* — Structured information in tables, not prose],
  [*Short paragraphs* — Content chunks cleanly for context windows],
)

This multi-audience optimization forces elimination of audience-specific conventions,
revealing form-independent essential content that serves everyone adequately
rather than serving one group perfectly and others poorly.

#pagebreak()
== Reference Tests as Specification

The toolkit maintains seven implementations across different programming languages:
Python, TypeScript, R, C\#, Kotlin, Rust, and Go.
Each implementation must produce identical numerical results for all estimators.

This cross-language consistency is achieved through executable specifications:

#block(inset: (left: 1em))[
```
Manual (definitions) ↔ C# (reference) → JSON (tests) → All languages (validation)
```
]

#v(0.3em)
The specification IS the test suite.
Reference tests serve three critical purposes:

#v(0.3em)
#list(marker: none, tight: true,
  [*Cross-language validation* — All implementations pass identical test cases],
  [*Regression prevention* — Changes validated against known outputs],
  [*Implementation guidance* — Concrete examples for porting to new languages],
)

#v(0.5em)
*Test Design Principles*

#v(0.3em)
#list(marker: none, tight: true,
  [*Minimal sufficiency* — Smallest test set providing high confidence in correctness],
  [*Comprehensive coverage* — Both typical cases and edge cases that expose errors],
  [*Deterministic reproducibility* — Fixed seeds for all random tests],
)

#v(0.5em)
*Test Categories*

#v(0.3em)
#list(marker: none, tight: true,
  [*Canonical cases* — Deterministic inputs like natural number sequences where outputs are easily verified],
  [*Edge cases* — Boundary conditions: single element, zeros, minimum viable sample sizes],
  [*Fuzzy tests* — Controlled random exploration beyond hand-crafted examples],
)

#v(0.5em)
The C\# implementation serves as the reference generator.
All test cases are defined programmatically, executed to produce expected outputs, and serialized to JSON.
Other implementations load these JSON files and verify their outputs match within numerical tolerance.

#pagebreak()
== Cross-Language Determinism

Reproducibility requires determinism at every layer.
When a simulation in Python produces a result,
the same simulation in Rust, Go, or any other supported language must produce the identical result.

#v(0.3em)
#list(marker: none, tight: true,
  [*Portable RNG* — $Rng("experiment-1")$ produces identical sequences in all languages],
  [*Specified algorithms* — xoshiro256++ for generation, SplitMix64 for seeding, FNV-1a for string hashing],
  [*No implementation-dependent behavior* — Floating-point operations follow IEEE 754],
)

#v(0.5em)
*Unified API*

Beyond numerical determinism, the toolkit maintains a consistent API across all implementations.
Function names, parameter orders, and return types follow the same conventions in every language.

#v(0.3em)
#list(marker: none, tight: true,
  [*Same vocabulary* — $Center$, $Spread$, $Shift$ mean the same thing everywhere],
  [*Same signatures* — `Center(x)` in Python, `Center(x)` in Rust, `Center(x)` in Go],
  [*Same behavior* — Edge cases, error conditions, and defaults are identical],
)

This unified API enables frictionless language switching.
A practitioner prototyping in Python can port to Rust for production
without learning new abstractions or revalidating statistical assumptions.
The mental model transfers directly; only syntax changes.

#v(0.5em)
*Benefits of Unification*

#list(marker: none, tight: true,
  [*Debugging across languages* — A failing test in TypeScript can be debugged in C\#],
  [*Verified ports* — New implementations can be validated against existing ones],
  [*Reproducible research* — Results can be reproduced in any supported language],
  [*Team flexibility* — Different team members can use preferred languages on the same analysis],
  [*Migration paths* — Move from prototype to production without statistical revalidation],
)

#pagebreak()
== Summary Principles

The methodology of this toolkit can be distilled into twelve guiding principles:

+ *Name things by what they do, not who discovered them* — Generative names encode operational knowledge
+ *All models are wrong; design for graceful degradation* — Robust methods fail gently
+ *Evaluate empirically, organize theoretically* — Simulation before derivation
+ *Self-reference provides robustness* — Pairwise operations compare data to itself
+ *29% breakdown is the practical optimum* — Balance robustness and precision
+ *Invert framings that require mental transformation* — Present directly actionable information
+ *Default to the common case* — Use $sqrt(n)$ convergence; handle exceptions explicitly
+ *Multi-audience optimization reveals essential content* — Serve everyone adequately, not one group perfectly
+ *Executable specifications are reliable specifications* — Tests define correctness
+ *Reproducibility requires portable determinism* — Same seeds, same results, any language
+ *Structural unity enables unified optimization* — "Median of pairwise" admits fast algorithms
+ *Utility is the ultimate criterion* — Methods that solve real problems are correct methods
