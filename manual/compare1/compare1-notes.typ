#import "/manual/definitions.typ": *

==== Verdict Boundary Condition

When $L = t$ (bounds lower equals threshold), the verdict is $upright("Inconclusive")$, not $upright("Greater")$.
When $U = t$ (bounds upper equals threshold), the verdict is $upright("Inconclusive")$, not $upright("Less")$.
The verdict is $upright("Greater")$ only when $L > t$ (strictly).

This conservative choice reflects the discrete nature of confidence bounds:
the true value could plausibly equal the boundary.

==== From Hypothesis Testing to Practical Thresholds

Compare1 embodies the #link(<sec-inversion-principle>)[Inversion Principle]:
instead of asking "Can I reject the hypothesis that Center equals zero?",
Compare1 answers "Is Center reliably greater than my practical threshold?"

Traditional hypothesis testing against zero may declare a $0.01%$ difference "statistically significant"
with large enough sample sizes, even when the difference is practically irrelevant.
Compare1 forces explicit specification of practical thresholds and returns a ternary verdict
($upright("Less")$, $upright("Greater")$, $upright("Inconclusive")$) that respects both statistical uncertainty and practical relevance.
