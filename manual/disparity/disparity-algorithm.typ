#import "/manual/definitions.typ": *

The $Disparity$ estimator is a composition of #link(<sec-alg-shift>)[Shift] and #link(<sec-alg-spread>)[Spread]:

$ Disparity(vx, vy) = Shift(vx, vy) / AvgSpread(vx, vy) $

where $AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m)$ is the pooled scale.

The algorithm proceeds as follows:

+ *Compute Spread for each sample* ---
  Delegate to the Spread algorithm for $vx$ and $vy$ independently.

+ *Compute AvgSpread* ---
  Form the weighted average $AvgSpread = (n dot Spread(vx) + m dot Spread(vy)) / (n + m)$.

+ *Domain check* ---
  Verify that $AvgSpread > 0$.
  If the pooled spread is zero, the division is undefined.

+ *Compute Shift* ---
  Delegate to the Shift algorithm for the pair $(vx, vy)$.

+ *Divide* ---
  Return $Shift(vx, vy) / AvgSpread(vx, vy)$.

#source-include("cs/Pragmastat/Estimators/DisparityEstimator.cs", "cs")
