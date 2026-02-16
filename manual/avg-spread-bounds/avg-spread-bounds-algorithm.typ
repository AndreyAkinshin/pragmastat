#import "/manual/definitions.typ": *

The $AvgSpreadBounds$ estimator constructs bounds on the pooled spread
  by combining two independent $SpreadBounds$ calls through a Bonferroni split.

The algorithm proceeds as follows:

+ *Equal Bonferroni split* ---
  Set $alpha = misrate / 2$.
  Each per-sample bounds call uses half the total error budget.

+ *Per-sample bounds* ---
  Compute $[L_x, U_x] = SpreadBounds(vx, alpha)$ and
  $[L_y, U_y] = SpreadBounds(vy, alpha)$
  (see #link(<sec-alg-spread-bounds>)[SpreadBounds]).

+ *Weighted linear combination* ---
  With weights $w_x = n / (n + m)$ and $w_y = m / (n + m)$, return:
  $ [L_A, U_A] = [w_x L_x + w_y L_y, w_x U_x + w_y U_y] $

By Bonferroni's inequality, the probability that both per-sample bounds simultaneously cover
  their respective true spreads is at least $1 - 2 alpha = 1 - misrate$.
Since $AvgSpread$ is a weighted average of the individual spreads,
  the linear combination of the bounds covers the true $AvgSpread$ whenever both individual bounds hold.

#source-include("cs/Pragmastat/Estimators/AvgSpreadBoundsEstimator.cs", "cs")
