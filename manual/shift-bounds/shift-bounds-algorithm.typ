#import "/manual/definitions.typ": *

The $ShiftBounds$ estimator constructs distribution-free bounds on $Shift(vx, vy)$
  by selecting specific order statistics from the pairwise differences.

Given samples $vx = (x_1, ..., x_n)$ and $vy = (y_1, ..., y_m)$, the algorithm proceeds as follows:

+ *Compute the margin* ---
  Call $PairwiseMargin(n, m, misrate)$ (see #link(<sec-alg-pairwise-margin>)[PairwiseMargin])
  to determine how many extreme pairwise differences to exclude from each tail.

+ *Determine quantile ranks* ---
  From the margin $M$, compute $k_"left" = floor(M / 2) + 1$ and $k_"right" = n m - floor(M / 2)$.
  These are the ranks of the order statistics that form the bounds.

+ *Compute quantiles via Shift* ---
  Use the #link(<sec-alg-shift>)[Shift] algorithm to compute
  the $k_"left"$-th and $k_"right"$-th order statistics of all $n m$ pairwise differences $x_i - y_j$.
  The Shift algorithm's value-space binary search finds these quantiles
  in $O((n + m) log L)$ time without materializing all differences.

+ *Return bounds* ---
  Return $[z_((k_"left")), z_((k_"right"))]$.

The $PairwiseMargin$ function encodes the statistical theory:
  it determines which order statistics provide bounds with coverage $1 - misrate$.
The $Shift$ algorithm provides the computational machinery:
  it extracts those specific order statistics efficiently from the implicit matrix of pairwise differences.
