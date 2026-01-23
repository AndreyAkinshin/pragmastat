#import "/manual/definitions.typ": *

== Disparity ('robust effect size')

$ Disparity(vx, vy) = Shift(vx, vy) / AvgSpread(vx, vy) $

- Measures a normalized $Shift$ between $vx$ and $vy$ expressed in spread units
- Expresses the 'effect size', renamed to $Disparity$ for clarity
- Pragmatic alternative to Cohen's d (note: exact estimates differ due to robust construction)
- Domain: $AvgSpread(vx, vy) > 0$
- Unit: spread unit

$ Disparity(vx + k, vy + k) = Disparity(vx, vy) $

$ Disparity(k dot vx, k dot vy) = op("sign")(k) dot Disparity(vx, vy) $

$ Disparity(vx, vy) = -Disparity(vy, vx) $
