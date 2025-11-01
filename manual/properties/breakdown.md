## Breakdown

Heavy-tailed distributions naturally produce extreme outliers that completely distort traditional estimators.
A single extreme measurement from the $\Power$ distribution can make the sample mean arbitrarily large.
Real-world data also contains corrupted measurements from instrument failures, recording errors, or transmission problems.
Both natural extremes and data corruption create the same challenge:
  how to extract reliable information when some measurements are too influential.

The breakdown point is the fraction of the sample that can be replaced by arbitrarily large values
  without making the estimator arbitrarily large.
The theoretical maximum is $50\%$ — no estimator can guarantee reliable results
  when more than half the measurements are extreme or corrupted.
In such cases, summary estimators are not applicable and a more sophisticated approach is needed.

Even $50\%$ is rarely needed in practice; more conservative breakdown points also cover practical needs.
Additionally, when the breakdown point is high, the precision is low
  (we lose information by neglecting part of the data).
The optimal practical breakdown point should be somewhere between
  $0\%$ (no robustness) and $50\%$ (low precision).

The $\Center$ and $\Spread$ estimators achieve $29\%$ breakdown points,
  providing substantial protection against realistic contamination levels
  while maintaining good precision.
Below is a comparison with traditional estimators.

**Asymptotic breakdown points** for average estimators:

| $\Mean$ | $\Median$ | $\Center$ |
|---------|-----------|-----------|
| 0%      | 50%       | 29%       |

**Asymptotic breakdown points** for dispersion estimators:

| $\StdDev$ | $\MAD$ | $\Spread$ |
|-----------|--------|-----------|
| 0%        | 50%    | 29%       |
