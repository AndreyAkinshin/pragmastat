#import "/manual/definitions.typ": *

= Assumptions

This chapter defines the *domain assumptions* that govern Pragmastat functions.
Unlike parametric assumptions (Normal, LogNormal, Pareto), domain assumptions describe
which values are _meaningful inputs_ in the first place.

They form a formal system:
every function declares the non-trivial assumptions it requires,
and a function is applicable only when its assumptions hold.

#pagebreak()
== Assumption Framework

*What assumptions are in Pragmastat*

Assumptions in Pragmastat are *domain constraints*, not statistical models.
Practitioners often do not know which parametric distribution fits,
but they do know the domain of their measurements:
"Are values strictly positive?", "Can zeros occur?", "Do ties have physical meaning?".

This chapter makes those constraints explicit and operational.

#v(0.3em)
#list(marker: none, tight: true,
  [*Domain over parametric* — assumptions define valid inputs, not distributional shape],
  [*Formal system* — each function declares a list of required assumptions],
  [*Decision rule* — a function is applicable iff all its assumptions hold],
  [*Robustness is not a substitute* — invalid-domain values must be cleaned or transformed],
)

#v(0.5em)
*Implicit Validity Assumption*

All functions in Pragmastat implicitly assume that each sample is a *valid sample*.
This is the shared domain of the toolkit and is not listed per function.

#list(marker: none, tight: true,
  [*Non-empty input* — every sample must contain at least one value],
  [*Finite defined real values* — no NaN, +Inf, or -Inf; all values must be representable real numbers],
)

Because this is implicit, it does not appear in function-specific assumption lists,
but it *does* have a formal ID (`validity`) and is reported in structured violations when it fails.

Parameter validity (e.g., $misrate$ ranges) is a regular requirement handled by each implementation
and is not part of the assumption system.

#v(0.5em)
*Decision strategy*

#list(marker: none, tight: true,
  [1. Ensure samples are valid (non-empty, finite, defined real numbers).],
  [2. Identify domain constraints (positivity, continuity, sparity).],
  [3. Select functions whose assumptions match those constraints.],
  [4. If needed, transform data (e.g., multiply by $-1$ for strictly negative data).],
)

#v(0.5em)
*Common decisions*

#list(marker: none, tight: true,
  [*Data may be negative* — use sign-agnostic estimators such as $Shift$, $Center$, $Spread$.],
  [*Data strictly positive* — ratio-based tools and log-space estimators become available.],
)

#v(0.5em)
*Hard vs. weak assumptions*

Assumptions in the registry are *hard constraints*: violating them makes the function inapplicable.
Pragmastat also documents *weak assumptions* (e.g., weak continuity and weak distribution assumptions)
as performance or design expectations; weak assumptions are not enforced and never produce violations.

#v(0.5em)
*Assumptions are a contract*

Each function provides an *Assumptions* line in its definition when non-trivial assumptions apply.
Those assumptions define the _domain support_ of the estimator and
the behavior expected across all implementations.

#pagebreak()
== Weak Distribution Assumption

*Definition*

We assume that real data are _often close_ to common generative families such as
$Additive$, $Multiplic$, $Power$, and $Uniform$.
This is not a requirement for validity.
It is a pragmatic *performance expectation*: estimators should work well on these
frequent real-world cases.

*Why it matters*

Pragmastat does not commit to a single "main" distribution.
Instead, it requires robust behavior across the distributions practitioners actually see.
This is consistent with procedure-first empiricism:
we select estimators by properties and validate them across typical generative families.

*Implication*

Weak distribution assumptions are not enforced by validation
and never produce errors.
They are assessed through drift tables, tests, and simulation studies,
not by checking inputs.

#pagebreak()
== Positivity Assumption

*Definition*

All values must be strictly positive: $x_i > 0$ for every element.

*Why positivity is fundamental*

Most physical measurements are positive by construction:
time, duration, length, mass, energy, concentration, price, latency.
Even "zero" is rarely a physical reality; processes do not occur instantly.
Zero and negative values are mathematical constructs used for convenience,
not typical states of the physical world.

This makes positivity a *pragmatic* assumption:
it reflects how real measurements are produced.
When zero or negative values appear in a positive-only domain,
they usually indicate measurement error, recording error, or preprocessing artifacts.

*Asymmetry of extremes*

Positivity implies a hard left boundary at zero,
so extremes are typically right-tailed.
This asymmetry is structural, not a violation.
It motivates ratio-based and log-space estimators that respect multiplicative scales.

*Workflow guidance*

#list(marker: none, tight: true,
  [*Strictly negative data* — multiply by $-1$, then apply positivity-based tools.],
  [*Mixed signs* — use sign-agnostic estimators such as $Shift$, $Center$, $Spread$.],
)

*Functions requiring positivity*

#list(marker: none, tight: true,
  [`Ratio(x, y)` — both samples must be strictly positive.],
  [`RelSpread(x)` — sample must be strictly positive (ensures $Center > 0$).],
)

#pagebreak()
== Weak Continuity Assumption

*Definition*

The data-generating process is assumed to be continuous,
meaning the probability of exact ties is zero in the underlying distribution.

In practice, ties occur because of measurement resolution
and finite precision in computer arithmetic.
Pragmastat treats ties as *artifacts of rounding*, not as meaningful properties of the data.

*Design implication*

Pragmastat does *not* introduce special tie-correction hacks.
All estimators handle ties naturally without adjustment.
This is pragmatic: device limitations belong in data cleaning,
not in the estimator definition.

*Why this matters for bounds*

Functions like $ShiftBounds$ and $PairwiseMargin$ compute distribution-free bounds
based on the assumption that pairwise comparisons have no ties in expectation.
When ties are present, bounds remain valid but may be slightly conservative.
Weak continuity is a weak assumption and is never reported as a violation.

Ties are tolerated as artifacts, but *tie dominance is a hard boundary*.
If ties dominate pairwise differences, the median pairwise distance collapses to zero,
and spread-based estimators are no longer meaningful. This is enforced by the sparity assumption.

#pagebreak()
== Sparity Assumption

*Naming*

_Sparity_ is a Pragmastat term combining "spread" and "-ity" (the property suffix):
the property of having positive spread.
It can also be read as evoking "sparse" — pairwise differences are not dominated by zeros.

This follows the toolkit's generative naming principle:
the name encodes what the assumption checks (`Spread > 0`), not who defined it.

*Definition*

The sample must be *non tie-dominant*:
the median of pairwise absolute differences must be strictly positive.
Equivalently, `Spread(x) > 0`.

Tie-dominant samples are those where at least half of all pairwise differences are zero,
which collapses the typical pairwise distance to zero.
This can happen even when `min(x) < max(x)`, so a min/max check is not sufficient.

*Why it matters*

Spread is defined as the median of pairwise differences.
If the median is zero, variability is not just small — it is not identifiable at the toolkit's scale.
The sparity assumption captures this and prevents tie-dominant samples from entering spread-based estimators.

*Implication*

Sparity automatically implies the sample has at least two elements.
For functions requiring sparity, this is the primary assumption to check.
A sample with $n = 1$ fails sparity because `Spread(x) = 0`, not because of a separate size requirement.

*Functions requiring sparity*

#list(marker: none, tight: true,
  [`Spread(x)` — one-sample spread requires sparity.],
  [`AvgSpread(x, y)` — both samples must have sparity.],
  [`Disparity(x, y)` — both samples must have sparity.],
)

#pagebreak()
== Assumption IDs and Violation Reporting

Assumption validation must be *structured* across all languages.
Errors should report _which assumption failed_,
not ad-hoc strings like "values must be positive".

Only one violation is reported per error. The violation is selected using a canonical priority order,
and for two-sample functions the first failing sample (`x` before `y`) is reported.

*Canonical priority order*

#list(marker: none, tight: true,
  [1. `validity`],
  [2. `positivity`],
  [3. `sparity`],
)

*Assumption ID registry*

#list(marker: none, tight: true,
  [*validity* — non-empty input with finite defined real values.],
  [*positivity* — values must be strictly positive.],
  [*sparity* — requires `Spread(x) > 0`; sample must be non tie-dominant.],
)

IDs are stable across languages and are the primary contract for error handling and localization.
Weak assumptions (e.g., weak continuity) are documented in the chapter but are not part of this registry.

*Violation schema (language-agnostic)*

```json
{
  "id": "positivity",
  "subject": "x"
}
```

*Recommendations (generated from IDs)*

#list(marker: none, tight: true,
  [*validity* — provide at least one finite real value; remove NaN/Inf before analysis.],
  [*positivity* — if all values are strictly negative, multiply by $-1$; if mixed-sign, use sign-agnostic estimators or split by sign.],
  [*sparity* — tie-dominant samples have no usable spread; increase measurement resolution, use a discrete/ordinal framework, or preprocess before applying spread-based tools.],
)

*Example minimal error string*

```
positivity(x)
```
