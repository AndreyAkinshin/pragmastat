#import "/manual/definitions.typ": *

== AvgSpread

$ AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) / (n + m) $

- Measures average dispersion across two samples
- Pragmatic alternative to the 'pooled standard deviation'
- Note: $AvgSpread(vx, vy) != Spread(vx union vy)$ in general (defines a pooled scale, not the spread of the concatenated sample)
- Domain: any real numbers
- Unit: the same as measurements

$ AvgSpread(vx, vx) = Spread(vx) $

$ AvgSpread(k_1 dot vx, k_2 dot vx) = (abs(k_1) + abs(k_2)) / 2 dot Spread(vx) $

$ AvgSpread(vx, vy) = AvgSpread(vy, vx) $

$ AvgSpread(k dot vx, k dot vy) = abs(k) dot AvgSpread(vx, vy) $
