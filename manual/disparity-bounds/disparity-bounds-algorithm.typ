#import "/manual/definitions.typ": *

The $DisparityBounds$ estimator constructs bounds on $Disparity(vx, vy)$
  by combining $ShiftBounds$ and $AvgSpreadBounds$ through a Bonferroni split.

*Misrate allocation*

The total $misrate$ budget is split between the shift and avg-spread components.
Let $min_S = 2 / binom(n + m, n)$ (minimum for $ShiftBounds$) and
$min_A = 2 dot max(2^(1-floor(n slash 2)), 2^(1-floor(m slash 2)))$ (minimum for $AvgSpreadBounds$).
The extra budget beyond the minimums is split equally:

$ alpha_S = min_S + (misrate - min_S - min_A) / 2, quad
  alpha_A = min_A + (misrate - min_S - min_A) / 2 $

*Component bounds*

Compute $[L_S, U_S] = ShiftBounds(vx, vy, alpha_S)$ and
$[L_A, U_A] = AvgSpreadBounds(vx, vy, alpha_A)$.
By Bonferroni's inequality, the probability that both intervals simultaneously contain
  their respective true values is at least $1 - alpha_S - alpha_A = 1 - misrate$.

*Interval division*

When $L_A > 0$, the disparity bounds are obtained by dividing the shift interval by the avg-spread interval.
Since dividing by a positive interval can flip the ordering depending on the sign of the numerator endpoints,
  the algorithm computes all four combinations and takes the extremes:

$ [L_D, U_D] = [min(L_S / L_A, L_S / U_A, U_S / L_A, U_S / U_A), max(L_S / L_A, L_S / U_A, U_S / L_A, U_S / U_A)] $

*Edge cases*

When $L_A = 0$ (the avg-spread interval includes zero), the bounds become partially or fully unbounded
  depending on the sign of $[L_S, U_S]$:

- $L_S > 0$: $[L_S / U_A, +infinity)$
- $U_S < 0$: $(-infinity, U_S / U_A]$
- $L_S = U_S = 0$: $[0, 0]$
- otherwise: $(-infinity, +infinity)$

When $U_A = 0$ (the avg-spread interval collapses to zero), only the sign of the shift determines the result.

#source-include("cs/Pragmastat/Estimators/DisparityBoundsEstimator.cs", "cs")
