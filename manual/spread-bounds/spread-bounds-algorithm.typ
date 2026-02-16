#import "/manual/definitions.typ": *

The $SpreadBounds$ estimator constructs distribution-free bounds on $Spread(vx)$
  by inverting a sign test on disjoint pairs.

Given a sample $vx = (x_1, ..., x_n)$, the algorithm proceeds as follows:

+ *Random disjoint pairing* ---
  Randomly pair the $n$ observations into $m = floor(n / 2)$ disjoint pairs.
  If $n$ is odd, one observation is discarded.
  The randomization ensures that the pairing does not depend on the data ordering.

+ *Absolute differences* ---
  For each pair $(x_(a_i), x_(b_i))$, compute the absolute difference $d_i = abs(x_(a_i) - x_(b_i))$.
  Under the sparity assumption, these $m$ absolute differences are exchangeable.

+ *Sort* ---
  Sort the differences to obtain $d_((1)) <= d_((2)) <= ... <= d_((m))$.

+ *SignMargin cutoff* ---
  Compute $r = SignMargin(m, misrate)$ (see #link(<sec-alg-sign-margin>)[SignMargin]).
  This determines how many extreme order statistics to exclude from each tail.

+ *Order statistic selection* ---
  Return $[d_((k_L)), d_((k_U))]$ where $k_L = floor(r / 2) + 1$ and $k_U = m - floor(r / 2)$.
  Clamping ensures the indices remain within $[1, m]$.

The key insight is that disjoint pairs provide independence under the symmetry assumption.
Under weak symmetry around the true spread, each absolute difference is equally likely
  to exceed or fall below the population spread.
This makes the count of differences exceeding the spread a $"Binomial"(m, 1\/2)$ variable,
  enabling exact coverage control via the sign test.

The randomized cutoff from $SignMargin$ matches the requested misrate exactly under weak continuity.
With tied values, the bounds become conservative (actual coverage exceeds $1 - misrate$).

#source-include("cs/Pragmastat/Estimators/SpreadBoundsEstimator.cs", "cs")
