#import "/manual/definitions.typ": *

= Assumptions <sec-assumptions>

This chapter defines the *domain assumptions* that govern Pragmastat functions.
Unlike parametric assumptions (Normal, LogNormal, Pareto), domain assumptions describe
which values are _meaningful inputs_ in the first place.

#v(0.3em)
#list(marker: none, tight: true,
  [*Domain over parametric* — assumptions define valid inputs, not distributional shape],
  [*Formal system* — each function declares required assumptions; a function is applicable iff all hold],
  [*Structured errors* — violations report assumption ID and subject, not ad-hoc strings],
)

#v(0.5em)
*Implicit Validity Assumption*

All functions implicitly require *valid samples*: non-empty, with finite defined real values
(no NaN, +Inf, -Inf). This shared constraint has formal ID `validity` but is not listed per function.

#v(0.5em)
*Hard vs. Weak Assumptions*

Hard assumptions (`validity`, `domain`, `positivity`, `sparity`) are enforced constraints —
violating them makes the function inapplicable and triggers a structured error.

Weak assumptions (e.g., weak continuity, weak distribution) are performance expectations —
estimators are designed to work well when they hold, but violations are never reported.
Weak assumptions are assessed through drift tables and simulation studies, not input validation.

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
== Positivity Assumption <sec-positivity>

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
  [`RatioBounds(x, y, misrate)` — both samples must be strictly positive.],
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
== Weak Symmetry Assumption <sec-weak-symmetry>

*Definition*

The data-generating distribution is assumed to be approximately symmetric
around its unknown center. This means deviations above and below the center
are equally likely in magnitude.

*Properties*

Weak symmetry is a performance expectation, not an enforced constraint:

#v(0.3em)
#list(marker: none, tight: true,
  [*Approximate symmetry* — The distribution need not be exactly symmetric;
  mild asymmetry produces mild coverage drift.],
  [*Ties tolerated* — Symmetry refers to the underlying process, not observed ties.],
  [*Not validated* — This is a modeling assumption, never checked computationally.],
  [*Violation behavior* — Asymmetric distributions cause coverage to drift from
  nominal; bounds remain valid but may be wider or narrower than requested.],
)

#v(0.5em)
*When symmetry is plausible*

#list(marker: none, tight: true,
  [Measurement error around a true value (additive errors)],
  [Physical quantities that can deviate equally in both directions],
  [Log-transformed multiplicative data (often more symmetric than raw)],
)

#v(0.5em)
*When symmetry is doubtful*

#list(marker: none, tight: true,
  [Durations, latencies, response times (right-skewed)],
  [Counts, concentrations, monetary values (often right-skewed)],
  [Data with natural lower bounds but no upper bounds],
)

#v(0.5em)
*Rule of thumb*

If skewness is the primary concern, use $MedianBounds$ (exact, no symmetry).
If you must target $Center$ despite asymmetry, use $CenterBoundsApprox$ (nominal).

#v(0.5em)
*Functions requiring weak symmetry*

#list(marker: none, tight: true,
  [$CenterBounds(vx, misrate)$ — requires weak symmetry for exact coverage.],
)

#v(0.5em)
*Relationship to weak continuity*

$CenterBounds$ requires BOTH weak continuity AND weak symmetry for exact coverage.
$MedianBounds$ and $CenterBoundsApprox$ require only weak continuity.

#pagebreak()
== Sparity Assumption <sec-sparity>

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

*Naming*

_Sparity_ is a Pragmastat term combining "spread" and "-ity" (the property suffix):
the property of having positive spread.
It can also be read as evoking "sparse" — pairwise differences are not dominated by zeros.
This follows the toolkit's generative naming principle:
the name encodes what the assumption checks (`Spread > 0`), not who defined it.

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
  [2. `domain`],
  [3. `positivity`],
  [4. `sparity`],
)

*Assumption ID registry*

#list(marker: none, tight: true,
  [*validity* — non-empty input with finite defined real values.],
  [*domain* — parameter value outside achievable range (e.g., misrate below minimum for given sample size).],
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

```json
{
  "id": "domain",
  "subject": "misrate"
}
```

*Recommendations (generated from IDs)*

#list(marker: none, tight: true,
  [*validity* — provide at least one finite real value; remove NaN/Inf before analysis.],
  [*domain* — parameter value exceeds data resolution; for misrate violations, increase sample size or use a larger misrate value (see minimum achievable misrate tables).],
  [*positivity* — if all values are strictly negative, multiply by $-1$; if mixed-sign, use sign-agnostic estimators or split by sign.],
  [*sparity* — tie-dominant samples have no usable spread; increase measurement resolution, use a discrete/ordinal framework, or preprocess before applying spread-based tools.],
)

*Example minimal error string*

```
positivity(x)
```
