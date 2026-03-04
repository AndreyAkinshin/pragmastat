#import "/manual/definitions.typ": *

==== Verdict Boundary Condition

When $L = t$ (bounds lower equals threshold), the verdict is $upright("Inconclusive")$, not $upright("Greater")$.
When $U = t$ (bounds upper equals threshold), the verdict is $upright("Inconclusive")$, not $upright("Less")$.
The verdict is $upright("Greater")$ only when $L > t$ (strictly).

This conservative choice reflects the discrete nature of confidence bounds:
the true value could plausibly equal the boundary.

==== From Hypothesis Testing to Practical Thresholds

Compare2 extends the #link(<sec-inversion-principle>)[Inversion Principle] to two-sample comparisons.
Instead of testing "Is Shift significantly different from zero?",
Compare2 answers "Is Shift reliably greater than my practical threshold?"

A Shift of $5$ ms may be statistically significant (bounds exclude zero)
but practically inconclusive (bounds include your threshold of $10$ ms).
Traditional hypothesis testing declares this "significant" and stops;
Compare2 declares it $upright("Inconclusive")$ relative to the practical threshold.
