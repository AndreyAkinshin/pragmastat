## From Confidence Level to Misrate

Traditional statistics expresses uncertainty through confidence levels:
  "95% confidence interval", "99% confidence", "99.9% confidence".
This convention emerged from early statistical practice
  when tables printed confidence intervals for common levels like 90%, 95%, and 99%.

The confidence level approach creates practical problems:

- **Cognitive difficulty with high confidence**.
  Distinguishing between 99.999% and 99.9999% confidence requires mental effort.
  The difference matters — one represents a 1-in-100,000 error rate, the other 1-in-1,000,000 —
  but the representation obscures this distinction.
- **Asymmetric scale**.
  The confidence level scale compresses near 100%, where most practical values cluster.
  Moving from 90% to 95% represents a 2× change in error rate,
  while moving from 99% to 99.9% represents a 10× change, despite similar visual spacing.
- **Indirect interpretation**.
  Practitioners care about error rates, not success rates.
  "What's the chance I'm wrong?" matters more than "What's the chance I'm right?"
  Confidence level forces mental subtraction to answer the natural question.
- **Unclear defaults**.
  Traditional practice offers no clear default confidence level.
  Different fields use different conventions (95%, 99%, 99.9%),
  creating inconsistency and requiring arbitrary choices.

The $\misrate$ provides a more natural representation.
Misrate expresses the probability that computed bounds fail to contain the true value:

$$
\misrate = 1 - \text{confidence level}
$$

This simple inversion provides several advantages:

- **Direct interpretation**.
  $\misrate = 0.01$ means "1% chance of error" or "wrong 1 time in 100".
  $\misrate = 10^{-6}$ means "wrong 1 time in a million".
  No mental arithmetic required.
- **Linear scale for practical values**.
  $\misrate = 0.1$ (10%), $\misrate = 0.01$ (1%), $\misrate = 0.001$ (0.1%)
  form a natural sequence.
  Scientific notation handles extreme values cleanly: $10^{-3}$, $10^{-6}$, $10^{-9}$.
- **Clear comparisons**.
  $10^{-5}$ versus $10^{-6}$ immediately shows a 10× difference in error tolerance.
  99.999% versus 99.9999% confidence obscures this same relationship.
- **Pragmatic default**.
  The toolkit recommends $\misrate = 10^{-6}$ (one-in-a-million error rate)
  as a reasonable default for most applications.
  This represents extremely high confidence (99.9999%)
  while remaining computationally practical and conceptually clear.

The terminology shift from "confidence level" to "misrate"
  parallels other clarifying renames in this toolkit.
Just as $\Additive$ better describes the distribution's formation than 'Normal',
  and $\Center$ better describes the estimator's purpose than 'Hodges-Lehmann',
  $\misrate$ better describes the quantity practitioners actually reason about:
  the probability of error.

Traditional confidence intervals become "bounds" in this framework,
  eliminating statistical jargon in favor of descriptive terminology.
$\ShiftBounds(\x, \y, \misrate)$ clearly indicates:
  it provides bounds on the shift, with a specified error rate.
No background in classical statistics required to understand the concept.
