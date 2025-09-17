## Breakdown

Heavy-tailed distributions naturally produce extreme outliers that completely distort traditional estimators.
A single extreme measurement from a power-law distribution can make the sample mean arbitrarily large.
Real-world data also contains corrupted measurements from instrument failures, recording errors, or transmission problems.
Both natural extremes and data corruption create the same challenge:
  how to extract reliable information when some measurements are untrustworthy.

Breakdown point measures an estimator's resistance to such contamination.
An estimator with breakdown point $\epsilon$ can withstand up to $100\epsilon\%$ extreme values
  before producing meaningless results, regardless of how extreme those values become.
The theoretical maximum is 50% — no estimator can guarantee reliable results
  when more than half the measurements are extreme or corrupted.
Beyond this threshold, the contaminated values dominate and the original signal becomes unrecoverable.

Pursuing maximum robustness unnecessarily sacrifices precision.
When nearly half the data consists of extreme values,
  the distribution likely exhibits multiple modes rather than a single distribution with outliers.
Such cases require different analysis techniques, not just robust estimators.
Maximum robustness also discards useful information from the sample,
  reducing precision even under normal conditions.
The practical goal balances adequate protection against precision loss.

The $\Center$ and $\Spread$ estimators achieve 29% breakdown points,
  providing substantial protection against realistic contamination levels
  while maintaining good statistical efficiency.
This represents an effective engineering choice —
  sufficient robustness for typical outlier scenarios without excessive precision sacrifice.

**Asymptotic breakdown points** for average estimators:

| $\Mean$ | $\Median$ | $\Center$ |
|---------|-----------|-----------|
| 0%      | 50%       | 29%       |

The $\Median$ achieves maximum robustness — corrupting less than half the data
  cannot move the middle value arbitrarily.
The $\Center$ inherits robustness from its median-based construction
  while incorporating more sample information than the median alone.

**Asymptotic breakdown points** for dispersion estimators:

| $\StdDev$ | $\MAD$ | $\Spread$ |
|-----------|--------|-----------|
| 0%        | 50%    | 29%       |

Higher breakdown points provide insurance against extreme values.
For data from well-behaved distributions, traditional estimators offer optimal precision.
For heavy-tailed distributions or when data quality is uncertain,
  robust estimators maintain reliability despite extreme outliers.
