#import "/manual/definitions.typ": *

== Synopsis

*One-Sample Estimators*

#list(marker: none, tight: true,
  [#link(<sec-center>)[$Center(vx)$] — robust average],
  [#link(<sec-center-bounds>)[$CenterBounds(vx, misrate)$] — confidence interval for center],
  [#link(<sec-spread>)[$Spread(vx)$] — robust spread],
  [#link(<sec-spread-bounds>)[$SpreadBounds(vx, misrate)$] — confidence interval for spread],
)

#v(0.5em)
*Two-Sample Estimators*

#list(marker: none, tight: true,
  [#link(<sec-shift>)[$Shift(vx, vy)$] — robust difference],
  [#link(<sec-shift-bounds>)[$ShiftBounds(vx, vy, misrate)$] — confidence interval for shift],
  [#link(<sec-ratio>)[$Ratio(vx, vy)$] — robust ratio],
  [#link(<sec-ratio-bounds>)[$RatioBounds(vx, vy, misrate)$] — confidence interval for ratio],
  [#link(<sec-disparity>)[$Disparity(vx, vy)$] — robust effect size $= Shift"/"AvgSpread$],
  [#link(<sec-disparity-bounds>)[$DisparityBounds(vx, vy, misrate)$] — confidence interval for disparity],
)

#v(0.5em)
*Randomization*

#list(marker: none, tight: true,
  [#link(<sec-rng>)[$r <- Rng(s)$] — random number generator with seed $s$],
  [#link(<sec-uniform-float>)[$r.UniformFloat()$] — uniform random value in $[0, 1)$],
  [#link(<sec-uniform-int>)[$r.UniformInt(a, b)$] — uniform random integer in $[a, b)$],
  [#link(<sec-sample>)[$r.Sample(vx, k)$] — select $k$ elements without replacement],
  [#link(<sec-resample>)[$r.Resample(vx, k)$] — select $k$ elements with replacement],
  [#link(<sec-shuffle>)[$r.Shuffle(vx)$] — uniformly random permutation],
)

#pagebreak()
The table below maps each toolkit function to the underlying algorithm and its complexity.

#v(0.5em)
*One-Sample Estimators*

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Function*], [*Algorithm*], [*Complexity*],
  table.hline(),
  [$Center$], [Monahan's implicit-matrix selection], [$O(n log n)$],
  [$CenterBounds$], [Binary search over pairwise averages + SignedRankMargin], [$O(n log n)$],
  [$Spread$], [Monahan's selection adapted for differences], [$O(n log n)$],
  [$SpreadBounds$], [Disjoint-pair sign-test inversion], [$O(n log n)$],
  table.hline(),
)

#v(0.5em)
*Two-Sample Estimators*

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Function*], [*Algorithm*], [*Complexity*],
  table.hline(),
  [$Shift$], [Value-space binary search over pairwise differences], [$O((n+m) log L)$],
  [$ShiftBounds$], [PairwiseMargin + Shift quantile selection], [$O((n+m) log L)$],
  [$Ratio$], [Log-exp transform + Shift], [$O((n+m) log L)$],
  [$RatioBounds$], [Log-exp transform + ShiftBounds], [$O((n+m) log L)$],
  [$Disparity$], [Composition: $Shift / AvgSpread$], [$O((n+m) log L + n log n + m log m)$],
  [$DisparityBounds$], [Bonferroni split: ShiftBounds + AvgSpreadBounds], [$O((n+m) log L + n log n + m log m)$],
  table.hline(),
)

#v(0.5em)
*Randomization*

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Function*], [*Algorithm*], [*Complexity*],
  table.hline(),
  [$UniformFloat$], [53-bit extraction from xoshiro256++ output], [$O(1)$ per draw],
  [$UniformInt$], [Modulo reduction of raw 64-bit output], [$O(1)$ per draw],
  [$Sample$], [Fan-Muller-Rezucha selection sampling], [$O(n)$],
  [$Resample$], [Uniform integer sampling with replacement], [$O(k)$],
  [$Shuffle$], [Fisher-Yates (Knuth shuffle)], [$O(n)$],
  table.hline(),
)

#v(0.5em)
*Auxiliary*

#table(
  columns: 3,
  align: (left, left, left),
  stroke: none,
  table.hline(),
  [*Function*], [*Algorithm*], [*Complexity*],
  table.hline(),
  [$AvgSpread$], [Weighted average of two Spread calls], [$O(n log n + m log m)$],
  [$AvgSpreadBounds$], [Bonferroni combination of two SpreadBounds], [$O(n log n + m log m)$],
  [$Median$], [Sort + pick middle], [$O(n log n)$],
  [$SignMargin$], [Binomial CDF inversion + randomized cutoff], [$O(n)$],
  [$PairwiseMargin$], [Löffler recurrence (exact) / Edgeworth (approx)], [$O(n m)$ / $O(log(n m))$],
  [$SignedRankMargin$], [Dynamic programming (exact) / Edgeworth (approx)], [$O(n^3)$ / $O(log n)$],
  table.hline(),
)
