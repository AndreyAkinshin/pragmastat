#import "/manual/definitions.typ": *

The $AvgSpread$ function computes the weighted average of per-sample spreads:

$ AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m) $

The algorithm delegates to the #link(<sec-alg-spread>)[Spread] algorithm independently for each sample,
  then forms the weighted linear combination with weights $n / (n + m)$ and $m / (n + m)$.

#source-include("cs/Pragmastat/Estimators/AvgSpreadEstimator.cs", "cs")
