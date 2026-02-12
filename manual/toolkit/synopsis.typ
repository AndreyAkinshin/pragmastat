#import "/manual/definitions.typ": *

== Synopsis

*One-Sample Estimators*

#list(marker: none, tight: true,
  [$Center(vx)$ — robust location; median of pairwise averages. \
  #h(2em) Like the mean but stable with outliers; tolerates up to 29% corrupted data.],
  [$CenterBounds(vx, misrate)$ — bounds on center with error rate $= misrate$. \
  #h(2em) Exact under weak symmetry.],
  [$Spread(vx)$ — robust dispersion; median of pairwise absolute differences. \
  #h(2em) Same units as data; tolerates up to 29% corrupted data.],
  [$RelSpread(vx)$ — relative dispersion $= Spread \/ abs(Center)$. \
  #h(2em) Dimensionless; compares variability across scales.],
)

#v(0.5em)
*Two-Sample Estimators*

#list(marker: none, tight: true,
  [$Shift(vx, vy)$ — robust location difference; median of pairwise differences. \
  #h(2em) Negative means first sample tends to be lower.],
  [$ShiftBounds(vx, vy, misrate)$ — bounds on shift with error rate $= misrate$. \
  #h(2em) If bounds exclude zero, the difference is reliable.],
  [$Ratio(vx, vy)$ — robust multiplicative ratio via log-space shift. \
  #h(2em) For positive-valued quantities (latency, price, concentration).],
  [$RatioBounds(vx, vy, misrate)$ — bounds on ratio with error rate $= misrate$. \
  #h(2em) If bounds exclude 1, the multiplicative difference is reliable.],
  [$AvgSpread(vx, vy)$ — pooled robust dispersion, weighted by sample sizes.],
  [$Disparity(vx, vy)$ — robust effect size $= Shift \/ AvgSpread$ (robust Cohen's d).],
)

#v(0.5em)
*Randomization*

#list(marker: none, tight: true,
  [$Rng(s)$ — deterministic pseudorandom generator from seed $s$. \
  #h(2em) Identical sequences across all supported languages.],
  [$r.Sample(vx, k)$ — select $k$ elements without replacement.],
  [$r.Shuffle(vx)$ — uniformly random permutation.],
  [$r.Resample(vx, k)$ — select $k$ elements with replacement.],
)
